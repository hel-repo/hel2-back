#[macro_use] extern crate diesel;
extern crate env_logger;
extern crate chrono;

extern crate failure;
#[macro_use] extern crate failure_derive;

extern crate futures;
extern crate actix;
extern crate actix_web;

extern crate serde;
extern crate toml;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod error;
mod config;
mod db;
mod app;
mod resources;
mod models;

use actix::{System, SyncArbiter};
use actix_web::server;

use app::State;

fn main() {
    env_logger::init();

    let config_path = ::std::env::var("HEL_CONFIG_PATH")
        .unwrap_or("config.toml".into());
    let config = config::Config::load(config_path).unwrap();

    let sys = System::new("hel2-back");

    let db_config = config.database.clone();
    let db = SyncArbiter::start(config.database.threads, move || {
        db::DbExecutor {
            conn: db::establish_connection(&db_config.url).unwrap()
        }
    });

    let state = State::new(config.clone(), db);

    let mut b = server::new(move || app::create(state.clone()));

    if let Some(http_threads) = config.http.threads {
        b = b.threads(http_threads);
    }

    b.bind(config.http.bind_address).unwrap().start();
    sys.run();
}