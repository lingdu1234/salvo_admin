use db::system::prelude::SysUpdateLogModel;
use middleware_fn::res_util::Res;
use salvo::{handler, Depot, Request};

use crate::system::api_fn;

/// add 添加
#[handler]
pub async fn add(depot: &mut Depot, request: &mut Request) -> Res<String> {
    match api_fn::sys_update_log::add(depot, request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(request: &mut Request) -> Res<String> {
    match api_fn::sys_update_log::delete(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
#[handler]
pub async fn edit(depot: &mut Depot, request: &mut Request) -> Res<String> {
    match api_fn::sys_update_log::edit(depot, request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all 获取全部
#[handler]
pub async fn get_all() -> Res<Vec<SysUpdateLogModel>> {
    match api_fn::sys_update_log::get_all().await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
