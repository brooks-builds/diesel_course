use diesel::{migration::Result, pg::Pg, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");
    let db_connection =
        &mut PgConnection::establish(&database_url).expect("Error connecting to postgres database");

    run_migrations(db_connection).expect("Error running migrations");
}

fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<()> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
