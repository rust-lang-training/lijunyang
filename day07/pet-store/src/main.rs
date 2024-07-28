use log;
use rocket::{fairing::AdHoc, get, http::Header, launch, routes};
use rocket_db_pools::Database;
use simple_logger;

mod common;
mod pet;
use common::{DbClient, RequestId};
#[get("/")]
fn index(request_id: RequestId) -> &'static str {
    log::info!("request_id: {}", &request_id.0);
    "hello world!!!!"
}

#[launch]
fn rocket() -> _ {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    log::debug!("Logger initialized with DEBUG level");

    rocket::build()
        .mount("/", routes![index])
        .attach(DbClient::init())
        .attach(AdHoc::on_response("request-id", |req, res| {
            Box::pin(async move {
                let request_id = req.guard::<RequestId>().await.unwrap().0;
                res.set_header(Header::new("X-Request-Id", request_id));
            })
        }))
        .attach(AdHoc::on_response("CORS", |_, res| {
            Box::pin(async move {
                res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
                res.set_header(Header::new(
                    "Access-Control-Allow-Methods",
                    "HEAD, GET, POST, PUT, DELETE, OPTIONS",
                ));
                res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
                res.set_header(Header::new("Access-Control-Max-Age", "86400"));
                res.set_header(Header::new("Access-Control-Expose-Headers", "X-Request-Id"));
            })
        }))
        .attach(pet::stage())
}
