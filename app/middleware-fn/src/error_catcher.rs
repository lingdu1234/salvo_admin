/// 将错误转换为200返回，方便数据统处理
use db::common::res::ErrorResp;
use salvo::{handler, writing::Json};
use salvo_core::{http::ResBody, hyper::StatusCode, Depot, FlowCtrl, Request, Response};

#[handler]
pub async fn error_catcher_fn(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    ctrl.call_next(req, depot, res).await;
    // 这里会出现status_code为None的情况
    if !(res.status_code == Some(StatusCode::OK) || res.status_code.is_none()) {
        let err_resp = match &res.body {
            ResBody::Error(err) => {
                let cause = err
                    .cause
                    .as_ref()
                    .map(|e| format!("{:#?}", e.as_ref()))
                    .unwrap_or_default();
                let code = err.code;
                let name = err.name.clone();
                let detail = err.detail.clone().unwrap_or_default();
                let brief = err.brief.clone();
                ErrorResp {
                    ____code: code.into(),
                    name,
                    brief,
                    detail,
                    cause,
                }
            }
            _ => ErrorResp {
                ..Default::default()
            },
        };
        res.status_code = Some(StatusCode::OK);
        res.render(Json(err_resp));
        ctrl.call_next(req, depot, res).await;
    }
}
