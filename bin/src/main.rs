use std::{fs::File, io::Read};

use configs::CFG;
use salvo::{
    compression::Compression,
    conn::rustls::{Keycert, RustlsConfig},
    cors::{Any, Cors},
    hyper::Method,
    prelude::*,
    serve_static::StaticDir,
    server::ServerHandle,
};
use service::service_utils::web_utils::set_xdb;
use tokio::signal;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
use utils::{
    log_setting::{set_log_format, set_log_level},
    my_env::RT,
};

fn main() {
    RT.block_on(async { app_run().await })
}

async fn app_run() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", &CFG.log.log_level);
    }
    // 文件日志
    let log_format = set_log_format();
    let file_appender = tracing_appender::rolling::hourly(&CFG.log.dir, &CFG.log.file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // 控制台
    let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    // 设置日志
    let logger = Registry::default()
        .with(EnvFilter::from_default_env().add_directive(set_log_level().into()))
        .with(
            fmt::Layer::default()
                .with_writer(std_non_blocking)
                .event_format(log_format.clone()),
        )
        .with(
            fmt::Layer::default()
                .with_writer(non_blocking)
                .event_format(log_format),
        );

    tracing::subscriber::set_global_default(logger).unwrap();

    let cors_handler = Cors::new()
        .allow_origin(Any)
        .allow_methods(vec![
            Method::DELETE,
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
        .max_age(3600)
        .into_handler();

    let router = Router::new()
        .hoop(Compression::new())
        .push(
            apis::apis(), // .hoop(error_catcher_fn),
        )
        .push(
            Router::with_path("<**path>").get(
                StaticDir::new(&CFG.web.dir)
                    .include_dot_files(false)
                    .defaults("index.html"),
            ),
        );

    let service = Service::new(router).hoop(Logger::new()).hoop(cors_handler);

    // 初始化所有api
    // apis全局初始化
    service::service_utils::ApiUtils::init_all_api().await;
    // 定时任务初始化
    service::tasks::timer_task_init()
        .await
        .expect("定时任务初始化失败");
    // ip 查询数据库初始化
    set_xdb();

    // let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);
    // let router = router
    //     .unshift(doc.into_router("/api-doc/openapi.json"))
    //     .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("doc"));

    match CFG.server.ssl {
        true => {
            // 证书配置
            let tls_config = RustlsConfig::new(
                Keycert::new()
                    .cert(get_tls_config(&CFG.cert.cert))
                    .key(get_tls_config(&CFG.cert.key)),
            );
            let listener = TcpListener::new(&CFG.server.address).rustls(tls_config.clone());

            let acceptor = QuinnListener::new(tls_config, &CFG.server.address)
                .join(listener)
                .bind()
                .await;
            let server = Server::new(acceptor);
            let handle = server.handle();
            tokio::spawn(listen_shutdown_signal(handle));
            server.serve(service).await;
        }
        false => {
            let acceptor = TcpListener::new(&CFG.server.address).bind().await;
            let server = Server::new(acceptor);
            let handle = server.handle();
            tokio::spawn(listen_shutdown_signal(handle));
            server.serve(service).await;
        }
    }
}

// 读取证书文件到Vec<u8>
fn get_tls_config(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect("failed to open cert file,please check path");
    let mut result = Vec::new();
    f.read_to_end(&mut result)
        .expect("failed to read cert file");
    result
}

async fn listen_shutdown_signal(handle: ServerHandle) {
    // Wait Shutdown Signal
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    tokio::select! {
        _ = ctrl_c => println!("ctrl_c signal received，is closing server"),
    };

    // Graceful Shutdown Server
    handle.stop_graceful(None);
}
