use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SysOperLogSearchReq {
    pub oper_id: Option<String>,
    pub title: Option<String>,
    pub oper_name: Option<String>,
    pub operator_type: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize)]
pub struct SysOperLogDeleteReq {
    pub oper_log_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OperationLogData {
    pub req_id: String,
    pub req_user: String,
    pub req_time: String,
    pub time_id: i64,
    pub req_ip: String,
    pub req_version: String,
    pub req_method: String,
    pub req_url: String,
    pub req_ori_url: String,
    pub req_data: String,
    pub res_status: String,
    pub res_body: String,
    pub res_time: String,
    pub elapsed_time: String,
}
