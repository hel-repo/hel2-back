use actix_web::HttpRequest;
use app::State;

pub fn index(_req: HttpRequest<State>) -> &'static str {
    "Hello, World!"
}
