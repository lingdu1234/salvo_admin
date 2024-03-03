use db::{common::res::ListData, system::prelude::SysJobLogModel};
use salvo::{handler, Request};

use crate::{system::api_fn, utils::res_util::Res};

/// get_list 获取列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(request: &mut Request) -> Res<ListData<SysJobLogModel>> {
    match api_fn::sys_job_log::get_sort_list(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(request: &mut Request) -> Res<String> {
    match api_fn::sys_job_log::delete(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 清空
#[handler]
pub async fn clean(request: &mut Request) -> Res<String> {
    match api_fn::sys_job_log::clean(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
