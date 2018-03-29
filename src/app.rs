use actix::{Addr, Syn};
use actix_web::{Application, Method};
use actix_web::middleware::Logger;

use resources;
use db::DbExecutor;
use config::Config;

#[derive(Clone)]
pub struct State {
    config: Config,
    db: Addr<Syn, DbExecutor>
}

impl State {
    pub fn new(config: Config, db: Addr<Syn, DbExecutor>) -> State {
        State { config, db }
    }
}

pub fn create(state: State) -> Application<State> {
    Application::with_state(state)
        .middleware(Logger::default())
        .resource("/", |r| r.method(Method::GET).f(resources::index))
}
