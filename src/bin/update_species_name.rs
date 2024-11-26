use diesel_course::{
    connect_to_db,
    queries::species_queries::{get_all_species, update_species_name},
};
use eyre::{Context, Result};
use std::{env, io::stdin};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Error, missing DATABASE_URL environment variable");
    let db = &mut connect_to_db(&database_url).expect("Error connecting to database");

    println!("which species name would you like to change?");
    for species in get_all_species(db).expect("error getting all species") {
        println!("{} - {}", species.id, species.name);
    }

    let id = match get_user_input("Enter the species id you would like to change the name of")
        .expect("Error getting species id as a string")
        .parse()
    {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error, the id must be a number");
            return;
        }
    };

    let new_name = get_user_input("What is the new name?").expect("Error getting new species name");

    update_species_name(db, &new_name, id).expect("Error updating species name");

    println!("Species with id {id} now has name {new_name}");
}

fn get_user_input(message: &str) -> Result<String> {
    println!("{message}");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .context("getting input from user")?;
    Ok(input.trim().to_owned())
}
