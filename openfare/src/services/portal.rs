use anyhow::Result;

pub fn pay(
    items: &Vec<openfare_lib::api::services::portal::basket::Item>,
    config: &crate::config::Config,
) -> Result<()> {
    let order = openfare_lib::api::services::portal::basket::Order {
        items: items.clone(),
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

fn submit_order(
    order: &openfare_lib::api::services::portal::basket::Order,
    config: &crate::config::Config,
) -> Result<url::Url> {
    let client = reqwest::blocking::Client::new();
    let url = config
        .services
        .portal
        .url
        .join(&openfare_lib::api::services::portal::basket::ROUTE)?;

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

    let response: openfare_lib::api::services::portal::basket::Response = response.json()?;
    Ok(response.checkout_url)
}
