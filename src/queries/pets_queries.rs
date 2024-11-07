use diesel::{query_dsl::methods::SelectDsl, PgConnection, RunQueryDsl, SelectableHelper};
use eyre::{Context, Result};

use crate::models::Pet;

pub fn get_all_pets(db: &mut PgConnection) -> Result<Vec<Pet>> {
    use crate::schema::pets::dsl::pets;

    pets.select(Pet::as_select())
        .load(db)
        .context("Getting all pets")
}
