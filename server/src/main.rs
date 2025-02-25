use salvo::{cors::Cors, http::Method, oapi::extract::QueryParam, prelude::*};
use tracing_subscriber::fmt::time;

#[endpoint]
async fn hello(name: QueryParam<String, false>) -> String {
    format!("Hello, {}!", name.as_deref().unwrap_or("world"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_timer(time::SystemTime::default()).init();
    let router = Router::new().push(Router::with_path("/api").push(Router::with_path("/hello").get(hello)));
    let acceptor = TcpListener::new("[::]:5701").bind().await;

    let doc = OpenApi::new("server api", "0.0.1").merge_router(&router);

    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));

    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .into_handler();

    let service = Service::new(router).hoop(Logger::new()).hoop(cors);

    Server::new(acceptor).serve(service).await;
}

