pub mod messages;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use actix::{SyncContext, Actor};

use super::error::Result;

pub fn establish_connection(db: &str) -> Result<PgConnection> {
    Ok(PgConnection::establish(db)?)
}

pub struct DbExecutor {
    pub conn: PgConnection
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
