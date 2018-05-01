use actix::{Addr, Syn};
use actix_web::{App};
use actix_web::http::Method;
use actix_web::middleware::Logger;

use resources;
use db::DbExecutor;
use config::Config;

#[derive(Clone)]
pub struct State {
    pub config: Config,
    pub db: Addr<Syn, DbExecutor>,
}

impl State {
    pub fn new(config: Config, db: Addr<Syn, DbExecutor>) -> State {
        State { config, db }
    }
}

pub fn create(state: State) -> App<State> {
    App::with_state(state)
        .middleware(Logger::default())
        .resource("/", |r| r.method(Method::GET).f(resources::index))
        .resource("/api/packages", |r| {
            r.method(Method::GET).a(resources::list_packages)
        })
}