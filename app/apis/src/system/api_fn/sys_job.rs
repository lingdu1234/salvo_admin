use anyhow::Result;
use db::{
    common::res::{ListData, PageParams},
    db_conn,
    system::{
        models::sys_job::{
            JobId, SysJobAddReq, SysJobDeleteReq, SysJobEditReq, SysJobSearchReq, SysJobStatusReq,
            ValidateReq, ValidateRes,
        },
        SysJobModel,
    },
    DB,
};
use middleware_fn::user_utils::get_current_user;
use salvo::{Depot, Request};
use service::{system, tasks};

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(request: &mut Request) -> Result<ListData<SysJobModel>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysJobSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_job::get_sort_list(db, page_params, req).await
}
/// add 添加
pub async fn add(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysJobAddReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_job::add(db, req, user.user_id.clone()).await
}

/// delete 完全删除
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysJobDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_job::delete(db, req).await
}

/// edit 修改
pub async fn edit(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysJobEditReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_job::edit(db, req, user.user_id.clone()).await
}

/// get_user_by_id 获取用户Id获取用户

pub async fn get_by_id(request: &mut Request) -> Result<SysJobModel> {
    let req = request.parse_queries::<SysJobSearchReq>()?;
    match req.job_id {
        None => Err(anyhow::anyhow!("id不能为空")),
        Some(id) => {
            let db = DB.get_or_init(db_conn).await;
            system::sys_job::get_by_id(db, id).await
        }
    }
}

pub async fn change_status(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysJobStatusReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_job::set_status(db, req).await
}

pub async fn run_task_once(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<JobId>().await?;

    tasks::run_once_task(req.job_id, req.task_id, true).await;
    Ok("任务开始执行".to_string())
}

pub async fn validate_cron_str(request: &mut Request) -> Result<ValidateRes> {
    let req = request.parse_json::<ValidateReq>().await?;
    system::sys_job::validate_cron_str(req.cron_str)
}
