#[macro_use] extern crate diesel;
#[macro_use] extern crate error_chain;
extern crate env_logger;
extern crate chrono;

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
use actix_web::HttpServer;

use app::State;

fn main() {
    env_logger::init();

    let config_path = ::std::env::var("HEL_CONFIG_PATH")
        .unwrap_or("config.toml".into());
    let config = config::Config::load(config_path).unwrap();

    let sys = System::new("hel2-back");

    let db_config = config.clone();
    let db = SyncArbiter::start(config.db_threads, move || {
        db::DbExecutor {
            conn: db::establish_connection(&db_config.db).unwrap()
        }
    });

    let state = State::new(config.clone(), db);

    let mut b = HttpServer::new(move || app::create(state.clone()));

    if let Some(http_threads) = config.http_threads {
        b = b.threads(http_threads);
    }

    b.bind(config.address).unwrap().start();
    sys.run();
}
