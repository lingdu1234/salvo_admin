use anyhow::Result;
use db::{
    common::res::{ListData, PageParams},
    db_conn,
    system::{
        models::sys_job_log::{SysJobLogCleanReq, SysJobLogDeleteReq, SysJobLogSearchReq},
        prelude::SysJobLogModel,
    },
    DB,
};
use salvo::Request;
use service::system;

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(request: &mut Request) -> Result<ListData<SysJobLogModel>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysJobLogSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_job_log::get_sort_list(db, page_params, req).await
}

/// delete 完全删除
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysJobLogDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_job_log::delete(db, req).await
}

/// 清空
pub async fn clean(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysJobLogCleanReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_job_log::clean(db, req.job_id).await
}
