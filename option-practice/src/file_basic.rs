use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::Read;

fn read_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename).map_err(|e| anyhow!("打开文件失败: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| anyhow!("读取文件失败: {}", e))?;
    Ok(contents)
}

#[test]
fn test() -> Result<()> {
    read_file("xx")?;
    Ok(())
}
