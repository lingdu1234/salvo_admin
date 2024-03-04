use db::{
    common::{jwt::AuthBody, res::ListData},
    system::models::sys_user::{UserInfo, UserInformation, UserWithDept},
};
use middleware_fn::res_util::Res;
use salvo::{handler, Depot, Request};

use crate::system::api_fn;

/// get_user_list 获取用户列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(request: &mut Request) -> Res<ListData<UserWithDept>> {
    match api_fn::sys_user::get_sort_list(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[handler]
pub async fn get_by_id(request: &mut Request) -> Res<UserInformation> {
    match api_fn::sys_user::get_by_id(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn get_profile(depot: &mut Depot) -> Res<UserInformation> {
    match api_fn::sys_user::get_profile(depot).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// add 添加
#[handler]
pub async fn add(depot: &mut Depot, request: &mut Request) -> Res<String> {
    match api_fn::sys_user::add(depot, request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(request: &mut Request) -> Res<String> {
    match api_fn::sys_user::delete(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
#[handler]
pub async fn edit(depot: &mut Depot, request: &mut Request) -> Res<String> {
    match api_fn::sys_user::edit(depot, request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn update_profile(request: &mut Request) -> Res<String> {
    match api_fn::sys_user::update_profile(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 用户登录
#[handler]
pub async fn login(request: &mut Request) -> Res<AuthBody> {
    match api_fn::sys_user::login(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
/// 获取用户登录信息
#[handler]
pub async fn get_info(depot: &mut Depot) -> Res<UserInfo> {
    match api_fn::sys_user::get_info(depot).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// edit 修改
#[handler]
pub async fn reset_passwd(request: &mut Request) -> Res<String> {
    match api_fn::sys_user::reset_passwd(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn update_passwd(depot: &mut Depot, request: &mut Request) -> Res<String> {
    match api_fn::sys_user::update_passwd(depot, request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
#[handler]
pub async fn change_status(request: &mut Request) -> Res<String> {
    match api_fn::sys_user::change_status(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
// fresh_token 刷新token
#[handler]
pub async fn fresh_token(depot: &mut Depot) -> Res<AuthBody> {
    match api_fn::sys_user::fresh_token(depot).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn change_role(request: &mut Request) -> Res<String> {
    match api_fn::sys_user::change_role(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn change_dept(request: &mut Request) -> Res<String> {
    match api_fn::sys_user::change_dept(request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn update_avatar(depot: &mut Depot, request: &mut Request) -> Res<String> {
    match api_fn::sys_user::update_avatar(depot, request).await {
        Ok(v) => Res::with_data(v),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
