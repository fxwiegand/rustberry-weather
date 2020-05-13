extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use hal::{Delay, I2cdev};
use bme280::BME280;
use std::time::SystemTime;

#[derive(Serialize, Clone, Debug)]
pub(crate) struct Measurement {
    humidity: f32,
    temperature: f32,
    pressure: f32,
    //time: SystemTime,
    time: string
}

pub(crate) fn make_measurement() -> Measurement {
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);
    bme280.init().unwrap();

    let measurements = bme280.measure().unwrap();
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();

    let measurement = Measurement {
        humidity: measurements.humidity,
        temperature: measurements.temperature,
        pressure: measurements.pressure,
        //time: now,
        time: datetime.format("%d.%m.%Y %H:%M:%S")
    };

    measurement
}

fn write_data(measurement: Measurement) {
    // write data to database
}