use crate::sgba::db::users::user::User;
use bb8_postgres::bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use std::fmt::Error;
use tokio_postgres::NoTls;
use crate::sgba::api::data::users::dto::{SingUp, SingIn};

pub async fn get(
    conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>,
) -> Result<Vec<User>, Error> {
    let res = conn
        .query("SELECT * FROM users", &[])
        .await
        .unwrap()
        .into_iter()
        .map(|row| User::from(row))
        .collect::<Vec<User>>();

    Ok(res)
}

pub async fn create(
    conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    sing_up: SingUp,
) -> Result<bool, Error> {

    if check_email_exist(conn, &sing_up.email).await.unwrap() {
        panic!("not correct email")
    }
    let res = conn
        .query_one(
            "INSERT INTO \
                                     users (\
                                         name, \
                                         email, \
                                         passwd) \
                                     VALUES ($1, $2, $3) \
                               RETURNING *",
            &[
                &sing_up.name,
                &normalize_email(&sing_up.email).await.unwrap(),
                &sing_up.passwd,
            ],
        )
        .await
        .unwrap();

    Ok(res.len() > 0)
}

pub async fn check(
    conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    sing_in: SingIn,
) -> Result<bool, Error> {
    let res = conn
        .query_one(
            "SELECT \
                        name \
                       FROM \
                        users \
                       WHERE \
                        email = $1 and passwd = $2",
            &[
                &normalize_email(&sing_in.email).await.unwrap(),
                &sing_in.passwd,
            ],
        )
        .await
        .unwrap();

    Ok(res.len() > 0)
}

async fn check_email_exist(
    conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    email:&str
) -> Result<bool, Error>{

    let res = conn.query(
        "SELECT \
                    * \
                   FROM \
                    users \
                   WHERE \
                    email = $1",
        &[
            &normalize_email(email).await.unwrap()
        ])
        .await
        .unwrap();

    Ok(res.len()>0)
}

async fn normalize_email(email: &str) -> Result<String, Error> {
    let mut mut_email = email.to_string();
    mut_email.make_ascii_lowercase();
    let res: &str = mut_email.trim();

    Ok(res.to_owned())
}