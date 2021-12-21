use anyhow::Result;

pub fn generate(
    package_configs: &openfare_lib::package::PackageConfigs,
    config: &crate::common::config::Config,
) -> Result<Option<PriceReport>> {
    log::info!("Generating price report for package and it's dependencies.");

    // Handle primary package first.
    let mut package_reports = vec![];

    if let Some(primary_package) = &package_configs.primary_package {
        let primary_package_price_report = get_package_price_report(
            &primary_package,
            &package_configs.primary_package_config,
            &config,
        )?;
        package_reports.push(primary_package_price_report);
    }

    for (package, package_config) in &package_configs.dependencies_configs {
        let price_report = get_package_price_report(&package, &package_config, &config)?;
        package_reports.push(price_report);
    }

    log::info!(
        "Number of price reports generated: {}",
        package_reports.len()
    );
    if package_reports.is_empty() {
        return Ok(None);
    }

    let total_price = package_reports
        .iter()
        .map(|r| r.price_quantity.unwrap_or(0))
        .sum::<u64>();

    let price_report = PriceReport {
        package_reports: package_reports,
        price: openfare_lib::package::plans::price::Price {
            quantity: total_price,
            currency: config.core.preferred_currency.clone(),
        },
    };
    Ok(Some(price_report))
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PriceReport {
    pub package_reports: Vec<PackagePriceReport>,
    pub price: openfare_lib::package::plans::price::Price,
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PackagePriceReport {
    pub package: openfare_lib::package::Package,
    pub price_quantity: Option<openfare_lib::package::plans::price::Quantity>,
    pub notes: Vec<String>,
}

/// Given a OpenFare package config, create a corresponding price report.
pub fn get_package_price_report(
    package: &openfare_lib::package::Package,
    package_config: &Option<openfare_lib::package::Config>,
    config: &crate::common::config::Config,
) -> Result<PackagePriceReport> {
    let package_config = match package_config {
        Some(c) => c,
        None => {
            return Ok(PackagePriceReport {
                package: package.clone(),
                price_quantity: None,
                notes: vec![],
            });
        }
    };

    let applicable_plans: Vec<_> = package_config
        .plans
        .iter()
        .filter(|plan| {
            plan.is_applicable(&config.metrics)
                .expect("plan applicable check")
        })
        .collect();

    Ok(if let Some(preferred_plan) = applicable_plans.first() {
        PackagePriceReport {
            package: package.clone(),
            price_quantity: Some(preferred_plan.total_price()?),
            notes: vec![],
        }
    } else {
        PackagePriceReport {
            package: package.clone(),
            price_quantity: Some(0),
            notes: vec![],
        }
    })
}
