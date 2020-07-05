use dotenv::dotenv;
use kyokdy_api::application::Application;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "import-channels-batch")]
struct Opt {
    #[structopt(short, long)]
    dry_run: bool
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    Ok(())
}
