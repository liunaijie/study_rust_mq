use crate::storage::data_wrap::StorageDataWrap;
use axum::Router;
use axum::extract::Path;
use axum::routing::{get, post};
use crate::storage::server::get_rocksdb_handler;

const VERSION_1: &str = "v1";

const ROUTE_ROOT: &str = "root";
const ROUTE_SET: &str = "set";
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
        .route("/v1/rocksdb/set/{key}/{value}", get(route_set))
        .route("/v1/rocksdb/get/{key}", get(route_get))
        .route(&v1_path(ROUTE_LIST), get(route_list()))
        .route(&v1_path(ROUTE_CREATE), post(route_create()));
    let app = Router::new().merge(service_routes);
    app
}

fn root_route() -> &'static str {
    "Hello, World!"
}

async fn route_set(Path(params): Path<(String, String)>) -> &'static str {
    let handler = get_rocksdb_handler();
    let cf = handler.cf_cluster();
    let data_wrap = StorageDataWrap::new(params.1);
    match handler.write(cf, &*params.0, data_wrap) {
        Ok(_) => "Done",
        Err(_) => "Failed to set value",
    }
}

async fn route_get(Path(key): Path<String>) -> String {
    let handler = get_rocksdb_handler();
    let cf = handler.cf_cluster();
    handler
        .read(cf, &*key)
        .unwrap()
        .get_data()
}

fn route_list() -> &'static str {
    "List"
}

fn route_create() -> &'static str {
    "Create"
}
