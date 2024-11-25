use std::{env, io::stdin};

use diesel_course::{connect_to_db, queries::pets_queries::get_pet_by_id};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Error, missing DATABASE_URL envioronment varialbe");
    let db = &mut connect_to_db(&database_url).expect("Error connecting to database");
    let mut input_id = String::new();

    println!("What Pet would you like to see? Enter the pet's id: ");

    if let Err(error) = stdin().read_line(&mut input_id) {
        eprintln!("There was an error reading the id: {error:?}");
        return;
    }

    let pet_id = match input_id.trim().parse::<i32>() {
        Ok(id) => id,
        Err(_error) => {
            eprintln!("Error, pet ids must be a number");
            return;
        }
    };

    let pet = match get_pet_by_id(db, pet_id) {
        Ok(Some(pet)) => pet,
        Ok(None) => {
            println!("No pet matching that id was found");
            return;
        }
        Err(error) => {
            eprintln!("Error getting pet from the database: {error:?}");
            return;
        }
    };

    println!(
        "{} - {} ({}): last fed: {:?}",
        pet.id, pet.name, pet.species_id, pet.last_fed
    );
}
