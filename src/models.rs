use crate::schema::pets;
use crate::schema::species;
use diesel::prelude::*;

#[derive(Debug, Insertable)]
#[diesel(table_name = species)]
pub struct CreateSpecies {
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = pets)]
pub struct CreatePet {
    pub name: String,
    pub species_id: i32,
}
