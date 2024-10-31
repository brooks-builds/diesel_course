use diesel::{Connection, Insertable, PgConnection, RunQueryDsl};
use diesel_course::models::{CreatePet, CreateSpecies};
use diesel_course::schema::pets as pets_schema;
use diesel_course::schema::species as species_schema;

const SPECIES_TO_CREATE: [&str; 3] = ["cat", "squirrel", "dog"];
const PETS_TO_CREATE: [(&str, usize); 4] =
    [("Xilbe", 0), ("Ranger", 2), ("bracket", 1), ("Phat Gus", 1)];

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");
    let db_connection = &mut PgConnection::establish(&database_url)
        .expect("Error connecting to the postgres database");

    let created_species = SPECIES_TO_CREATE
        .iter()
        .map(|species_name| {
            let species = CreateSpecies {
                name: species_name.to_string(),
            };
            species
                .insert_into(species_schema::table)
                .returning(species_schema::id)
                .get_result(db_connection)
                .unwrap()
        })
        .collect::<Vec<i32>>();

    PETS_TO_CREATE.iter().for_each(|(pet_name, species_index)| {
        let pet = CreatePet {
            name: pet_name.to_string(),
            species_id: created_species[*species_index],
        };

        pet.insert_into(pets_schema::table)
            .execute(db_connection)
            .unwrap();
    });
}
