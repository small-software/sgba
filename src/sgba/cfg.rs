
use clap::{Parser};

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
}