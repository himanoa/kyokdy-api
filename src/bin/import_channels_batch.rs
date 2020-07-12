use std::collections::HashMap;
use std::env::vars;
use std::process::exit;
use std::sync::Arc;

use anyhow::Result;
use dotenv::dotenv;
use log::{error, info};
use pretty_env_logger;
use structopt::StructOpt;
use tokio::spawn;
use tokio_postgres::NoTls;

use kyokdy_api::domain::channel::bulk_register_service::BulkRegisterService;
use kyokdy_api::infra::channel::postgresql_repository::PostgreSQLChannelRepository;
use kyokdy_api::infra::vtuber::vlueprint_vtuber_repository::VlueprintVTuberRepository;

#[derive(StructOpt, Debug)]
#[structopt(name = "import-channels-batch")]
struct Opt {
    #[structopt(short, long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::try_init_timed()?;
    dotenv().ok();

    let envs: HashMap<_, _> = vars().collect();

    let db_config = envs.get("DATABASE_URL").unwrap_or_else(|| {
        error!("DATABASE_URL must be set.");
        exit(1);
    });

    let opt: Opt = Opt::from_args();

    let (pg_client, pg_connection) = tokio_postgres::connect(db_config, NoTls)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to establish connection to database: {}", e);
            exit(1);
        });

    let a_pg_client = Arc::new(pg_client);

    spawn(async move {
        if let Err(e) = pg_connection.await {
            error!("Connection error: {}", e);
            exit(1);
        }
    });

    let service = BulkRegisterService::new(
        Arc::new(VlueprintVTuberRepository {}),
        Arc::new(PostgreSQLChannelRepository::new(a_pg_client)),
    );
    if opt.dry_run {
        info!("Start import channels batch(dry_run)");
        service.dry_run().await
    } else {
        info!("Start import channels batch");
        service.run().await
    }
}
