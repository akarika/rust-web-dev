#[macro_use]


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use diesel::r2d2::{Pool,PooledConnection,ConnectionManager,PoolError};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;



fn init_pool(database_url:&str)-> Result<PgPool,PoolError>{
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}



pub fn establish_connection() -> PgPool {
    dotenv().ok(); // This will load our .env file.

    // Load the DATABASE_URL env variable into database_url, in case of error
    // it will through a message "DATABASE_URL must be set"
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    init_pool(&database_url).expect("Failed to create pool")


}