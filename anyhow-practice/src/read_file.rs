

use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::fs::File;
use std::io::Read;

fn read_config_file(path: &str) -> Result<String> {
    let mut file = File::open(path)
        .with_context(|| format!("无法打开配置文件: {}", path))?; // 加 context，定位文件打开错误
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("读取配置文件内容失败")?; // 加 context，定位文件读取错误
    Ok(contents)
}

fn parse_config(data: &str) -> Result<Value> {
    serde_json::from_str(data)
        .context("解析配置文件失败") // 加 context，定位 JSON 解析错误
}

fn extract_api_key(json: &Value) -> Result<String> {
    json.get("api_key")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("api_key 不存在")) // 底层错误
        .context("提取 api_key 时失败") // 加 context，标明提取 api_key 出错
}

fn send_request(api_key: &str) -> Result<()> {
    // 模拟请求失败
    Err(anyhow!("服务器拒绝连接"))
        .context(format!("发送请求到服务器时失败 (api_key = {})", api_key)) // context + 动态信息
}

#[test]
fn test() -> Result<()> {
    let path = "config.json";

    let data = read_config_file(path)?;
    let json = parse_config(&data)?;
    
    println!("{:#?}",json);
    let api_key = extract_api_key(&json)?;
    send_request(&api_key)?;

    println!("请求发送成功");
    Ok(())
}
