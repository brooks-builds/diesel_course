use crate::models::CreatePet;
use crate::{models::Pet, schema::pets};
use chrono::{NaiveDateTime, Utc};
use diesel::associations::HasTable;
use diesel::pg::Pg;
use diesel::{debug_query, prelude::*};
use eyre::{Context, Result};

pub fn get_all_pets(db: &mut PgConnection) -> Result<Vec<Pet>> {
    use crate::schema::pets::dsl::{deleted_at, pets};

    pets.select(Pet::as_select())
        .filter(deleted_at.is_null())
        .load(db)
        .context("Getting all pets")
}

pub fn get_pet_by_id(db: &mut PgConnection, id: i32) -> Result<Option<Pet>> {
    let pet = pets::table.find(id);

    println!("**pet query:**\n{}\n", debug_query::<Pg, _>(&pet));

    pet.first(db).optional().context("getting pet by id")
}

pub fn create_pet(db: &mut PgConnection, name: &str, species_id: i32) -> Result<(i32, String)> {
    let new_pet = CreatePet {
        name: name.to_owned(),
        species_id,
    };

    new_pet
        .insert_into(pets::table)
        .returning((pets::dsl::id, pets::dsl::name))
        // .returning(Pet::as_select())
        // .returning(pets::dsl::id)
        .get_result(db)
        .context("inserting new pet")
}

pub fn batch_create_pets(db: &mut PgConnection, pets: &[CreatePet]) -> Result<Vec<Pet>> {
    diesel::insert_into(Pet::table())
        .values(pets)
        .get_results(db)
        .context("batch inserting pets")
}

pub fn update_pet_last_fed(
    db: &mut PgConnection,
    last_fed: NaiveDateTime,
    pet_id: i32,
) -> Result<()> {
    diesel::update(pets::table.filter(pets::dsl::id.eq(pet_id)))
        .set(pets::dsl::last_fed.eq(Some(last_fed)))
        .execute(db)
        .context("updating last fed for pet")?;

    Ok(())
}

pub fn update_pet_name(db: &mut PgConnection, name: &str, id: i32) -> Result<()> {
    diesel::update(pets::table.filter(pets::dsl::id.eq(id)))
        .set(pets::dsl::name.eq(name))
        .execute(db)
        .context("updating pet name")?;

    Ok(())
}

pub fn hard_delete_pet(db: &mut PgConnection, id: i32) -> Result<()> {
    diesel::delete(pets::table.filter(pets::dsl::id.eq(id)))
        .execute(db)
        .context("hard deleting pet")?;

    Ok(())
}

pub fn soft_delete_pet(db: &mut PgConnection, id: i32) -> Result<()> {
    let now = Utc::now().naive_utc();

    diesel::update(pets::table.filter(pets::dsl::id.eq(id)))
        .set(pets::dsl::deleted_at.eq(now))
        .execute(db)
        .context("soft deleting pet")?;

    Ok(())
}

pub fn soft_delete_pets_by_species(db: &mut PgConnection, id: i32) -> Result<()> {
    let now = Utc::now().naive_utc();

    diesel::update(pets::table.filter(pets::dsl::species_id.eq(id)))
        .set(pets::dsl::deleted_at.eq(now))
        .execute(db)
        .context("soft deleting all pets with given species id")?;

    Ok(())
}
