use anyhow::Result;
use db::{
    common::res::{ListData, PageParams},
    db_conn,
    system::{
        models::sys_dict_type::{
            SysDictTypeAddReq, SysDictTypeDeleteReq, SysDictTypeEditReq, SysDictTypeSearchReq,
        },
        prelude::SysDictTypeModel,
    },
    DB,
};
use salvo::{Depot, Request};
use service::system;

use crate::utils::user_utils::get_current_user;

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(request: &mut Request) -> Result<ListData<SysDictTypeModel>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysDictTypeSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_dict_type::get_sort_list(db, page_params, req).await
}

/// add 添加
pub async fn add(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysDictTypeAddReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_dict_type::add(db, req, user.user_id.clone()).await
}

/// delete 完全删除
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysDictTypeDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_dict_type::delete(db, req).await
}

/// edit 修改
pub async fn edit(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysDictTypeEditReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_dict_type::edit(db, req, user.user_id.clone()).await
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(request: &mut Request) -> Result<SysDictTypeModel> {
    let req = request.parse_queries::<SysDictTypeSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_dict_type::get_by_id(db, req).await
}

/// 获取全部
pub async fn get_all() -> Result<Vec<SysDictTypeModel>> {
    let db = DB.get_or_init(db_conn).await;
    system::sys_dict_type::get_all(db).await
}
