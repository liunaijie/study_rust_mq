use axum::routing::{get, post};
use axum::Router;

const VERSION_1: &str = "v1";

const ROUTE_ROOT: &str = "root";
const ROUTE_GET: &str = "get";
const ROUTE_LIST: &str = "list";
const ROUTE_CREATE: &str = "create";

//构建请求地址
fn v1_path(path: &str) -> String {
    format!("/{}/{}", VERSION_1, path)
}

// 注册请求地址与处理函数
pub fn routes() -> Router {
    let service_routes = Router::new()
        .route("/", get(root_route()))
        .route(&v1_path(ROUTE_ROOT), get(root_route()))
        .route(&v1_path(ROUTE_GET), get(route_get()))
        .route(&v1_path(ROUTE_LIST), get(route_list()))
        .route(&v1_path(ROUTE_CREATE), post(route_create()));
    let app = Router::new().merge(service_routes);
    app
}

fn root_route() -> &'static str {
    "Hello, World!"
}

fn route_get() -> &'static str {
    "Get"
}

fn route_list() -> &'static str {
    "List"
}

fn route_create() -> &'static str {
    "Create"
}
