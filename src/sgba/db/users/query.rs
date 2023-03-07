use std::fmt::Error;
use bb8_postgres::bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::sgba::db::users::user::User;

pub async fn get_users(conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>) -> Result<Vec<User>, Error> {

    // let result = client.query("SELECT id, name FROM users", &[]).await?;
    // let users: Vec<User> = result.into_iter().map(|row| User::from(row)).collect();

    let res = conn
        .query("SELECT * FROM users", &[])
        .await.unwrap()
        .into_iter()
        .map(|row| User::from(row))
        .collect::<Vec<User>>();
    Ok(res)
}