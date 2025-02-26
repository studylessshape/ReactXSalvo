use std::fmt::Debug;

use clap::Parser;
use salvo::{cors::Cors, http::Method, oapi::extract::{FormBody, QueryParam}, prelude::*, routing::PathFilter};
use server::cli::StartArgs;
use ::time::{macros::format_description, UtcOffset};
use tracing_subscriber::fmt::time::OffsetTime;

#[endpoint]
async fn hello(name: QueryParam<String, false>) -> String {
    format!("Hello, {}!", name.as_deref().unwrap_or("world"))
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct AddFormBody {
    left: u64,
    right: u64
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct AddResult {
    result: u64,
}

#[endpoint]
async fn add(body: FormBody<AddFormBody>) -> Json<AddResult> {
    Json(AddResult {
        result: test_crates::add(body.left, body.right)
    })
}

#[tokio::main]
async fn main() {
    tracing_init();

    let args = StartArgs::parse();

    let router = router_build();

    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE, Method::HEAD, Method::CONNECT, Method::TRACE])
        .into_handler();

    let mut service = Service::new(router)
        // .hoop(Logger::new())
        .hoop(cors);
    
    if args.debug {
        service = service.hoop(Logger::new());
    }

    let acceptor = TcpListener::new("[::]:5701").bind().await;
    let server = Server::new(acceptor);
    let handle = server.handle();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        tracing::event!(tracing::Level::WARN, "Receive Ctrl+C, will stop server...");
        handle.stop_graceful(None);
    });

    server.serve(service).await;
}

fn tracing_init() {
    let timer = OffsetTime::new(
        UtcOffset::current_local_offset().expect("could not get local offset!"),
        format_description!("[year]-[month]-[day]T[hour repr:24]:[minute]:[second].[subsecond digits:3]")
    );
    tracing_subscriber::fmt().with_timer(timer).init();
}

fn router_build() -> Router {
    let static_rule = regex::Regex::new("(static|assets)").unwrap();
    PathFilter::register_wisp_regex("static_files", static_rule);
    let router = Router::new()
        .push(
            Router::with_path("/api")
            .push(Router::with_path("/hello").get(hello))
            .push(Router::with_path("/add").post(add))
        )
        .push(Router::with_path("{*path}").get(StaticDir::new(["./static", "./static/assets"]).auto_list(true).defaults("index.html")));

    let doc = OpenApi::new("server api", "0.0.1").merge_router(&router);

    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}