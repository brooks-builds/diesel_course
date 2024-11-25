use crate::{models::Pet, schema::pets};
use diesel::prelude::*;
use diesel::{debug_query, pg::Pg, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use eyre::{Context, Result};

pub fn get_all_pets(db: &mut PgConnection) -> Result<Vec<Pet>> {
    use crate::schema::pets::dsl::pets;

    let query = pets.select(Pet::as_select());

    println!("get all pets query: {}", debug_query::<Pg, _>(&query));

    query.load(db).context("Getting all pets")
}

pub fn get_pet_by_id(db: &mut PgConnection, id: i32) -> Result<Option<Pet>> {
    pets::table
        .find(id)
        .first(db)
        .optional()
        .context("getting pet by id")
}
