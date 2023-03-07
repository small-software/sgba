use tokio_postgres::Row;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,               // 100
    pub mail: String,               // 254 //https://haacked.com/archive/2007/08/21/i-knew-how-to-validate-an-email-address-until-i.aspx
    pub passwd: String,             // 128
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            mail: row.get("mail"),
            passwd: row.get("passwd"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}