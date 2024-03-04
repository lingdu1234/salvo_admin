use db::{common::res::ListData, system::prelude::SysOperLogModel};
use middleware_fn::res_util::Res;
use salvo::{handler, Request};

use crate::system::api_fn;

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(request: &mut Request) -> Res<ListData<SysOperLogModel>> {
    match api_fn::sys_oper_log::get_sort_list(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(request: &mut Request) -> Res<String> {
    match api_fn::sys_oper_log::delete(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
#[handler]
pub async fn clean() -> Res<String> {
    match api_fn::sys_oper_log::clean().await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[handler]
pub async fn get_by_id(request: &mut Request) -> Res<SysOperLogModel> {
    match api_fn::sys_oper_log::get_by_id(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
