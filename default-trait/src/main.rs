

#[derive(Debug)]
struct ServerConfig{
    host: String,
    port: u16,
    use_tls: bool,
    max_connections: usize
}

impl Default for ServerConfig{
    fn default() -> Self {
        ServerConfig{
            host: "localhost".to_string(),
            port: 8080,
            use_tls: false,
            max_connections: 1024
        }
    }
}


// 提供一个 Option<ServerConfig> 的处理函数
fn get_config(value: Option<ServerConfig>) -> ServerConfig {
    value.unwrap_or_default()
}

fn main() {
    
    // 默认配置
    let default_cfg = ServerConfig::default();
    println!("默认配置：{default_cfg:#?}");

    let custom_cfg = ServerConfig{
        port: 5173,
        ..ServerConfig::default()
    };

    println!("自定义配置: {custom_cfg:#?}");

    // 结合 Option
    let cfg = get_config(None);
    println!("处理后的配置: {cfg:#?}");
}
