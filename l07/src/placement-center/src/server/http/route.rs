use crate::storage::data_wrap::StorageDataWrap;
use crate::storage::rocksdb::RocksDBEngine;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

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
pub fn routes(rocksdb_engine_handler: Arc<RocksDBEngine>) -> Router {
    let service_routes = Router::new()
        .route("/", get(root_route()))
        .route(&v1_path(ROUTE_ROOT), get(root_route()))
        .route(
            &v1_path(ROUTE_SET),
            get(route_set(rocksdb_engine_handler.clone())),
        )
        .route(
            &v1_path(ROUTE_GET),
            get(route_get(rocksdb_engine_handler.clone())),
        )
        .route(&v1_path(ROUTE_LIST), get(route_list()))
        .route(&v1_path(ROUTE_CREATE), post(route_create()));
    let app = Router::new().merge(service_routes);
    app
}

fn root_route() -> &'static str {
    "Hello, World!"
}

fn route_set(rocksdb_engine_handler: Arc<RocksDBEngine>) -> () {
    let cf = rocksdb_engine_handler.cf_cluster();
    rocksdb_engine_handler.write_str(cf, "k1", String::from("v1"));
}

fn route_get(rocksdb_engine_handler: Arc<RocksDBEngine>) -> String {
    let cf = rocksdb_engine_handler.cf_cluster();
    rocksdb_engine_handler.read_str(cf, "k1")
}

fn route_list() -> &'static str {
    "List"
}

fn route_create() -> &'static str {
    "Create"
}
