-- Your SQL goes here
CREATE TABLE pets (
    id         SERIAL       PRIMARY KEY,
    name       VARCHAR(255) NOT NULL,
    species_id INT NOT NULL REFERENCES species(id)
);
