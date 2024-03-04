use anyhow::Result;
use db::{
    common::res::{ListData, PageParams},
    db_conn,
    system::{
        models::sys_role::{
            DataScopeReq, SysRoleAddReq, SysRoleDeleteReq, SysRoleEditReq, SysRoleResp,
            SysRoleSearchReq, SysRoleStatusReq,
        },
        prelude::SysRoleModel,
    },
    DB,
};
use middleware_fn::user_utils::get_current_user;
use salvo::{Depot, Request};
use service::system;

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(request: &mut Request) -> Result<ListData<SysRoleModel>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysRoleSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_role::get_sort_list(db, page_params, req).await
}

/// add 添加
pub async fn add(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysRoleAddReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_role::add(db, req, &user.user_id).await
}

/// delete 完全删除
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysRoleDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_role::delete(db, req).await
}

/// edit 修改
pub async fn edit(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysRoleEditReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_role::edit(db, req, &user.user_id).await
}

/// set_status 修改状态
pub async fn change_status(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysRoleStatusReq>().await?;

    let db = DB.get_or_init(db_conn).await;

    system::sys_role::set_status(db, req).await
}
/// set_data_scope 修改数据权限范围
pub async fn set_data_scope(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<DataScopeReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_role::set_data_scope(db, req).await
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(request: &mut Request) -> Result<SysRoleResp> {
    let req = request.parse_queries::<SysRoleSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_role::get_by_id(db, req).await
}

/// get_all 获取全部
pub async fn get_all() -> Result<Vec<SysRoleResp>> {
    let db = DB.get_or_init(db_conn).await;
    system::sys_role::get_all(db).await
}

/// get_role_menu 获取角色授权菜单id数组
pub async fn get_role_menu(request: &mut Request) -> Result<Vec<String>> {
    let req = request.parse_queries::<SysRoleSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    match req.role_id {
        None => Err(anyhow::anyhow!("role_id不能为空")),
        Some(id) => match system::sys_menu::get_role_permissions(db, &id).await {
            Ok((_, x)) => Ok(x),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        },
    }
}

/// get_role_dept 获取角色授权部门id数组
pub async fn get_role_dept(request: &mut Request) -> Result<Vec<String>> {
    let req = request.parse_queries::<SysRoleSearchReq>()?;
    match req.role_id {
        None => Err(anyhow::anyhow!("role_id不能为空")),
        Some(id) => {
            let db = DB.get_or_init(db_conn).await;
            system::sys_dept::get_dept_by_role_id(db, id).await
        }
    }
}

// pub async fn get_auth_users_by_role_id(Query(mut req): Query<UserSearchReq>,
// Query(page_params): Query<PageParams>) -> Res<ListData<UserWithDept>> {
//     let db = DB.get_or_init(db_conn).await;
//     let role_id = match req.role_id.clone() {
//         None => return Res::with_err("角色Id不能为空"),
//         Some(id) => id,
//     };
//     let user_ids = match system::sys_role::get_auth_users_by_role_id(db,
// &role_id).await {         Ok(x) => x,
//         Err(e) => return Res::with_err(&e.to_string()),
//     };
//     req.user_ids = Some(user_ids.join(","));
//     let res = system::sys_user::get_sort_list(db, page_params, req).await;
//     match res {
//         Ok(x) => Res::with_data(x),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }

// pub async fn get_un_auth_users_by_role_id(Query(mut req):
// Query<UserSearchReq>, Query(page_params): Query<PageParams>) ->
// Res<ListData<UserResp>> {     let db = DB.get_or_init(db_conn).await;
//     let role_id = match req.role_id.clone() {
//         None => return Res::with_err("角色Id不能为空"),
//         Some(id) => id,
//     };
//     let user_ids = match system::sys_role::get_auth_users_by_role_id(db,
// &role_id).await {         Ok(x) => x,
//         Err(e) => return Res::with_err(&e.to_string()),
//     };
//     req.user_ids = Some(user_ids.join(","));
//     let res = system::sys_user::get_un_auth_user(db, page_params, req).await;
//     match res {
//         Ok(x) => Res::with_data(x),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }

// // edit 修改

// pub async fn update_auth_role(user: Claims, Json(req):
// Json<UpdateAuthRoleReq>) -> Res<String> {     let db =
// DB.get_or_init(db_conn).await;
//     match system::sys_role::add_role_by_user_id(db, &req.user_id,
// req.role_ids, user.id).await {         Ok(_) =>
// Res::with_msg("角色授权更新成功"),         Err(e) =>
// Res::with_err(&e.to_string()),     }
// }

// pub async fn add_auth_user(user: Claims, Json(req):
// Json<AddOrCancelAuthRoleReq>) -> Res<String> {     let db =
// DB.get_or_init(db_conn).await;     let res =
// system::sys_role::add_role_with_user_ids(db, req.clone().user_ids,
// req.role_id, user.id).await;     match res {
//         Ok(_) => Res::with_msg("授权成功"),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }

// pub async fn cancel_auth_user(Json(req): Json<AddOrCancelAuthRoleReq>) ->
// Res<String> {     let db = DB.get_or_init(db_conn).await;
//     let res = system::sys_role::cancel_auth_user(db, req).await;
//     match res {
//         Ok(_) => Res::with_msg("取消授权成功"),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }
