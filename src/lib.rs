#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::{Local, DateTime};
use self::models::Value;
use crate::models::NewValue;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_value(conn: &PgConnection,
                    timestamp: chrono::NaiveDateTime, temperature: f32,
                    pressure: f32, humidity: f32) -> Value {
    use schema::values;

    let new_value = NewValue {
        timestamp: timestamp,
        temperature: bigdecimal::FromPrimitive::from_f32(temperature).unwrap(),
        pressure:bigdecimal::FromPrimitive::from_f32(pressure).unwrap(),
        humidity: bigdecimal::FromPrimitive::from_f32(humidity).unwrap(),
    };

    diesel::insert_into(values::table)
        .values(&new_value)
        .get_result(conn)
        .expect("Error saving new value")
}