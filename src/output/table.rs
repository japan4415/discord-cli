use anyhow::Result;
use tabled::{Table, Tabled};

pub fn render_table<T: Tabled>(items: &[T]) -> Result<()> {
    let table = Table::new(items).to_string();
    println!("{}", table);
    Ok(())
}
