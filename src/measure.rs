use chrono::offset::Utc;
use chrono::{Local, DateTime, TimeZone};
use chrono_locale::LocaleDate;
use hal::{Delay, I2cdev};
use bme280::BME280;
//use std::time::SystemTime;

#[derive(Serialize, Clone, Debug)]
pub(crate) struct Measurement {
    humidity: f32,
    temperature: f32,
    pressure: f32,
    //time: SystemTime,
    time: String
}

pub(crate) fn make_measurement() -> Measurement {
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);
    bme280.init().unwrap();

    let measurements = bme280.measure().unwrap();
    //let now = SystemTime::now();
    let datetime: DateTime<Local> = Local::now();

    let measurement = Measurement {
        humidity: measurements.humidity,
        temperature: measurements.temperature,
        pressure: measurements.pressure,
        //time: now,
        time: datetime.formatl("%a, %d %B %Y %H:%M:%S", "en").to_string()
    };

    measurement
}

fn write_data(measurement: Measurement) {
    // write data to database
}