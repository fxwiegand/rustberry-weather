#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
#[macro_use] extern crate diesel;

//extern crate linux_embedded_hal as hal;
extern crate rocket_contrib;
extern crate clap;
extern crate tera;
extern crate dotenv;

//use bme280::BME280;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use clap::{App, SubCommand, ArgMatches};
use tera::{Tera, Context};
use std::collections::HashMap;
use measure::{Measurement, make_measurement};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::{Local, DateTime};
use self::models::Value;
use crate::models::NewValue;


mod measure;

pub mod schema;
pub mod models;

#[get("/current")]
fn current() -> Json<Measurement> {
    let response = make_measurement();
    Json(response)
}

/*#[get("/average/")]
fn average() -> Json<Measurement> {
    // Read from Database and calculate average values
    Json(response)
}*/

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<&str , Vec<()>> = HashMap::new();
    //context.insert();

    Template::render("index", &context)
}

fn main() {
    let matches = App::new("rustberry weather")
        .version("1.0")
        .author("Felix W. <fxwiegand@wgdnet.de>")
        .about("a weather station for the raspberry pi in rust")
        .subcommand(SubCommand::with_name("server")
            .about("starts server")
            .version("1.0")
            .author("Felix W. <fxwiegand@wgdnet.de>"))
        .get_matches();


    match matches.subcommand_name() {
        Some("server") => {
            rocket::ignite()
                .mount("/",  StaticFiles::from("static"))
                .mount("/", routes![index])
                .mount("/api/v1", routes![current])
                .attach(Template::fairing())
                .launch();
        },
        None        => println!("Try using a subcommand. Type help for more."),
        _           => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }
}

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