#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;

extern crate average;
extern crate clap;
extern crate dotenv;
extern crate linux_embedded_hal as hal;
extern crate rocket_contrib;
extern crate tera;

use crate::measure::{
    establish_connection, get_average_values, get_latest_value, get_max_values, get_min_values,
    get_values,
};
use crate::models::Value;
use clap::{App, SubCommand};
use measure::{measure, Measurement};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::{thread, time};

mod measure;

pub mod models;
pub mod schema;

#[get("/current")]
fn current() -> Json<Value> {
    let conn = establish_connection();
    let response = get_latest_value(&conn);
    Json(response)
}

#[get("/interval/<days>")]
fn interval(days: u32) -> Json<Vec<Value>> {
    let conn = establish_connection();
    let response = get_values(&conn, days);
    Json(response)
}

#[get("/average")]
fn average() -> Json<Measurement> {
    let conn = establish_connection();
    let response = get_average_values(&conn);
    Json(response)
}

#[get("/min")]
fn min() -> Json<Measurement> {
    let conn = establish_connection();
    let response = get_min_values(&conn);
    Json(response)
}

#[get("/max")]
fn max() -> Json<Measurement> {
    let conn = establish_connection();
    let response = get_max_values(&conn);
    Json(response)
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, Vec<()>> = HashMap::new();
    //context.insert();

    Template::render("index", &context)
}

fn main() {
    let matches = App::new("rustberry weather")
        .version("1.0")
        .author("Felix W. <fxwiegand@wgdnet.de>")
        .about("a weather station for the raspberry pi in rust")
        .subcommand(
            SubCommand::with_name("server")
                .about("starts server")
                .version("1.0")
                .author("Felix W. <fxwiegand@wgdnet.de>"),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("server") => {
            thread::spawn(move || {
                let sleep = time::Duration::from_millis(60000);

                loop {
                    measure();
                    thread::sleep(sleep);
                }
            });

            rocket::ignite()
                .mount("/", StaticFiles::from("static"))
                .mount("/", routes![index])
                .mount("/api/v1", routes![current, interval, average, min, max])
                .attach(Template::fairing())
                .launch();
        }
        None => println!("Try using a subcommand. Type help for more."),
        _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }
}
