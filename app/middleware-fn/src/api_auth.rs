use configs::CFG;
use db::common::{ctx::ReqCtx, jwt::Claims, res::ResErr};
use salvo_core::{
    handler,
    http::{Request, Response, StatusError},
    prelude::Json,
    Depot, FlowCtrl,
};
use service::service_utils::ApiUtils;
use tracing::info;

#[handler]
pub async fn api_auth_fn(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // 获取请求人的id
    if let Ok(user) = depot.get::<Claims>("Claims") {
        if CFG.system.super_user.contains(&user.user_id) {
            // 如果是超级用户，则不需要验证权限，直接放行
            ctrl.call_next(req, depot, res).await;
        } else {
            // 否则需要验证API权限
            let req_ctx = match depot.get::<ReqCtx>("req_ctx") {
                Ok(v) => v,
                Err(_) => {
                    return res
                        .render(StatusError::internal_server_error().brief("处理请求信息出错"))
                }
            };
            if ApiUtils::is_in(&req_ctx.path).await {
                if ApiUtils::check_api_permission(&req_ctx.path, &req_ctx.method, &user.user_id)
                    .await
                {
                    ctrl.call_next(req, depot, res).await;
                } else {
                    info!("{:?}的{:?}权限验证未通过", &user.name, &req_ctx.path);
                    let data = ResErr {
                        data: None,
                        code: Some(500),
                        msg: Some("你的该操作未授权".to_string()),
                    };
                    depot.insert("res_v", serde_json::to_string(&data).unwrap_or_default());
                    res.render(Json(data));
                    ctrl.skip_rest();
                }
            } else {
                // 该api没有在权限库，默认通过
                info!("{:?}没有在权限库默认通过", &req_ctx.path);
                ctrl.call_next(req, depot, res).await;
            }
        }
    } else {
        // 验证失败,一般不大可能走到这里
        res.render(
            StatusError::unauthorized()
                .brief("you need to login first or the api permission is set wrong"),
        );
        ctrl.skip_rest();
    }
}
