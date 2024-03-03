use anyhow::Result;
use db::{
    common::res::{ListData, PageParams},
    db_conn,
    system::{
        models::sys_login_log::{SysLoginLogDeleteReq, SysLoginLogSearchReq},
        prelude::SysLoginLogModel,
    },
    DB,
};
use salvo::Request;
use service::system;

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(request: &mut Request) -> Result<ListData<SysLoginLogModel>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysLoginLogSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_login_log::get_sort_list(db, page_params, req).await
}

/// 删除登录日志
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysLoginLogDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_login_log::delete(db, req).await
}

/// 清楚登录日志

pub async fn clean() -> Result<String> {
    let db = DB.get_or_init(db_conn).await;
    system::sys_login_log::clean(db).await
}
