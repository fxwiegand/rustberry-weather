extern crate linux_embedded_hal as hal;
use bme280::BME280;
use tera;
use rocket::State;
use tera::Template;
use std::collections::HashMap;
use std::path::Path;

mod measure;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    //context.insert();

    Template::render("report", &context)
}

fn main() {
    let matches = App::new("rustberry weather")
        .version("1.0")
        .author("Felix W. <fxwiegand@wgdnet.de>")
        .about("a weather station for the raspberry pi in rust")
        .subcommand(SubCommand::with_name("server")
            .about("starts server")
            .version("1.0")
            .author("Felix W. <fxwiegand@wgdnet.de>"));

    match matches.subcommand_name() {
        Some("server") => {
            rocket::ignite()
                .mount("/",  StaticFiles::from("static"))
                .mount("/api/v1", routes![index])
                .launch();
        },
        None        => println!("Try using a subcommand. Type help for more."),
        _           => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }
}
