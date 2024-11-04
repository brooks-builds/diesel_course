use crate::schema::pets;
use crate::schema::species;
use chrono::NaiveDateTime;
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

#[derive(Debug, Identifiable, Queryable, Selectable)]
#[diesel(table_name = species)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Species {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Species))]
pub struct Pet {
    pub id: i32,
    pub name: String,
    pub species_id: i32,
    pub last_fed: Option<NaiveDateTime>,
}
