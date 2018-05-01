use actix_web::error::ResponseError;

impl ResponseError for ::error::ParseError {}