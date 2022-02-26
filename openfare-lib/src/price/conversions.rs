use super::{Currency, Price};
use anyhow::Result;

pub fn one_btc_in_usd() -> Result<rust_decimal::Decimal> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("http://rate.sx/1BTC")
        .header(reqwest::header::USER_AGENT, crate::HTTP_USER_AGENT)
        .send()?
        .text()?;
    let response = response.replace("\n", "");
    let one_btc_in_usd = rust_decimal::Decimal::from_str_exact(&response)?;
    Ok(one_btc_in_usd)
}

// To BTC.

pub fn usd_to_btc(usd_price: &Price) -> Result<Price> {
    let target_currency = Currency::BTC;
    let one_btc_in_usd = one_btc_in_usd()?;
    let one_usd_in_btc = rust_decimal::Decimal::from(1) / one_btc_in_usd;
    let quantity = (usd_price.quantity * one_usd_in_btc).round_dp_with_strategy(
        target_currency.decimal_points(),
        rust_decimal::prelude::RoundingStrategy::AwayFromZero,
    );

    Ok(Price {
        quantity,
        currency: target_currency,
    })
}

pub fn sats_to_btc(sats_price: &Price) -> Result<Price> {
    let target_currency = Currency::BTC;
    let quantity = (sats_price.quantity * rust_decimal::Decimal::from_str_exact("0.00000001")?)
        .round_dp_with_strategy(
            target_currency.decimal_points(),
            rust_decimal::prelude::RoundingStrategy::AwayFromZero,
        );
    Ok(Price {
        quantity,
        currency: target_currency,
    })
}

// To sats.

pub fn usd_to_sats(usd_price: &Price) -> Result<Price> {
    let btc_price = usd_to_btc(usd_price)?;
    btc_to_sats(&btc_price)
}

pub fn btc_to_sats(btc_price: &Price) -> Result<Price> {
    let target_currency = Currency::SATS;
    let quantity = (btc_price.quantity * rust_decimal::Decimal::from(100000000 as i64))
        .round_dp_with_strategy(
            target_currency.decimal_points(),
            rust_decimal::prelude::RoundingStrategy::AwayFromZero,
        );
    Ok(Price {
        quantity,
        currency: target_currency,
    })
}

// To USD.

pub fn btc_to_usd(btc_price: &Price) -> Result<Price> {
    let one_btc_in_usd = one_btc_in_usd()?;
    let quantity = (one_btc_in_usd * btc_price.quantity).round_dp_with_strategy(
        Currency::USD.decimal_points(),
        rust_decimal::prelude::RoundingStrategy::AwayFromZero,
    );
    Ok(Price {
        quantity,
        currency: Currency::USD,
    })
}

pub fn sats_to_usd(sats_price: &Price) -> Result<Price> {
    let btc_price = sats_to_btc(&sats_price)?;
    btc_to_usd(&btc_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sats_to_btc() -> anyhow::Result<()> {
        let result = Price::try_from("50   sats")?;
        let result = sats_to_btc(&result)?;
        let expected = Price::try_from("0.00000050 btc")?;
        assert!(result == expected);
        Ok(())
    }

    #[test]
    fn test_btc_to_sats() -> anyhow::Result<()> {
        let result = Price::try_from("2 BTC")?;
        let result = btc_to_sats(&result)?;
        let expected = Price::try_from("200000000SATS")?;
        assert!(result == expected);
        Ok(())
    }
}
