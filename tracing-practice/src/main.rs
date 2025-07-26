use tracing_practice::utils;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tracing_practice=info,tracing_practice::utils=debug".into()),
        )
        .init();

    // 不会输出
    tracing::debug!("main入口 debug");
    // 会输出
    tracing::warn!("main入口 warn");

    utils::do_something();

    test_str();
}

fn test_str() {
    let s = "hello";
    let s1 = String::from(s);

    let s2: &str = s.into();
    let s3: String = s.into();
    println!("{s} {s1} {s2} {s3}");
}
