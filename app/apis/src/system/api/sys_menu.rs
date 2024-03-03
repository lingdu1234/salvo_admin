use db::{
    common::res::ListData,
    system::{
        models::sys_menu::{MenuRelated, MenuResp, SysMenuTree, SysMenuTreeAll},
        prelude::SysMenuModel,
    },
};
use salvo::{handler, Depot, Request};

use crate::{system::api_fn, utils::res_util::Res};
/// get_all_menu_tree 获取全部菜单
#[handler]
pub async fn get_sort_list(request: &mut Request) -> Res<ListData<SysMenuModel>> {
    match api_fn::sys_menu::get_sort_list(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[handler]
pub async fn get_by_id(request: &mut Request) -> Res<MenuResp> {
    match api_fn::sys_menu::get_by_id(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// add 添加
#[handler]
pub async fn add(request: &mut Request) -> Res<String> {
    match api_fn::sys_menu::add(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(request: &mut Request) -> Res<String> {
    match api_fn::sys_menu::delete(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
#[handler]
pub async fn edit(request: &mut Request) -> Res<String> {
    match api_fn::sys_menu::edit(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
// update_log_cache_method 修改菜单日志缓存方法
#[handler]
pub async fn update_log_cache_method(request: &mut Request) -> Res<String> {
    match api_fn::sys_menu::update_log_cache_method(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all_menu_tree 获取全部菜单树
#[handler]
pub async fn get_all_enabled_menu_tree(request: &mut Request) -> Res<Vec<SysMenuTreeAll>> {
    match api_fn::sys_menu::get_all_enabled_menu_tree(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_related_api_and_db 获取全部菜单树
#[handler]
pub async fn get_related_api_and_db(request: &mut Request) -> Res<ListData<MenuRelated>> {
    match api_fn::sys_menu::get_related_api_and_db(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 获取用户路由
#[handler]
pub async fn get_routers(depot: &mut Depot) -> Res<Vec<SysMenuTree>> {
    match api_fn::sys_menu::get_routers(depot).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
