use chrono::{Local, DateTime};
use schema::values;

#[derive(Queryable)]
pub struct Value {
    pub id: i32,
    pub timestamp: DateTime<Local>,
    pub temperature: f32,
    pub pressure: f32,
    pub humidity: f32,
}

#[derive(Insertable)]
#[table_name = "values"]
pub struct NewValue<'a> {
    pub timestamp: DateTime<Local>,
    pub temperature: f32,
    pub pressure: f32,
    pub humidity: f32
}