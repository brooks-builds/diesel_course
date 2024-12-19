use std::io::stdin;

use diesel_course::{
    connect_to_db,
    queries::pets_queries::{get_all_pets, soft_delete_pet},
};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
    let db = &mut connect_to_db(&database_url).expect("Error connecting to database");

    for (pet, _species) in get_all_pets(db).expect("Error getting all pets") {
        println!("{} - {}", pet.name, pet.id);
    }
    println!("What pet id should be deleted");

    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Error getting pet id from user");
    let pet_id = user_input
        .trim()
        .parse()
        .expect("Error converting user input into number");

    soft_delete_pet(db, pet_id).expect("Error deleting pet");

    println!("Pet has been deleted");
}
