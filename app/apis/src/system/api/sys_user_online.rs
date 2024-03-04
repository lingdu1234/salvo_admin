use db::{common::res::ListData, system::prelude::SysUserOnlineModel};
use middleware_fn::res_util::Res;
use salvo::{handler, Depot, Request};

use crate::system::api_fn;

/// get_list 获取列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(request: &mut Request) -> Res<ListData<SysUserOnlineModel>> {
    match api_fn::sys_user_online::get_sort_list(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 删除
#[handler]
pub async fn delete(request: &mut Request) -> Res<String> {
    match api_fn::sys_user_online::delete(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 登出
#[handler]
pub async fn log_out(depot: &mut Depot) -> Res<String> {
    match api_fn::sys_user_online::log_out(depot).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
