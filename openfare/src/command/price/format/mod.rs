use anyhow::Result;
mod table;

pub enum Format {
    Table,
}

pub fn print(
    report: &crate::price::PriceReport,
    format: &Format,
    first_row_separate: bool,
) -> Result<()> {
    match format {
        Format::Table => {
            let table = table::get(&report, first_row_separate)?;
            table.printstd();
        }
    }
    Ok(())
}
