use anyhow::Result;
use db::{
    db_conn,
    system::{
        models::sys_update_log::{SysUpdateLogAddReq, SysUpdateLogDeleteReq, SysUpdateLogEditReq},
        prelude::SysUpdateLogModel,
    },
    DB,
};
use middleware_fn::user_utils::get_current_user;
use salvo::{Depot, Request};
use service::system;

/// add 添加
pub async fn add(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysUpdateLogAddReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_update_log::add(db, req, &user.user_id).await
}

/// delete 完全删除
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysUpdateLogDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_update_log::soft_delete(db, &req.id).await
}

/// edit 修改
pub async fn edit(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysUpdateLogEditReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_update_log::edit(db, req, &user.user_id).await
}

/// get_all 获取全部
pub async fn get_all() -> Result<Vec<SysUpdateLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    system::sys_update_log::get_all(db).await
}
