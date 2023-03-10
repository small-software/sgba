mod sgba;
mod test;

use axum::{
    routing::{get,post},
    Router,
};

use bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;
use std::net::SocketAddr;
use tokio_postgres::NoTls;
use tracing::Level;

use clap::{Parser};
use crate::sgba::cfg::SgbaDataArgs;
use sgba::api::data::users::controller::*;

#[tokio::main]
async fn main() {

    let args = SgbaDataArgs::parse();
    let level_login = if args.debug {
        Level::DEBUG
    } else {
        Level::INFO
    };

    tracing_subscriber::fmt()
        .with_level(true)
        .with_target(true)
        .with_max_level(level_login)
        .compact()
        .init();

    // set up connection pool
    let manager =
        PostgresConnectionManager::new_from_stringlike(format!("host=localhost dbname=sgba user={} password={}",args.user_db,args.passwd_db).to_owned(), NoTls)
            .unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    let v_api = "v1";

    // build our application with some routes
    let app = Router::new()
        .route(&format!("/{}/users",v_api), get(get_users))
        .route(&format!("/{}/sing_up",v_api), post(sing_up))
        .route( &format!("/{}/sing_in",v_api), post(sing_in))
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], args.sport));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}