use http::{HeaderValue, Method};

use micro_web::{handler_fn, Router, Server};
use micro_web::filter::header;
use micro_web::router::{get, post};

async fn simple_handler_1(method: &Method, str: Option<String>, str2: Option<String>) -> String {
    println!("receive body: {}, {}", str.is_some(), str2.is_some());
    format!("handler_1 : receive from method: {}\r\n", method)
}

async fn simple_handler_2(method: &Method) -> String {
    format!("handler_2: receive from method: {}\r\n", method)
}

async fn simple_handler_3(method: &Method, str: Option<String>, str2: Option<String>) -> String {
    println!("receive body: {}, {}", str.is_some(), str2.is_some());
    format!("handler_3: receive from method: {}\r\n", method)
}

async fn simple_handler_4(method: &Method, str: Option<String>, str2: Option<String>) -> String {
    println!("receive body: {}, {}", str.is_some(), str2.is_some());
    format!("handler_4: receive from method: {}\r\n", method)
}



async fn default_handler() -> &'static str {
    "404 not found"
}


#[tokio::main]
async fn main() {
    let router = Router::builder()
        .route("/", get(handler_fn(simple_handler_1)))
        .route(
            "/",
            post(handler_fn(simple_handler_2)).with(header(
                http::header::CONTENT_TYPE,
                HeaderValue::from_str(mime::APPLICATION_WWW_FORM_URLENCODED.as_ref()).unwrap(),
            )),
        )
        .route("/", post(handler_fn(simple_handler_3)))
        .route("/4", get(handler_fn(simple_handler_4)))
        .build();

    Server::builder()
        .router(router)
        .default_handler(handler_fn(default_handler))
        .build()
        .start()
        .await;
}
