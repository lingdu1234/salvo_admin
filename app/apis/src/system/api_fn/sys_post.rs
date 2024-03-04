use anyhow::Result;
use db::{
    common::res::{ListData, PageParams},
    db_conn,
    system::{
        models::sys_post::{
            SysPostAddReq, SysPostDeleteReq, SysPostEditReq, SysPostResp, SysPostSearchReq,
        },
        prelude::SysPostModel,
    },
    DB,
};
use middleware_fn::user_utils::get_current_user;
use salvo::{Depot, Request};
use service::system;

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(request: &mut Request) -> Result<ListData<SysPostModel>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysPostSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_post::get_sort_list(db, page_params, req).await
}

/// add 添加
pub async fn add(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysPostAddReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_post::add(db, req, user.user_id.clone()).await
}

/// delete 完全删除
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysPostDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_post::delete(db, req).await
}

/// edit 修改
pub async fn edit(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysPostEditReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_post::edit(db, req, user.user_id.clone()).await
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(request: &mut Request) -> Result<SysPostResp> {
    let req = request.parse_queries::<SysPostSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_post::get_by_id(db, req).await
}

/// get_all 获取全部
pub async fn get_all() -> Result<Vec<SysPostResp>> {
    let db = DB.get_or_init(db_conn).await;
    system::sys_post::get_all(db).await
}
