use diesel_course::{
    connect_to_db,
    models::{Pet, Species},
    queries::pets_queries::get_all_pets,
};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("missing environment variable DATABASE_URL");
    let db = &mut connect_to_db(&database_url).unwrap();
    let all_pets = get_all_pets(db).unwrap();

    for (
        Pet {
            id, name, last_fed, ..
        },
        Species {
            name: species_name, ..
        },
    ) in all_pets
    {
        println!("{id} - {name} ({species_name}): last fed: {last_fed:?}");
    }
}
