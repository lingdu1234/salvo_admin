use anyhow::Result;
use configs::CFG;
use db::{
    common::res::{ListData, PageParams},
    db_conn,
    system::{
        models::sys_menu::{
            LogCacheEditReq, MenuRelated, MenuResp, SysMenuAddReq, SysMenuDeleteReq,
            SysMenuEditReq, SysMenuSearchReq, SysMenuTree, SysMenuTreeAll,
        },
        prelude::SysMenuModel,
    },
    DB,
};
use middleware_fn::user_utils::get_current_user;
use salvo::{Depot, Request};
use service::system;

/// get_all_menu_tree 获取全部菜单
pub async fn get_sort_list(request: &mut Request) -> Result<ListData<SysMenuModel>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysMenuSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_menu::get_sort_list(db, page_params, req).await
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0
pub async fn get_by_id(request: &mut Request) -> Result<MenuResp> {
    let req = request.parse_queries::<SysMenuSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_menu::get_by_id(db, req).await
}

/// add 添加
pub async fn add(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysMenuAddReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_menu::add(db, req).await
}

/// delete 完全删除
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysMenuDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_menu::delete(db, &req.id).await
}

/// edit 修改
pub async fn edit(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysMenuEditReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_menu::edit(db, req).await
}

/// update_log_cache_method 修改菜单日志缓存方法
pub async fn update_log_cache_method(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<LogCacheEditReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_menu::update_log_cache_method(db, req).await
}

/// get_all_menu_tree 获取全部菜单树
pub async fn get_all_enabled_menu_tree(request: &mut Request) -> Result<Vec<SysMenuTreeAll>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysMenuSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_menu::get_all_enabled_menu_tree(db, page_params, req).await
}

/// get_related_api_and_db 获取全部菜单树
pub async fn get_related_api_and_db(request: &mut Request) -> Result<ListData<MenuRelated>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysMenuSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_menu::get_related_api_and_db(db, page_params, req).await
}

/// 获取用户路由

pub async fn get_routers(depot: &mut Depot) -> Result<Vec<SysMenuTree>> {
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    //  获取 用户角色
    let role_id = match system::sys_role::get_current_admin_role(db, &user.user_id).await {
        Ok(x) => x,
        Err(e) => return Err(anyhow::anyhow!(e.to_string())),
    };

    // 检查是否超管用户
    if CFG.system.super_user.contains(&user.user_id) {
        system::sys_menu::get_all_router_tree(db).await
    } else {
        system::sys_menu::get_admin_menu_by_role_ids(db, &role_id).await
    }
}
