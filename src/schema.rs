// @generated automatically by Diesel CLI.

diesel::table! {
    pets (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        species_id -> Int4,
        last_fed -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    species (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(pets -> species (species_id));

diesel::allow_tables_to_appear_in_same_query!(pets, species,);
