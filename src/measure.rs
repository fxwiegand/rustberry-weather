use chrono::{Local, DateTime, NaiveDateTime};
use hal::{Delay, I2cdev};
use bme280::BME280;
use rand::Rng;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::models::{NewValue, Value};
use bigdecimal::{ToPrimitive};
use average::{Mean, Estimate};

#[derive(Serialize, Clone, Debug)]
pub struct Measurement {
    humidity: f32,
    temperature: f32,
    pressure: f32,
    time: String,
}

/*fn bme280_mockup() -> Measurement {
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
}*/

pub fn measure() {
    let conn = establish_connection();

    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);
    bme280.init().unwrap();

    let measurements = bme280.measure().unwrap();

    let naive_datetime = get_naive_datetime();

    create_value(&conn, naive_datetime, measurements.temperature,measurements.pressure/100 as f32, measurements.humidity);
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

    
    diesel::insert_into(values::table)
        .values(&new_value)
        .get_result(conn)
        .expect("Error saving new value")
}

pub fn get_values(conn: &PgConnection, days: u32) -> Vec<Value> {
    use crate::schema::values::dsl::*;

    let mut sql = String::from("NOW()::DATE-EXTRACT(DOW FROM NOW())::INTEGER-");
    sql.push_str(&days.to_string());

    let v = values
        .filter(timestamp.gt(diesel::dsl::sql(&sql)))
        .load::<Value>(conn)
        .expect("Error loading posts");

    v
}

pub fn get_latest_value(conn: &PgConnection) -> Value {
    use crate::schema::values::dsl::*;

    let mut v = values
        .order(timestamp.desc())
        .limit(1)
        .load::<Value>(conn)
        .expect("Error loading posts");

    let m = v.pop();

    m.unwrap()
}

pub fn get_average_values(conn: &PgConnection) -> Measurement {
    let all_values = get_values(conn, 7);

    let mut temp: Mean = Mean::new();
    let mut hum: Mean = Mean::new();
    let mut pres: Mean = Mean::new();

    for v in all_values {
        temp.add(v.temperature.to_f64().unwrap());
        hum.add(v.humidity.to_f64().unwrap());
        pres.add(v.pressure.to_f64().unwrap());
    }

    let t = temp.mean();
    let p = pres.mean();
    let h = hum.mean();

    let m = Measurement {
        humidity: h as f32,
        temperature: t as f32,
        pressure: p as f32,
        time: "".to_string()
    };

    m
}

pub fn get_max_values(conn: &PgConnection) -> Measurement {
    let all_values = get_values(conn, 7);

    let mut min_temp = 0.0;
    let mut min_humi = 0.0;
    let mut min_pressure = 0.0;


    for v in all_values {
        if v.temperature.to_f64().unwrap() > min_temp {
            min_temp = v.temperature.to_f64().unwrap();
        }
        if v.pressure.to_f64().unwrap() > min_pressure {
            min_pressure= v.pressure.to_f64().unwrap();
        }
        if v.humidity.to_f64().unwrap() > min_humi {
            min_humi = v.humidity.to_f64().unwrap();
        }
    }

    let m = Measurement {
        humidity: min_humi as f32,
        temperature: min_temp as f32,
        pressure: min_pressure as f32,
        time: "".to_string()
    };

    m
}

pub fn get_min_values(conn: &PgConnection) -> Measurement {
    let all_values = get_values(conn, 7);

    let mut min_temp = 100.0;
    let mut min_humi = 100.0;
    let mut min_pressure = 10000.0;


    for v in all_values {
        if v.temperature.to_f64().unwrap() < min_temp {
            min_temp = v.temperature.to_f64().unwrap();
        }
        if v.pressure.to_f64().unwrap() < min_pressure {
            min_pressure= v.pressure.to_f64().unwrap();
        }
        if v.humidity.to_f64().unwrap() < min_humi {
            min_humi = v.humidity.to_f64().unwrap();
        }
    }

    let m = Measurement {
        humidity: min_humi as f32,
        temperature: min_temp as f32,
        pressure: min_pressure as f32,
        time: "".to_string()
    };

    m
}

fn get_naive_datetime() -> NaiveDateTime {
    let datetime: DateTime<Local> = Local::now();
    let date_string = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let naive_datetime = NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap();

    naive_datetime
}
