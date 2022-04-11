use anyhow::Result;

pub fn donation_splits(
    donation: &openfare_lib::price::Price,
    items: &Vec<openfare_lib::api::services::basket::Item>,
    is_payee_applicable: fn(&openfare_lib::lock::payee::Payee) -> Result<bool>,
) -> Result<Vec<(openfare_lib::lock::payee::Payee, openfare_lib::price::Price)>> {
    match &donation.currency {
        openfare_lib::price::Currency::SATS => println!("Donation: {}", donation),
        _ => println!(
            "Donation: {} ({sats})",
            donation,
            sats = donation.to_sats()?
        ),
    }
    let donation = donation.to_sats()?;

    // Filter for package which has a volunteer plan and at least one applicable payee.
    let items = filter_voluntary(&items, is_payee_applicable);

    // Round down to avoid overflowing specified donation.
    let package_donation_quantity = (donation.quantity / rust_decimal::Decimal::from(items.len()))
        .round_dp_with_strategy(
            donation.currency.decimal_points(),
            rust_decimal::prelude::RoundingStrategy::ToZero,
        );
    let package_donation = openfare_lib::price::Price {
        quantity: package_donation_quantity,
        currency: donation.currency.clone(),
    };
    println!(
        "Splitting donation between {count_packages} packages. {package_donation} each.",
        count_packages = items.len(),
        package_donation = package_donation
    );

    let mut payee_donations =
        Vec::<(openfare_lib::lock::payee::Payee, openfare_lib::price::Price)>::new();
    for item in items {
        let payees = filter_for_applicable_payees(&item.payees, is_payee_applicable)?;

        if let Some(shares) = &item.shares {
            // Only consider shares for applicable payees.
            let shares = filter_for_applicable_shares(&shares, &payees)?;
            let total: u64 = shares.iter().map(|(_, share)| share).sum();
            let total = rust_decimal::Decimal::from(total);

            for (label, share) in shares {
                let share = rust_decimal::Decimal::from(share);
                let fraction = share / total;

                // Round down to avoid overflowing specified donation.
                let payee_donation_quantity = (package_donation.quantity * fraction)
                    .round_dp_with_strategy(
                        package_donation.currency.decimal_points(),
                        rust_decimal::prelude::RoundingStrategy::ToZero,
                    );
                let payee_donation = openfare_lib::price::Price {
                    quantity: payee_donation_quantity,
                    currency: package_donation.currency.clone(),
                };

                if let Some(payee) = item.payees.get(label.as_str()) {
                    payee_donations.push((payee.clone(), payee_donation));
                }
            }
        } else {
            // No shares defined, split package donation evenly between all applicable payees.
            // Round down to avoid overflowing specified donation.
            let payee_donation_quantity = (package_donation.quantity
                / rust_decimal::Decimal::from(payees.len()))
            .round_dp_with_strategy(
                package_donation.currency.decimal_points(),
                rust_decimal::prelude::RoundingStrategy::ToZero,
            );
            let payee_donation = openfare_lib::price::Price {
                quantity: payee_donation_quantity,
                currency: package_donation.currency.clone(),
            };
            for (_, payee) in &payees {
                payee_donations.push((payee.clone(), payee_donation.clone()));
            }
        }
    }

    check_payee_donations(&donation, &payee_donations);
    Ok(payee_donations)
}

/// Filter for items which have at least one voluntary payment plan and corresponding applicable payee.
fn filter_voluntary(
    items: &Vec<openfare_lib::api::services::basket::Item>,
    is_payee_applicable: fn(&openfare_lib::lock::payee::Payee) -> Result<bool>,
) -> Vec<openfare_lib::api::services::basket::Item> {
    items
        .iter()
        .cloned()
        .filter(|item| {
            let voluntary_plans = item
                .plans
                .iter()
                .filter(|(_id, plan)| plan.r#type == openfare_lib::lock::plan::PlanType::Voluntary)
                .collect::<Vec<_>>();

            let valid_payees =
                filter_for_applicable_payees(&item.valid_payees(), is_payee_applicable)
                    .unwrap_or_default();
            !valid_payees.is_empty() && !voluntary_plans.is_empty()
        })
        .collect()
}

fn filter_for_applicable_payees(
    payees: &openfare_lib::lock::payee::Payees,
    is_payee_applicable: fn(&openfare_lib::lock::payee::Payee) -> Result<bool>,
) -> Result<openfare_lib::lock::payee::Payees> {
    Ok(payees
        .iter()
        .filter(|(_, payee)| is_payee_applicable(&payee).unwrap_or(false))
        .map(|(label, payee)| (label.clone(), payee.clone()))
        .collect())
}

fn filter_for_applicable_shares(
    shares: &openfare_lib::lock::plan::Shares,
    applicable_label_payees: &openfare_lib::lock::payee::Payees,
) -> Result<openfare_lib::lock::plan::Shares> {
    Ok(shares
        .iter()
        .filter(|(label, _share)| applicable_label_payees.contains_key(label.as_str()))
        .map(|(label, share)| (label.clone(), share.clone()))
        .collect())
}

fn check_payee_donations(
    total_donation: &openfare_lib::price::Price,
    payee_donations: &Vec<(openfare_lib::lock::payee::Payee, openfare_lib::price::Price)>,
) {
    let total_payee_donations: openfare_lib::price::Quantity = payee_donations
        .iter()
        .map(|(_, price)| price.quantity)
        .sum();
    assert!(total_donation.quantity >= total_payee_donations);
}
