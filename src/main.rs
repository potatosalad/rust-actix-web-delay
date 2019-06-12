use actix_web::{error, http, middleware, web, App, Error, HttpResponse, HttpServer};
use futures::{
    future::{err, ok},
    Future,
};
use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::timer::Delay;

static PENDING: AtomicUsize = AtomicUsize::new(0);
static TOTAL: AtomicUsize = AtomicUsize::new(0);

fn delay(millis: u64) -> impl Future<Item = HttpResponse, Error = Error> {
    PENDING.fetch_add(1, Ordering::SeqCst);
    Delay::new(Instant::now() + Duration::from_millis(millis))
        .or_else(|_| err(error::ErrorInternalServerError("timer error".to_owned())))
        .and_then(move |_| {
            let pending = PENDING.fetch_sub(1, Ordering::SeqCst);
            let total = TOTAL.fetch_add(1, Ordering::SeqCst) + 1;
            ok(HttpResponse::Ok()
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body(format!("{}:{}:{}", millis, pending, total)))
        })
}

fn random_delay(info: web::Path<u64>) -> impl Future<Item = HttpResponse, Error = Error> {
    let millis = if *info == 0 {
        0
    } else {
        let mut rng = thread_rng();
        rng.gen_range(0, *info)
    };
    delay(millis)
}

fn static_delay(info: web::Path<u64>) -> impl Future<Item = HttpResponse, Error = Error> {
    delay(*info)
}

const DEFAULT_PORT: u16 = 8080_u16;

fn main() -> std::io::Result<()> {
    env_logger::init();
    let port: u16 = std::env::var("PORT")
        .map(|port| port.parse::<u16>().unwrap_or(DEFAULT_PORT))
        .unwrap_or(DEFAULT_PORT);
    assert!(port > 0);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/random/{millis}").route(web::get().to_async(random_delay)))
            .service(web::resource("/static/{millis}").route(web::get().to_async(static_delay)))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
}
