use tracing_practice::utils;
use tracing_subscriber::EnvFilter;

fn main() {
    // 初始化全局日志订阅者
    // tracing_subscriber::fmt()
    //     .with_env_filter(EnvFilter::from_default_env()) // 支持通过环境变量设置日志级别
    //     .init();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tracing_practice=info,tracing_practice::utils=debug".into()),
        )
        .init();

    // 不会输出
    tracing::debug!("main入口 debug");
    // 会输出
    tracing::warn!("main入口 warn");

    utils::do_something();
}
