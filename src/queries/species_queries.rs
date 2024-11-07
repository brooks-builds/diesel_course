use crate::models::Species;
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
