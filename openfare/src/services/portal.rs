use anyhow::Result;

pub fn pay(
    all_extension_locks: &Vec<super::common::ExtensionLocks>,
    config: &crate::config::Config,
) -> Result<()> {
    let mut items = Vec::<_>::new();
    for extension_locks in all_extension_locks {
        let packages_plans = get_packages_plans(
            &extension_locks.extension_name,
            &extension_locks.package_locks,
            &config,
        )?;
        items.extend(packages_plans);
    }

    let order = openfare_lib::api::portal::basket::Order {
        items,
        api_key: config.services.portal.api_key.clone(),
    };

    if order.is_empty() {
        println!("No applicable payment plans found.");
        return Ok(());
    }

    let checkout_url = submit_order(&order, &config)?;
    println!("Checkout via URL:\n{}", checkout_url);
    Ok(())
}

/// Get applicable payment plans from packages.
fn get_packages_plans(
    extension_name: &str,
    package_locks: &std::collections::BTreeMap<
        openfare_lib::package::Package,
        openfare_lib::lock::Lock,
    >,
    config: &crate::config::Config,
) -> Result<Vec<openfare_lib::api::portal::basket::Item>> {
    let mut packages_plans: Vec<_> = vec![];
    for (package, lock) in package_locks {
        let plans =
            openfare_lib::lock::plan::filter_applicable(&lock.plans, &config.profile.parameters)?;
        if plans.is_empty() {
            // Skip package if no applicable plans found.
            continue;
        }

        let plans: Vec<_> = plans
            .into_iter()
            .map(|(plan_id, plan)| openfare_lib::api::portal::basket::Plan { plan_id, plan })
            .collect();

        let total_price = plans
            .iter()
            .map(|p| p.plan.payments.total.clone().unwrap_or_default())
            .sum();

        let order_item = openfare_lib::api::portal::basket::Item {
            package: package.clone(),
            extension_name: extension_name.to_string(),
            plans,
            total_price,
            payees: lock.payees.clone(),
        };
        packages_plans.push(order_item);
    }
    Ok(packages_plans)
}

fn submit_order(
    order: &openfare_lib::api::portal::basket::Order,
    config: &crate::config::Config,
) -> Result<url::Url> {
    let client = reqwest::blocking::Client::new();
    let url = config
        .services
        .portal
        .url
        .join(&openfare_lib::api::portal::basket::ROUTE)?;

    log::debug!("Submitting orders: {:?}", order);
    log::debug!("HTTP POST orders to endpoint: {}", url);
    let response = client.post(url.clone()).json(&order).send()?;
    if response.status() != 200 {
        return Err(anyhow::format_err!(
            "Portal response error ({status}):\n{url}",
            status = response.status(),
            url = url.to_string()
        ));
    }

    let response: openfare_lib::api::portal::basket::Response = response.json()?;
    Ok(response.checkout_url)
}
