#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate linux_embedded_hal as hal;
extern crate rocket_contrib;
extern crate clap;
extern crate tera;

use bme280::BME280;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use clap::{App, SubCommand, ArgMatches};
use tera::{Tera, Context};
use std::collections::HashMap;

mod measure;

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
                .launch();
        },
        None        => println!("Try using a subcommand. Type help for more."),
        _           => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }
}
