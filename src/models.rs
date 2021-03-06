use crate::schema::values;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Serialize)]
pub struct Value {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub temperature: BigDecimal,
    pub pressure: BigDecimal,
    pub humidity: BigDecimal,
}

#[derive(Insertable, Debug)]
#[table_name = "values"]
pub struct NewValue {
    pub timestamp: NaiveDateTime,
    pub temperature: BigDecimal,
    pub pressure: BigDecimal,
    pub humidity: BigDecimal,
}
