use std::{convert::Infallible, time::Duration};

use db::{common::captcha::CaptchaImage, system::models::server_info::SysInfo};
use futures_util::StreamExt;
use salvo::{
    handler,
    sse::{self, SseEvent},
    Response,
};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

use crate::utils::res_util::Res;

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
