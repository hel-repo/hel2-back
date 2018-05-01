pub mod messages;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use failure::Error;

use actix::{SyncContext, Actor};

pub fn establish_connection(db: &str) -> Result<PgConnection, Error> {
    Ok(PgConnection::establish(db)?)
}

pub struct DbExecutor {
    pub conn: PgConnection
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
