#[macro_use]
extern crate actix_web;

#[derive(Serialize)]
struct IndexResponse {
    message: String,
}

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;

pub struct MessagesApp {
    port: u16,
}

impl MessagesApp {
    pub fn new(port: u16) -> Self {
        MessagesApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{} ", self.port);
        let result = HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
        .bind(("0.0.0.0", self.port));
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        result.unwrap().workers(8).run()
    }
}

#[get("/")]
fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| "world");

    Ok(web::Json(IndexResponse {
        message: hello.to_owned(),
    }))
}
