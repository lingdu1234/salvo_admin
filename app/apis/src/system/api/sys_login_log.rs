use db::{common::res::ListData, system::prelude::SysLoginLogModel};
use salvo::{handler, Request};

use crate::{system::api_fn, utils::res_util::Res};

/// get_list 获取列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(request: &mut Request) -> Res<ListData<SysLoginLogModel>> {
    match api_fn::sys_login_log::get_sort_list(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 删除登录日志
#[handler]
pub async fn delete(request: &mut Request) -> Res<String> {
    match api_fn::sys_login_log::delete(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 清楚登录日志
#[handler]
pub async fn clean() -> Res<String> {
    match api_fn::sys_login_log::clean().await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
