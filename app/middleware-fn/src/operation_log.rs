use anyhow::Result;
use chrono::Local;
use db::{
    common::{client::ClientInfo, ctx::ReqCtx, jwt::Claims},
    system::models::sys_oper_log::OperationLogData,
};
use salvo::hyper::StatusCode;
use salvo_core::{
    handler,
    http::{cookie::time::Instant, Request, ResBody, Response, StatusError},
    Depot, FlowCtrl,
};
use service::{
    service_utils::{get_client_info, ApiUtils::ALL_APIS},
    system::sys_oper_log::add_to_db,
};

#[handler]
pub async fn operation_log_fn(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let start_time = Instant::now();
    // 继续执行后续任务
    ctrl.call_next(req, depot, res).await;
    //
    let duration = start_time.elapsed();
    // 获取请求上下文,必须在执行完后才能获取，否则报错
    let req_ctx = match depot.get::<ReqCtx>("req_ctx") {
        Ok(v) => v.clone(),
        Err(_) => {
            return res.render(StatusError::internal_server_error().brief("处理请求信息出错"))
        }
    };
    let user = match depot.get::<Claims>("Claims") {
        Ok(v) => v.clone(),
        Err(_) => {
            return res.render(StatusError::internal_server_error().brief("处理请求信息出错"))
        }
    };

    let res_v = match depot.get::<String>("res_v") {
        Ok(v) => v.clone(),
        Err(_) => "".to_string(),
    };
    let status = res.status_code.unwrap_or(match &res.body {
        ResBody::None => StatusCode::NOT_FOUND,
        ResBody::Error(e) => e.code,
        _ => StatusCode::OK,
    });
    // 执行结束后设置res中的数据
    // 生成一个请求id
    let req_id = scru128::new_string();
    let req_time = Local::now();
    let remote_ip = req.remote_addr().as_ipv4().unwrap().ip().to_string();
    let d = duration.whole_microseconds() as i64;
    let log = OperationLogData {
        req_id: req_id.clone(),
        req_user: user.user_id.clone(),
        req_data: get_body_data(req).await,
        req_time: req_time.format("%Y-%m-%d %H:%M:%S %.f %Z").to_string(),
        time_id: req_time.timestamp(),
        req_version: format!("{:?}", req.version()),
        req_ip: remote_ip.clone(),
        req_method: req_ctx.method.clone(),
        req_ori_url: req_ctx.ori_uri.clone(),
        req_url: req_ctx.path.clone(),
        res_status: format!("{}", status),
        res_body: res_v,
        res_time: Local::now().format("%Y-%m-%d %H:%M:%S %.f %Z").to_string(),
        elapsed_time: format!("{:?}μs", d),
    };
    let headers = req.headers().clone();
    // 最后单开一个线程打印处理日志
    tokio::spawn(async move {
        let uc = get_client_info(&headers, remote_ip).await;
        match add_operation_log(&user, &req_ctx, &uc, log, d).await {
            Ok(v) => tracing::info!(v),
            Err(e) => tracing::info!("日志添加失败：{}", e.to_string()),
        };
    });
}

/// 获取请求的json数据
async fn get_body_data(req: &mut Request) -> String {
    let v = match req.payload().await {
        Ok(v) => v.clone(),
        Err(_) => "".into(),
    };
    match std::str::from_utf8(&v) {
        Ok(x) => x.to_string(),
        Err(_) => "".to_string(),
    }
}

/// 添加日志
async fn add_operation_log(
    user: &Claims,
    req_ctx: &ReqCtx,
    uc: &ClientInfo,
    log: OperationLogData,
    d: i64,
) -> Result<String> {
    let apis = ALL_APIS.lock().await;

    let (api_name, is_log) = match apis.get(&req_ctx.path) {
        Some(x) => (x.name.clone(), x.log_method.clone()),
        None => ("".to_string(), "0".to_string()),
    };
    drop(apis);
    //     开始添加日志
    match is_log.as_str() {
        "1" => add_file_log(&log),
        "2" => add_to_db(user, req_ctx, uc, log, api_name, d).await,
        "3" => {
            add_file_log(&log).expect("日志写入失败");
            add_to_db(user, req_ctx, uc, log, api_name, d).await
        }
        _ => Ok("无需添加日志".to_string()),
    }
}

fn add_file_log(log: &OperationLogData) -> Result<String> {
    tracing::info!("{:#?}", log);
    Ok("".to_string())
}
