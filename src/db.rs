use diesel::prelude::*;
use diesel::pg::PgConnection;

use actix::{SyncContext, Actor};

use error::Result;

pub fn establish_connection(db: &str) -> Result<PgConnection> {
    Ok(PgConnection::establish(db)?)
}

pub struct DbExecutor(pub PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
