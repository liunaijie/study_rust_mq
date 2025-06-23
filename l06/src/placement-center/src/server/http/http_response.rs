use serde::{Deserialize, Serialize};
use std::ptr::null;

// 定义Http请求的返回信息
#[derive(Serialize, Deserialize)]
pub struct HttpResponse<T> {
    pub code: i32,
    pub data: T,
}



pub fn success_response<T: Serialize>(data: T) -> String {
    let response = HttpResponse { code: 0, data };
    serde_json::to_string(&response).unwrap()
}

pub fn error_response<T: Serialize>(data: T) -> String {
    error_response_with_customize_code(-1, data)
}

pub fn error_response_with_customize_code<T: Serialize>(code: i32, data: T) -> String {
    let response = HttpResponse { code, data };
    serde_json::to_string(&response).unwrap()
}
