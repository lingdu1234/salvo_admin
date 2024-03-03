use configs::CFG;
use db::common::ctx::ReqCtx;
use salvo_core::{
    handler,
    http::{Request, Response},
    Depot, FlowCtrl,
};

#[handler]
pub async fn ctx_fn(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    // 获取原始路径
    let req_path_origin = req.uri().path().to_string();
    let req_path = req_path_origin.replacen(&(CFG.server.api_prefix.clone() + "/"), "", 1);
    // 获取请求方法
    let req_method = req.method().to_string();
    // 获取路径参数
    let path_params = req.uri().query().unwrap_or("").to_string();
    //     储存ctx
    let req_ctx = ReqCtx {
        ori_uri: if path_params.is_empty() {
            req_path_origin
        } else {
            req_path_origin + "?" + &path_params
        },
        path: req_path,
        path_params,
        method: req_method,
    };
    depot.insert("req_ctx", req_ctx);
    ctrl.call_next(req, depot, res).await;
}
