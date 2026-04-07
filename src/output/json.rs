use anyhow::Result;
use serde::Serialize;

pub fn render_json<T: Serialize + ?Sized>(item: &T) -> Result<()> {
    let output = serde_json::to_string(item)?;
    println!("{}", output);
    Ok(())
}

pub fn render_json_pretty<T: Serialize + ?Sized>(item: &T) -> Result<()> {
    let output = serde_json::to_string_pretty(item)?;
    println!("{}", output);
    Ok(())
}
