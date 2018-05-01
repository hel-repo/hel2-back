use actix_web::{
    AsyncResponder,
    Error as ActixError,
    HttpRequest,
    HttpResponse,
    HttpMessage,
    Path as PathExtractor,
    State as StateExtractor,
};
use futures::Future;

use ::app::State;
use ::db::messages;
use ::models;

type ResponseFuture = Box<Future<Item=HttpResponse, Error=ActixError>>;

pub fn index(_req: HttpRequest<State>) -> &'static str {
    "Hello, World!"
}

pub fn list_packages(req: HttpRequest<State>) -> ResponseFuture {
    let db = req.state().db.clone();
    let page_limit = req.state().config.http.pagination_limit;

    req.json()
        .from_err::<ActixError>()
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

pub fn get_package(state: StateExtractor<State>, path: PathExtractor<models::api::Name>)
    -> ResponseFuture
{
    state.db.send(messages::GetPackage(path.name.clone()))
        .from_err::<ActixError>()
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(res?))
        })
        .from_err()
        .responder()
}