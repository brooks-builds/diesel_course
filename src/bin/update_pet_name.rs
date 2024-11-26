use std::{env, io::stdin};

use diesel_course::{
    connect_to_db,
    queries::pets_queries::{get_all_pets, update_pet_name},
};
use eyre::{Context, Result};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Error, missing DATABASE_URL environment variable");
    let db = &mut connect_to_db(&database_url).expect("Error connecting to database");

    println!("which pet name would you like to change?");
    for pet in get_all_pets(db).expect("error getting all pets") {
        println!("{} - {}", pet.id, pet.name);
    }

    let id = match get_user_input("Enter the pet id you would like to change the name of")
        .expect("Error getting pet id as a string")
        .parse()
    {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error, the id must be a number");
            return;
        }
    };

    let new_name = get_user_input("What is the new name?").expect("Error getting new pet name");

    update_pet_name(db, &new_name, id).expect("Error updating pet name");

    println!("Pet with id {id} now has name {new_name}");
}

fn get_user_input(message: &str) -> Result<String> {
    println!("{message}");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .context("getting input from user")?;
    Ok(input.trim().to_owned())
}
