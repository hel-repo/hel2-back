use actix_web::{self, HttpRequest, HttpResponse, HttpMessage, AsyncResponder};
use futures::Future;

use ::app::State;
use ::db::messages;
use ::models;

type ResponseFuture<E> = Box<Future<Item=HttpResponse, Error=E>>;

pub fn index(_req: HttpRequest<State>) -> &'static str {
    "Hello, World!"
}

pub fn list_packages(req: HttpRequest<State>) -> ResponseFuture<actix_web::Error> {
    let db = req.state().db.clone();
    let page_limit = req.state().config.http.pagination_limit;

    req.json()
        .from_err::<actix_web::Error>()
        .and_then(move |page: models::api::PaginationRq| {
            let page = page.validate(page_limit);

            db.send(messages::GetPackages {
                page: page.page,
                limit: page.limit,
            }).from_err()
        })
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(res?))
        })
        .from_err()
        .responder()
}