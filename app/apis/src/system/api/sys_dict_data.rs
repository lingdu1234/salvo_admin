use db::{common::res::ListData, system::prelude::SysDictDataModel};
use salvo::{handler, Depot, Request};

use crate::{system::api_fn, utils::res_util::Res};

/// get_list 获取列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(request: &mut Request) -> Res<ListData<SysDictDataModel>> {
    match api_fn::sys_dict_data::get_sort_list(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// add 添加
#[handler]
pub async fn add(depot: &mut Depot, request: &mut Request) -> Res<String> {
    match api_fn::sys_dict_data::add(depot, request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(request: &mut Request) -> Res<String> {
    match api_fn::sys_dict_data::delete(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改

#[handler]
pub async fn edit(depot: &mut Depot, request: &mut Request) -> Res<String> {
    match api_fn::sys_dict_data::edit(depot, request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[handler]
pub async fn get_by_id(request: &mut Request) -> Res<SysDictDataModel> {
    match api_fn::sys_dict_data::get_by_id(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[handler]
pub async fn get_by_type(request: &mut Request) -> Res<Vec<SysDictDataModel>> {
    match api_fn::sys_dict_data::get_by_type(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all 获取全部
#[handler]
pub async fn get_all() -> Res<Vec<SysDictDataModel>> {
    match api_fn::sys_dict_data::get_all().await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
