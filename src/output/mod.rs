pub mod json;
pub mod table;

use anyhow::Result;
use serde::Serialize;
use tabled::Tabled;

use crate::cli::OutputFormatArg;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    JsonPretty,
    Table,
}

impl From<OutputFormatArg> for OutputFormat {
    fn from(arg: OutputFormatArg) -> Self {
        match arg {
            OutputFormatArg::Json => OutputFormat::Json,
            OutputFormatArg::JsonPretty => OutputFormat::JsonPretty,
            OutputFormatArg::Table => OutputFormat::Table,
        }
    }
}

pub fn render<T: Serialize + Tabled>(format: &OutputFormat, item: &T) -> Result<()> {
    match format {
        OutputFormat::Json => json::render_json(item),
        OutputFormat::JsonPretty => json::render_json_pretty(item),
        OutputFormat::Table => table::render_table(std::slice::from_ref(item)),
    }
}

pub fn render_list<T: Serialize + Tabled>(format: &OutputFormat, items: &[T]) -> Result<()> {
    match format {
        OutputFormat::Json => json::render_json(items),
        OutputFormat::JsonPretty => json::render_json_pretty(items),
        OutputFormat::Table => table::render_table(items),
    }
}
