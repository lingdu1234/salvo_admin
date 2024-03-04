use configs::CFG;
use middleware_fn::{api_auth, ctx, jwt_auth, operation_log};
use salvo::{prelude::StaticDir, Router};

pub mod common;
pub mod system;

pub fn apis() -> Router {
    Router::with_path(&CFG.server.api_prefix)
        .push(
            Router::with_path(&CFG.web.upload_url).push(
                Router::with_path("<**path>").get(
                    StaticDir::new(&CFG.web.upload_dir)
                        .include_dot_files(false)
                        .auto_list(true),
                ),
            ),
        )
        .push(no_auth_api())
        .push(auth_api())
}

// 不需要授权的api
pub fn no_auth_api() -> Router {
    Router::with_path("comm")
        .push(Router::with_path("get_captcha").get(common::get_captcha))
        .push(Router::with_path("login").post(system::api::sys_user::login))
        .push(
            Router::with_path("log_out")
                .hoop(jwt_auth::jwt_auth_fn)
                .post(system::api::sys_user_online::log_out),
        )
}

// 需要授权的api
pub fn auth_api() -> Router {
    Router::with_hoop_when(operation_log::operation_log_fn, |_, _| {
        CFG.log.enable_oper_log
    })
    .hoop(jwt_auth::jwt_auth_fn) //JWT认证
    .hoop(ctx::ctx_fn) //添加请求上下文中间件
    .hoop(api_auth::api_auth_fn) //Api认证
    .push(Router::with_path("system").push(system::system_api())) //系统api
}
