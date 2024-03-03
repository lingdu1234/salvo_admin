use anyhow::Result;
use db::{
    db_conn,
    system::{
        models::sys_api_db::{SysApiDbAddEditReq, SysApiDbSearchReq},
        prelude::SysApiDbModel,
    },
    DB,
};
use salvo::Request;

/// add 添加
pub async fn add(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysApiDbAddEditReq>().await?;
    let db = DB.get_or_init(db_conn).await;
    service::system::sys_api_db::add(db, req).await
}

/// 按id获取api数据
pub async fn get_by_id(request: &mut Request) -> Result<Vec<SysApiDbModel>> {
    let req = request.parse_queries::<SysApiDbSearchReq>()?;
    let db = DB.get_or_init(db_conn).await;
    service::system::sys_api_db::get_by_id(db, &req.api_id).await
}
