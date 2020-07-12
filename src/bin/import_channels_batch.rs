use std::sync::Arc;

use dotenv::dotenv;
use log::{error, info};
use pretty_env_logger;
use structopt::StructOpt;

use kyokdy_api::domain::channel::bulk_register_service::BulkRegisterService;
use kyokdy_api::infra::vtuber::vlueprint_vtuber_repository::VlueprintVTuberRepository;

#[derive(StructOpt, Debug)]
#[structopt(name = "import-channels-batch")]
struct Opt {
    #[structopt(short, long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::try_init_timed()?;
    dotenv().ok();

    let opt: Opt = Opt::from_args();

    let service = BulkRegisterService::new(Arc::new(VlueprintVTuberRepository {}));
    if opt.dry_run {
        info!("Start import channels batch(dry_run)");
        service.dry_run().await?
    }
    Ok(())
}
