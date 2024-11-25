use diesel_course::{connect_to_db, queries::species_queries::get_species_by_id};
use std::{env, io::stdin};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Error, missing DATABASE_URL environment variable");
    let db = &mut connect_to_db(&database_url).expect("Error connecting to database");

    println!("What Species would you like to see? Please enter the species id");

    let mut input_id = String::new();

    stdin()
        .read_line(&mut input_id)
        .expect("Error reading input from user");

    let species_id = match input_id.trim().parse::<i32>() {
        Ok(id) => id,
        Err(_error) => {
            eprintln!("Species id's must be a number, please try again with a number for the id");
            return;
        }
    };
    let species = match get_species_by_id(db, species_id) {
        Ok(Some(species)) => species,
        Ok(None) => {
            println!("No species with that id exists in the database");
            return;
        }
        Err(error) => {
            eprintln!("There was an error getting the species by id from the database: {error:?}");
            return;
        }
    };

    println!("{} - {}", species.id, species.name);
}
