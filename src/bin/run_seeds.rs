use diesel::{Connection, PgConnection};
use diesel_course::models::{CreatePet, CreateSpecies};
use diesel_course::queries::pets_queries::batch_create_pets;
use diesel_course::queries::species_queries::batch_create_species;

const SPECIES_TO_CREATE: [&str; 3] = ["cat", "squirrel", "dog"];
const PETS_TO_CREATE: [(&str, usize); 4] =
    [("Xilbe", 0), ("Ranger", 2), ("bracket", 1), ("Phat Gus", 1)];

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");
    let db_connection = &mut PgConnection::establish(&database_url)
        .expect("Error connecting to the postgres database");

    let species = SPECIES_TO_CREATE
        .iter()
        .map(|species_name| CreateSpecies {
            name: species_name.to_string(),
        })
        .collect::<Vec<CreateSpecies>>();

    let created_species =
        batch_create_species(db_connection, &species).expect("Inserting species for seeds");

    let pets = PETS_TO_CREATE
        .iter()
        .map(|(pet_name, species_index)| CreatePet {
            name: pet_name.to_string(),
            species_id: created_species[*species_index].id,
        })
        .collect::<Vec<CreatePet>>();

    batch_create_pets(db_connection, &pets).expect("Error seeding pets");
}
