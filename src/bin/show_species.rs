use diesel_course::{connect_to_db, models::Species, queries::species_queries::get_all_species};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");

    let db = &mut connect_to_db(&database_url).unwrap();
    let species = get_all_species(db).unwrap();

    for Species { id, name } in species {
        println!("{id} - {name}");
    }
}
