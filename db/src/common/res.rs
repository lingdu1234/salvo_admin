use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
/// 查 数据返回
pub struct ListData<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub total_pages: u64,
    pub page_num: u64,
}
/// 分页参数
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct PageParams {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct ResErr {
    pub code: Option<i32>,
    pub data: Option<String>,
    pub msg: Option<String>,
}
