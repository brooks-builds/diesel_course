use std::{env, io::stdin};

use diesel_course::{
    connect_to_db,
    queries::{pets_queries::create_pet, species_queries::get_all_species},
};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Error, missing DATABASE_URL environment variable");
    let db = &mut connect_to_db(&database_url).expect("Error connecting to database");

    println!("What is the name of the pet you want to create?");
    let mut new_pet_name = String::new();
    stdin()
        .read_line(&mut new_pet_name)
        .expect("Error reading pet name from user");
    let new_pet_name = new_pet_name.trim().to_owned();

    println!("What is the species id of the pet you want to create?");
    println!("Possible species to choose from:");
    for species in get_all_species(db).expect("error getting all species") {
        println!("{} - {}", species.id, species.name);
    }
    let mut new_species_id = String::new();
    stdin()
        .read_line(&mut new_species_id)
        .expect("Error getting species id from user");
    let new_species_id = match new_species_id.trim().parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error, the species id must be a valid number");
            return;
        }
    };

    let created_pet_id =
        create_pet(db, &new_pet_name, new_species_id).expect("Error creating new pet");

    println!(
        "Pet {new_pet_name} has been created with an id of {:?}",
        created_pet_id
    );
}
