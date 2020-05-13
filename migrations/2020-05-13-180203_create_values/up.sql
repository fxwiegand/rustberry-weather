-- Your SQL goes here
CREATE TABLE values (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP NOT NULL,
    temperature NUMERIC (6, 2) NOT NULL,
    pressure NUMERIC (6,2) NOT NULL,
    humidity NUMERIC (6,2) NOT NULL
)