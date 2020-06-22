use dotenv::dotenv;
use kyokdy_api::application::Application;
use log::{error, info};
use pretty_env_logger;
use tokio::spawn;
use tokio_postgres::NoTls;
use warp::serve;

use std::{collections::HashMap, env::vars, net::SocketAddr, process::exit};

use kyokdy_api::routes::routes;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv().ok();
    let envs: HashMap<_, _> = vars().collect();

    let db_config = envs.get("DATABASE_URL").unwrap_or_else(|| {
        error!("DATABASE_URL must be set.");
        exit(1);
    });

    let listen_address: SocketAddr = envs
        .get("LISTEN_ADDRESS")
        .map_or("127.0.0.1:3000".to_string(), |address| address.to_string())
        .parse()
        .unwrap_or_else(|e| {
            error!("Failed to parse `LISTEN_ADDRESS`: {}", e);
            exit(1);
        });

    let (pg_client, pg_connection) = tokio_postgres::connect(db_config, NoTls)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to establish connection to database: {}", e);
            exit(1);
        });

    spawn(async move {
        if let Err(e) = pg_connection.await {
            error!("Connection error: {}", e);
            exit(1);
        }
    });

    let application = Application::new(pg_client);
    serve(routes(application)).run(listen_address).await;
    Ok(())
}
