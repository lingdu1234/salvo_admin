use db::system::prelude::SysApiDbModel;
use salvo::{handler, Request};

use crate::{system::api_fn, utils::res_util::Res};

/// add 添加
#[handler]
pub async fn add(request: &mut Request) -> Res<String> {
    match api_fn::sys_api_db::add(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 按id获取api数据
#[handler]
pub async fn get_by_id(request: &mut Request) -> Res<Vec<SysApiDbModel>> {
    match api_fn::sys_api_db::get_by_id(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
