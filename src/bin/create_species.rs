use diesel_course::{connect_to_db, queries::species_queries::create_species};
use std::io::stdin;

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");
    let db = &mut connect_to_db(&database_url).expect("Error connecting to database");

    println!("What is the name of the species you would like to create?");
    let mut new_name = String::new();
    stdin()
        .read_line(&mut new_name)
        .expect("Error getting new species name from user");
    let new_name = new_name.trim().to_owned();

    let species_id = create_species(db, &new_name).expect("Error creating species");

    println!("Create species {new_name} with id {species_id}");
}
