use std::{convert::Infallible, time::Duration};

use db::{
    common::{
        captcha::CaptchaImage,
        sse_msg::{SseQuery, SSEMSG},
    },
    system::models::server_info::SysInfo,
};
use futures_util::StreamExt;
use middleware_fn::res_util::Res;
use salvo::{
    handler,
    sse::{self, SseEvent},
    Response,
};
use salvo_core::Request;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

/// 获取验证码
#[handler]
pub fn get_captcha() -> Res<CaptchaImage> {
    Res::with_data(service::common::get_captcha())
}

///  获取服务器信息
#[handler]
pub async fn get_server_info() -> Res<SysInfo> {
    let res = service::system::server_info::get_oper_sys_info();
    Res::with_data(res)
}

///  通过SSE获取服务器信息
#[handler]
pub async fn get_server_info_sse(res: &mut Response) {
    let event_stream = {
        let interval = interval(Duration::from_secs(1));
        let stream = IntervalStream::new(interval);
        stream.map(move |_| {
            let res = service::system::server_info::get_oper_sys_info();
            sse_data(res)
        })
    };
    sse::stream(res, event_stream);
}

fn sse_data(data: SysInfo) -> Result<SseEvent, Infallible> {
    Ok(SseEvent::default().text(serde_json::to_string(&data).unwrap_or_else(|_| "".to_string())))
}
fn sse_string(data: &str) -> Result<SseEvent, Infallible> {
    Ok(SseEvent::default().text(data))
}

/// 获取 SSE 消息
#[handler]
pub async fn get_sse_msg(request: &mut Request, res: &mut Response) {
    let req = match request.parse_queries::<SseQuery>() {
        Ok(v) => v,
        Err(_e) => return,
    };
    let id = req.id;

    let event_stream = {
        let interval = interval(Duration::from_secs(1));
        let stream = IntervalStream::new(interval);
        stream.map(move |_| {
            let sse_msg = SSEMSG.lock().expect("获取map错误");
            let msg = match sse_msg.get(&id) {
                Some(msg) => msg,
                None => "-1",
            };
            sse_string(msg)
        })
    };
    sse::stream(res, event_stream);
}

#[handler]
pub async fn delete_sse_msg(request: &mut Request) -> Res<String> {
    let req = match request.parse_json::<SseQuery>().await {
        Ok(v) => v,
        Err(_) => return Res::with_msg("参数错误"),
    };
    let id = req.id;
    let mut sse_msg = SSEMSG.lock().expect("获取map错误");
    match sse_msg.remove(&id) {
        Some(_) => Res::with_msg("移除数据成功"),
        None => Res::with_msg("移除数据失败"),
    }
}
