use dotenv::dotenv;
use log::{error, info};
use pretty_env_logger;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "import-channels-batch")]
struct Opt {
    #[structopt(short, long)]
    dry_run: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::try_init_timed()?;
    dotenv().ok();

    let opt = Opt::from_args();

    info!("Start import channels batch");
    Ok(())
}
