mod logger;

use clap::{App, Arg};
use logger::prelude::*;
use logger::Logger;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let clap = App::new(NAME)
        .version(VERSION)
        .arg(
            Arg::with_name("color")
                .long("color")
                .short("c")
                .help("Enables console coloring"),
        )
        .get_matches();
    let color = clap.is_present("color");

    Logger::init(color, &[]);

    info!("Starting {} [{}]...", NAME, VERSION);
}
