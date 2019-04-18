mod app;
mod game;
mod logger;

use app::Application;
use clap::{App, Arg};
use game::GameState;
use logger::{prelude::*, Logger};
use specs::DispatcherBuilder;

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

    let dispatcher = DispatcherBuilder::new().build();
    let state = GameState::new();

    match Application::new(dispatcher, state) {
        Ok(mut app) => {
            info!("Starting {} [{}]...", NAME, VERSION);

            if let Err(err) = app.run() {
                error!("Game crashed: {}", err);
            }
        }
        Err(err) => error!("Initialization failed: {}", err),
    }
}
