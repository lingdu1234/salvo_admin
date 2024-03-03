use db::common::jwt::Claims;
use salvo_core::{
    handler,
    http::{Request, Response, StatusError},
    Depot, FlowCtrl,
};
use service::{service_utils::jwt::KEYS, system::check_user_online};

#[handler]
pub async fn jwt_auth_fn(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    if let Some((token_id, token_v)) = find_token(req).await {
        // 第一步对token进行解码
        if let Ok(payload) = jsonwebtoken::decode::<Claims>(
            &token_v,
            &KEYS.decoding,
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
        ) {
            // 第二步对payload进行验证
            if payload.claims.token_id == token_id {
                // 第三部验证是否在线
                let (is_online, _) = check_user_online(None, token_id.clone()).await;
                if is_online {
                    // 这里将数据写入depot
                    let user_ctx = Claims {
                        user_id: payload.claims.user_id,
                        token_id: payload.claims.token_id,
                        name: payload.claims.name,
                        exp: payload.claims.exp,
                    };
                    depot.insert("Claims", user_ctx);

                    ctrl.call_next(req, depot, res).await;
                } else {
                    // 用户不在线,说明已经失效，或者被后台强制踢下线
                    res.render(StatusError::unauthorized().brief("token is expired or revoked"));
                    ctrl.skip_rest();
                }
            } else {
                res.render(StatusError::unauthorized().brief("token may be modified"));
                ctrl.skip_rest();
            }
        } else {
            res.render(StatusError::unauthorized().brief("decode token error"));
            ctrl.skip_rest();
        }
    } else {
        res.render(StatusError::unauthorized().brief("missing token,token not found"));
        ctrl.skip_rest();
    }
}

async fn find_token(req: &mut Request) -> Option<(String, String)> {
    if let Some(Ok(auth)) = req.headers().get("authorization").map(|auth| auth.to_str()) {
        if auth.starts_with("Bearer") {
            let token_and_token_id = match auth.split_once(' ').map(|(_, token)| token.to_string())
            {
                Some(token) => token,
                None => return None,
            };
            let cut = token_and_token_id.len() - scru128::new_string().len();
            let t_v = token_and_token_id[0..cut].to_string();
            let t_id = token_and_token_id[cut..].to_string();
            Some((t_id, t_v))
        } else {
            None
        }
    } else {
        None
    }
}
