use anyhow::Result;
use db::{
    common::res::{ListData, PageParams},
    db_conn,
    system::{
        models::sys_dept::{
            DeptResp, RespTree, SysDeptAddReq, SysDeptDeleteReq, SysDeptEditReq, SysDeptSearchReq,
        },
        prelude::SysDeptModel,
    },
    DB,
};
use salvo::{Depot, Request};
use service::system;

use crate::utils::user_utils::get_current_user;

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(request: &mut Request) -> Result<ListData<SysDeptModel>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysDeptSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_dept::get_sort_list(db, page_params, req).await
}

/// add 添加

pub async fn add(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysDeptAddReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_dept::add(db, req, user.user_id.clone()).await
}

/// delete 完全删除

pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysDeptDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_dept::delete(db, req).await
}

// edit 修改

pub async fn edit(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysDeptEditReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_dept::edit(db, req, user.clone().user_id).await
}

/// get_user_by_id 获取用户Id获取用户

pub async fn get_by_id(request: &mut Request) -> Result<DeptResp> {
    let req = request.parse_queries::<SysDeptSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;

    if let Some(x) = req.dept_id {
        system::sys_dept::get_by_id(db, &x).await
    } else {
        Err(anyhow::anyhow!("参数错误"))
    }
}

/// get_all 获取全部

pub async fn get_all() -> Result<Vec<DeptResp>> {
    let db = DB.get_or_init(db_conn).await;
    system::sys_dept::get_all(db).await
}

pub async fn get_dept_tree() -> Result<Vec<RespTree>> {
    let db = DB.get_or_init(db_conn).await;
    system::sys_dept::get_dept_tree(db).await
}
