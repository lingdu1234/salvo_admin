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
pub struct ErrorResp {
    pub ____code: u16,
    pub name: String,
    pub brief: String,
    pub detail: String,
    pub cause: String,
}
