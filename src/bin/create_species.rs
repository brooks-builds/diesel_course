use diesel::{associations::HasTable, Connection, Insertable, PgConnection, RunQueryDsl};
use diesel_course::{
    models::{CreateSpecies, Species},
    schema,
};
use eyre::{Context, Result};

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");
    let db_connection =
        &mut PgConnection::establish(&database_url).expect("Error connecting to the database");

    let new_name = get_new_species_name().expect("Error getting new species name from user");
    let new_species = CreateSpecies { name: new_name };

    let created_species = new_species
        .insert_into(Species::table())
        .returning(schema::species::id)
        // .returning((schema::species::id, schema::species::name))
        .get_result::<i32>(db_connection)
        .expect("inserting new species into the database");

    println!("Created new species with id {created_species}");
}

fn get_new_species_name() -> Result<String> {
    println!("Enter a species name and press <Enter>:");

    let mut name = String::new();

    std::io::stdin()
        .read_line(&mut name)
        .context("Reading name")?;

    Ok(name.trim().to_lowercase())
}
