use log::{info};
use log4rs;

fn main() {
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found.");
    info!("We now have nice logging!");
}
