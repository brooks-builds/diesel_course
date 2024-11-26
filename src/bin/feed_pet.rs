use chrono::Utc;
use diesel_course::{
    connect_to_db,
    queries::pets_queries::{get_all_pets, update_pet_last_fed},
};
use std::{env, io::stdin};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Error, missing DATABASE_URL environment variable");
    let db = &mut connect_to_db(&database_url).expect("Error connecting to database");

    println!("which pet would you like to feed?");
    for pet in get_all_pets(db).expect("Error getting all pets") {
        println!("{} - {}", pet.id, pet.name);
    }

    let mut pet_id = String::new();
    stdin()
        .read_line(&mut pet_id)
        .expect("getting pet id from user");
    let pet_id = pet_id
        .trim()
        .parse()
        .expect("Error, pet id must be a number");

    let now = Utc::now().naive_utc();

    update_pet_last_fed(db, now, pet_id).expect("Error updating last time pet was fed");

    println!("Pet has been fed");
}
