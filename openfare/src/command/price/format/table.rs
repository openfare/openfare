use anyhow::Result;
use prettytable::{self, cell};

/// Generates and returns a table from a given price report.
pub fn get(
    price_report: &crate::price::PriceReport,
    first_row_separate: bool,
) -> Result<prettytable::Table> {
    let mut table = prettytable::Table::new();
    table.set_titles(prettytable::row![c =>
        "name",
        "version",
        format!("price ({})", price_report.price.currency.to_string()),
        "notes",
    ]);
    table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    let mut reports_iter = price_report.package_reports.iter();
    if first_row_separate {
        if let Some(report) = reports_iter.next() {
            let row = get_row(&report);
            table.add_row(row);
            table.add_row(prettytable::row![c => "", "", "", ""]);
        }
    }

    for report in reports_iter {
        let row = get_row(&report);
        table.add_row(row);
    }
    Ok(table)
}

fn get_row(report: &crate::price::PackagePriceReport) -> prettytable::Row {
    let price = report
        .price_quantity
        .map(|p| p.to_string())
        .unwrap_or("-".to_string());
    prettytable::Row::new(vec![
        prettytable::Cell::new_align(&report.package.name, prettytable::format::Alignment::LEFT),
        prettytable::Cell::new_align(
            &report.package.version,
            prettytable::format::Alignment::LEFT,
        ),
        prettytable::Cell::new_align(&price, prettytable::format::Alignment::CENTER),
    ])
}
