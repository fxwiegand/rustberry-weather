use chrono::offset::Utc;
use chrono::{Local, DateTime, Datelike, NaiveDateTime};
use chrono_locale::LocaleDate;
//use hal::{Delay, I2cdev};
//use bme280::BME280;
//use std::time::SystemTime;
use rand::Rng;
use diesel;
use rustberry_weather::{establish_connection, create_value};

#[derive(Serialize, Clone, Debug)]
pub(crate) struct Measurement {
    humidity: f32,
    temperature: f32,
    pressure: f32,
    time: String,
    time_de: String
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
        time: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        time_de: datetime.formatl("%a, %d. %B %Y %H:%M:%S", "de").to_string(),
    };

    m1
}

pub(crate) fn make_measurement() -> Measurement {
    let conn = establish_connection();

    // let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    // let mut bme280 = BME280::new_primary(i2c_bus, Delay);
    // bme280.init().unwrap();
    //
    // let measurements = bme280.measure().unwrap();
    // //let now = SystemTime::now();
    let datetime: DateTime<Local> = Local::now();
    let date_string = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let naive_datetime = NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap();

    // let measurement = Measurement {
    //     humidity: measurements.humidity,
    //     temperature: measurements.temperature,
    //     pressure: measurements.pressure,
    //     //time: now,
    //     time: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
    //     time_de: datetime.formatl("%a, %d. %B %Y %H:%M:%S", "de").to_string(),
    // };

    let measurement = bme280_mockup();
    let value = create_value(&conn,
                             naive_datetime,
                             measurement.temperature,
                             measurement.humidity,
                             measurement.pressure
    );

    measurement
}

fn write_data(measurement: Measurement) {
    // write data to database
}