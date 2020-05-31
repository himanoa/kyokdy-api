use tokio_postgres::NoTls;
use tokio::spawn;
use dotenv::dotenv;
use pretty_env_logger;
use log::{error, info};

use std::{collections::HashMap, env::vars, process::exit};


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    let envs: HashMap<_, _> = vars().collect();
    
    let db_config  = envs.get("DATABASE_URL").unwrap_or_else(|| {
        error!("DATABASE_URL must be set.");
        exit(1);
    });

    let (pg_client, pg_connection) = tokio_postgres::connect(db_config, NoTls).await.unwrap_or_else(|e| { 
        error!("Failed to establish connection to database: {}", e);
        exit(1);
    });

    spawn(async move {
        if let Err(e) = pg_connection.await {
            error!("Connection error: {}", e);
            exit(1);
        }
    });


    println!("Hello, world!");
}
