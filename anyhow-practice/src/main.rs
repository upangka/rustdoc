mod read_file;

use anyhow::{Context, Result, anyhow};

fn main() -> Result<()> {
    println!("Hello, world!");
    // context 简单字符串
    might_fail(false).context("might_fail() failed")?;
    // with_context 动态生成上下文信息
    might_fail(false).with_context(|| "might_fail() failed")?;
    Ok(())
}

fn might_fail(succeed: bool) -> Result<()> {
    if succeed {
        Ok(())
    } else {
        Err(anyhow!("可能发生的错误"))
    }
}
