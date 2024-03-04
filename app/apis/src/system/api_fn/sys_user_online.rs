use anyhow::Result;
use db::{
    common::res::{ListData, PageParams},
    db_conn,
    system::{
        models::sys_user_online::{SysUserOnlineDeleteReq, SysUserOnlineSearchReq},
        prelude::SysUserOnlineModel,
    },
    DB,
};
use middleware_fn::user_utils::get_current_user;
use salvo::{Depot, Request};
use service::system;

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(request: &mut Request) -> Result<ListData<SysUserOnlineModel>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysUserOnlineSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user_online::get_sort_list(db, page_params, req).await
}

/// 删除
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysUserOnlineDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user_online::delete(db, req).await
}

/// 登出

pub async fn log_out(depot: &mut Depot) -> Result<String> {
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user_online::log_out(db, user.token_id.clone()).await
}
