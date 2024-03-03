use anyhow::Result;
use db::{
    common::res::{ListData, PageParams},
    db_conn,
    system::{
        models::sys_oper_log::{SysOperLogDeleteReq, SysOperLogSearchReq},
        prelude::SysOperLogModel,
    },
    DB,
};
use salvo::Request;
use service::system;

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(request: &mut Request) -> Result<ListData<SysOperLogModel>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysOperLogSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_oper_log::get_sort_list(db, page_params, req).await
}

/// delete 完全删除
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysOperLogDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;

    system::sys_oper_log::delete(db, req).await
}

pub async fn clean() -> Result<String> {
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    system::sys_oper_log::clean(db).await
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(request: &mut Request) -> Result<SysOperLogModel> {
    let req = request.parse_queries::<SysOperLogSearchReq>()?;

    match req.oper_id {
        None => Err(anyhow::anyhow!("id不能为空")),
        Some(id) => {
            let db = DB.get_or_init(db_conn).await;
            system::sys_oper_log::get_by_id(db, id).await
        }
    }
}
