use std::io::stdin;

use diesel_course::{
    connect_to_db,
    models::Species,
    queries::{
        pets_queries::soft_delete_pets_by_species,
        species_queries::{get_all_species, soft_delete_species},
    },
};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("Error missing DATABASE_URL environment variable");
    let db = &mut connect_to_db(&database_url).expect("Error connecting to database");

    for Species { id, name, .. } in get_all_species(db).expect("Error getting all species") {
        println!("{name} - {id}");
    }
    println!("Enter the id of the species you would like to delete");

    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Error getting species id from user");
    let species_id = user_input
        .trim()
        .parse()
        .expect("Error, id must be a number");

    soft_delete_species(db, species_id).expect("Error deleting species");

    println!("Deleted species");

    soft_delete_pets_by_species(db, species_id)
        .expect("Error soft deleting pets for deleted species");

    println!("Removed pets for the removed species");
}
