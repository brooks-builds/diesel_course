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
