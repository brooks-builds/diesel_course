use crate::models::CreateSpecies;
use crate::{models::Species, schema::species};
use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use eyre::{Context, Result};

pub fn get_all_species(db: &mut PgConnection) -> Result<Vec<Species>> {
    use crate::schema::species::dsl::species;

    species
        .select(Species::as_select())
        // .select((id, name))
        // .select(name)
        .load(db)
        .context("getting all species")
}

pub fn get_species_by_id(db: &mut PgConnection, id: i32) -> Result<Option<Species>> {
    species::table
        .find(id)
        .first(db)
        .optional()
        .context("Loading species by id")
}

pub fn create_species(db: &mut PgConnection, name: &str) -> Result<i32> {
    let species = CreateSpecies {
        name: name.to_owned(),
    };

    species
        .insert_into(species::table)
        .returning(species::dsl::id)
        .get_result(db)
        .context("Inserting species into the database")
}

pub fn update_species_name(db: &mut PgConnection, name: &str, id: i32) -> Result<()> {
    diesel::update(species::table.filter(species::dsl::id.eq(id)))
        .set(species::dsl::name.eq(name))
        .execute(db)
        .context("updating species name")?;

    Ok(())
}
