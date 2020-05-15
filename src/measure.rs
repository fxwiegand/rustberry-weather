use chrono::offset::Utc;
use chrono::{Local, DateTime, Datelike, NaiveDateTime};
use chrono_locale::LocaleDate;
use hal::{Delay, I2cdev};
use bme280::BME280;
use rand::Rng;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::models::{NewValue, Value};
use bigdecimal::FromPrimitive;

#[derive(Serialize, Clone, Debug)]
pub(crate) struct Measurement {
    humidity: f32,
    temperature: f32,
    pressure: f32,
    time: String,
}

fn bme280_mockup() -> Measurement {
    let datetime: DateTime<Local> = Local::now();
    let mut rng = rand::thread_rng();
    let h = rng.gen_range(30.0, 60.0) as f32;
    let t = rng.gen_range(0.0, 30.0) as f32;
    let p = rng.gen_range(950.0, 1050.0) as f32;

    let m1 = Measurement {
        humidity: h.round(),
        temperature: t.round(),
        pressure: p.round(),
        time: get_naive_datetime().to_string(),
    };

    m1
}

pub(crate) fn make_measurement() -> Measurement {
    let conn = establish_connection();

    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);
    bme280.init().unwrap();

    let measurements = bme280.measure().unwrap();

    let naive_datetime = get_naive_datetime();

    let measurement = Measurement {
        humidity: measurements.humidity,
        temperature: measurements.temperature,
        pressure: measurements.pressure,
        time: naive_datetime.to_string(),
    };

    println!("{:?}", measurements.pressure.clone());
    println!("{:?}", bigdecimal::FromPrimitive::from_f32(measurements.pressure.clone()).unwrap());

    //let measurement = bme280_mockup();
    let value = create_value(&conn,
                             naive_datetime,
                             measurement.temperature,
                             measurement.pressure,
                             measurement.humidity
    );

    measurement
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
    use crate::schema::values;

    let new_value = NewValue {
        timestamp: timestamp,
        temperature: bigdecimal::FromPrimitive::from_f32(temperature).unwrap(),
        pressure:bigdecimal::FromPrimitive::from_f32(pressure).unwrap(),
        humidity: bigdecimal::FromPrimitive::from_f32(humidity).unwrap(),
    };

    println!("{:?}", new_value);

    diesel::insert_into(values::table)
        .values(&new_value)
        .get_result(conn)
        .expect("Error saving new value")
}

fn get_naive_datetime() -> NaiveDateTime {
    let datetime: DateTime<Local> = Local::now();
    let date_string = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let naive_datetime = NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap();

    naive_datetime
}