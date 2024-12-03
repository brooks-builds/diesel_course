use diesel_course::{connect_to_db, models::Pet, queries::pets_queries::get_all_pets};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("missing environment variable DATABASE_URL");
    let db = &mut connect_to_db(&database_url).unwrap();
    let all_pets = get_all_pets(db).unwrap();

    for Pet {
        id,
        name,
        species_id,
        last_fed,
        ..
    } in all_pets
    {
        println!("{id} - {name} ({species_id}): last fed: {last_fed:?}");
    }
}
