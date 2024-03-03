use anyhow::Result;
use db::{
    common::{
        jwt::AuthBody,
        res::{ListData, PageParams},
    },
    db_conn,
    system::models::sys_user::{
        ChangeDeptReq, ChangeRoleReq, ChangeStatusReq, ResetPwdReq, SysUserAddReq,
        SysUserDeleteReq, SysUserEditReq, SysUserSearchReq, UpdateProfileReq, UpdatePwdReq,
        UserInfo, UserInformation, UserLoginReq, UserWithDept,
    },
    DB,
};
use salvo::{Depot, Request};
use service::{common, system};
use tokio::join;

use crate::utils::user_utils::get_current_user;

/// get_user_list 获取用户列表
/// page_params 分页参数
pub async fn get_sort_list(request: &mut Request) -> Result<ListData<UserWithDept>> {
    let page_params = request.parse_queries::<PageParams>()?;
    let req = request.parse_queries::<SysUserSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::get_sort_list(db, page_params, req).await
}
/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(request: &mut Request) -> Result<UserInformation> {
    let req = request.parse_queries::<SysUserSearchReq>()?;

    let db = DB.get_or_init(db_conn).await;
    match req.user_id {
        Some(user_id) => match system::sys_user::get_user_info_by_id(db, &user_id).await {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        },
        None => Err(anyhow::anyhow!("用户id不能为空")),
    }
}

pub async fn get_profile(depot: &mut Depot) -> Result<UserInformation> {
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    match system::sys_user::get_user_info_by_id(db, &user.user_id).await {
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
        Ok(res) => Ok(res),
    }
}

/// add 添加
pub async fn add(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysUserAddReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::add(db, req, user.user_id.clone()).await
}

/// delete 完全删除
pub async fn delete(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysUserDeleteReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::delete(db, req).await
}

/// edit 修改
pub async fn edit(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<SysUserEditReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::edit(db, req, user.user_id.clone()).await
}

pub async fn update_profile(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<UpdateProfileReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::update_profile(db, req).await
}

/// 用户登录
pub async fn login(request: &mut Request) -> Result<AuthBody> {
    let req = request.parse_json::<UserLoginReq>().await?;
    let header = request.headers();

    let remote_ip = request.remote_addr().as_ipv4().unwrap().ip().to_string();

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::login(db, req, header, remote_ip).await
}
/// 获取用户登录信息

pub async fn get_info(depot: &mut Depot) -> Result<UserInfo> {
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;

    let (role_ids_r, dept_ids_r, user_r) = join!(
        system::sys_user_role::get_role_ids_by_user_id(db, &user.user_id),
        system::sys_user_dept::get_dept_ids_by_user_id(db, &user.user_id),
        system::sys_user::get_user_info_permission(db, &user.user_id),
    );

    let roles = match role_ids_r {
        Ok(x) => x,
        Err(e) => return Err(anyhow::anyhow!(e.to_string())),
    };
    let depts = match dept_ids_r {
        Ok(x) => x,
        Err(e) => return Err(anyhow::anyhow!(e.to_string())),
    };
    let (user, permissions) = match user_r {
        Ok((x, y)) => (x, y),
        Err(e) => return Err(anyhow::anyhow!(e.to_string())),
    };

    Ok(UserInfo {
        user,
        roles,
        depts,
        permissions,
    })
}

/// edit 修改
pub async fn reset_passwd(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<ResetPwdReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::reset_passwd(db, req).await
}

pub async fn update_passwd(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let req = request.parse_json::<UpdatePwdReq>().await?;
    let user = get_current_user(depot)?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::update_passwd(db, req, &user.user_id).await
}

/// edit 修改
pub async fn change_status(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<ChangeStatusReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::change_status(db, req).await
}
// fresh_token 刷新token

pub async fn fresh_token(depot: &mut Depot) -> Result<AuthBody> {
    let user = get_current_user(depot)?;
    system::sys_user::fresh_token(user).await
}

pub async fn change_role(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<ChangeRoleReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::change_role(db, req).await
}

pub async fn change_dept(request: &mut Request) -> Result<String> {
    let req = request.parse_json::<ChangeDeptReq>().await?;

    let db = DB.get_or_init(db_conn).await;
    system::sys_user::change_dept(db, req).await
}

pub async fn update_avatar(depot: &mut Depot, request: &mut Request) -> Result<String> {
    let file = match request.file("avatarfile").await {
        None => return Err(anyhow::anyhow!("上传的文件不存在")),
        Some(v) => v,
    };

    let user = get_current_user(depot)?;

    let img_url = common::upload_img(file).await?;

    let db = DB.get_or_init(db_conn).await;
    let _ = system::sys_user::update_avatar(db, &img_url, &user.user_id).await?;
    Ok(img_url)
}
