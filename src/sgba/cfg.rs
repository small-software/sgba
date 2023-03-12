use std::sync::Mutex;
use clap::{Parser};
use once_cell::sync::OnceCell;


static JWT_SECRET: OnceCell<Mutex<String>> = OnceCell::new();

#[derive(Parser)]
#[command(author, version)]
pub struct SgbaDataArgs{
    /// db-user
    #[arg(short, long, default_value = "postgres")]
    pub user_db: String,
    /// db-passwd
    #[arg(short, long, default_value = "test")]
    pub passwd_db: String,

    /// port range 0..65535  0..2^16-1
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t=3000)]
    pub sport: u16,

    /// Enable log mode debug
    #[arg(short, long, default_value_t=false)]
    pub debug: bool,

    #[arg(short, long, default_value = "test" )]
    pub jwt_secret: String,
}

pub fn get_jwt_secret() -> String {
    ensure_jwt_secret().lock().unwrap().clone()
}

pub fn set_jwt_secret(jwt_secret: String) {
    *ensure_jwt_secret().lock().unwrap() = jwt_secret;
}

fn ensure_jwt_secret() -> &'static Mutex<String> {
    JWT_SECRET.get_or_init(|| Mutex::new(String::new()))
}

