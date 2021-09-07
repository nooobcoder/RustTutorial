use actix_web::{middleware, web, App, HttpServer, Result};
use serde::Serialize;
use std::cell::Cell;
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}
#[macro_use]
extern crate actix_web;

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    messages: Vec<String>,
}

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
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let ms = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: ms.clone(),
    }))
}

pub fn run(&self) -> std::io::Result<()> {
    let messages = Arc::new(Mutex::new(vec![]));
    println!("Starting http server: 127.0.0.1:{}", self.port);
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                request_count: Cell::new(0),
                messages: messages.clone(),
            })
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind(("0.0.0.0", self.port))?
    .workers(8)
    .run();
}
