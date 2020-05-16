# rustberry-weather

A weather station for the raspberry pi written in rust

## What you need

* A [raspberry pi](https://www.raspberrypi.org) with [rust](https://www.rust-lang.org) installed
* A [Bosch bme280](https://www.bosch-sensortec.com/products/environmental-sensors/humidity-sensors-bme280/) sensor with breakout board
* Some jumper cables for the connection between the pi and the sensor

## Setup your weather station

* Connect the vcc, gnd, scl and sda pins of your bme280 sensor to the corresponding pins of your raspberry pi
* Clone the repository
* Configure the [Rocket.toml](Rocket.toml) so the website is running on your desired adress, port etc.
* Setup an [Postgres](https://www.postgresql.org) database and export your database url with `export DATABASE_URL=postgres://username:password@domain:5432/yourdatabase`
* Either grant yourself access to `/dev/i2c-1/` for letting the program connect to the sensor to run with `cargo run server` or build the executable with `cargo build --release` and run as sudo with `sudo target/release/rustberry-weather server`


## Built with

* [Rocket](https://rocket.rs) - A web framework for Rust
* [Tera](https://tera.netlify.app) - A powerful, easy to use template engine for Rust
* [bme280](https://github.com/uber-foo/bme280-rs) - A rust device driver for the Bosch BME280

## Authors

* **Felix Wiegand** - (https://github.com/fxwiegand)
* **Christian Wiegand** - (https://github.com/christianwgd)


## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

