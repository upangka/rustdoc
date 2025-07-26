
use anyhow::{anyhow,Context,Result};

fn main() -> Result<()>{
    println!("Hello, world!");
     // might_fail(false).with_context(|| "might_fail() failed")?;
     might_fail(false).context("might_fail() failed")?;
    Ok(())
}


fn might_fail(succeed: bool) -> Result<()> {
    if succeed{
        Ok(())
    }else{
        Err(anyhow!("可能发生的错误"))
    }
}