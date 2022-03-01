use anyhow::Result;
use rust_decimal::prelude::ToPrimitive;
use serde::de::Deserialize;

static BASE_URL: &str = "https://api.lnpay.co/v1/";
static DEFAULT_WALLET_NAME: &str = "openfare";

pub type Invoice = String;

#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Get request builder.
    fn get(&self, url: &url::Url) -> reqwest::blocking::RequestBuilder {
        self.client
            .get(url.clone())
            .header(reqwest::header::USER_AGENT, crate::common::HTTP_USER_AGENT)
            .header("X-Api-Key", self.api_key.clone())
    }

    /// Post request builder.
    fn post(&self, url: &url::Url) -> reqwest::blocking::RequestBuilder {
        self.client
            .post(url.clone())
            .header(reqwest::header::USER_AGENT, crate::common::HTTP_USER_AGENT)
            .header("X-Api-Key", self.api_key.clone())
    }

    /// Send request.
    fn send(
        &self,
        request: reqwest::blocking::RequestBuilder,
    ) -> Result<reqwest::blocking::Response> {
        log::debug!("Sending request: {:?}", &request);
        Ok(request.send()?)
    }

    pub fn wallets(&self) -> Result<Wallets> {
        let url = url::Url::parse(BASE_URL)?.join("wallets")?;
        let request = self.get(&url);
        let wallets: Vec<Wallet> = self.send(request)?.json()?;
        Ok(Wallets(wallets))
    }

    pub fn wallet(&self, user_label: &str) -> Result<Option<Wallet>> {
        Ok(self
            .wallets()?
            .iter()
            .filter(|w| w.user_label == user_label)
            .cloned()
            .next())
    }

    /// Creates a wallet if absent, otherwise returns error.
    pub fn create_wallet(&self, user_label: &str) -> Result<Wallet> {
        // Check for existing.
        let wallets = self.wallets()?;
        for wallet in wallets.0 {
            if wallet.user_label.to_lowercase() == user_label.to_string().to_lowercase() {
                return Err(anyhow::format_err!(
                    "Found existing wallet with the same user label."
                ));
            }
        }

        let url = url::Url::parse(BASE_URL)?.join("wallet")?;
        #[derive(Debug, serde::Serialize)]
        struct Body {
            user_label: String,
        }
        let body = Body {
            user_label: user_label.to_string(),
        };
        let body = serde_json::to_string(&body)?;

        let request = self
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::CONTENT_LENGTH, body.len())
            .body(body);
        let wallet: Wallet = self.send(request)?.json()?;
        Ok(wallet)
    }

    /// Creates a wallet if absent, otherwise returns existing.
    pub fn ensure_wallet(&self, user_label: &str) -> Result<Wallet> {
        Ok(if let Some(wallet) = self.wallet(&user_label)? {
            wallet
        } else {
            self.create_wallet(&user_label)?
        })
    }

    pub fn probe_lnurl(&self, lnurl: &str) -> Result<LnUrlProbe> {
        let url = url::Url::parse(BASE_URL)?.join(format!("lnurlp/probe/{}", lnurl).as_str())?;
        let request = self.get(&url);
        let probe: LnUrlProbe = self.send(request)?.json()?;
        Ok(probe)
    }

    pub fn invoice_from_lnurl(&self, amount_msat: usize, lnurl: &str) -> Result<Invoice> {
        let probe = self.probe_lnurl(&lnurl)?;
        let url = probe.callback;
        let url = url::Url::parse(&url)?;
        #[derive(Debug, serde::Deserialize)]
        struct Response {
            #[serde(rename = "pr")]
            invoice: String,
        }
        let request = self
            .get(&url)
            .query(&[("amount", amount_msat.to_string().as_str())]);
        let response: Response = self.send(request)?.json()?;
        Ok(response.invoice)
    }

    pub fn pay_invoice(&self, invoice: &Invoice, wallet: &Wallet) -> Result<serde_json::Value> {
        let url = url::Url::parse(BASE_URL)?.join(&format!(
            "wallet/{wallet_key}/withdraw",
            wallet_key = &wallet.key
        ))?;

        #[derive(Debug, serde::Serialize)]
        struct Body {
            payment_request: String,
        }
        let body = Body {
            payment_request: invoice.clone(),
        };
        let body = serde_json::to_string(&body)?;
        let request = self
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::CONTENT_LENGTH, body.len())
            .body(body);
        let transaction: serde_json::Value = self.send(request)?.json()?;
        Ok(transaction)
    }

    pub fn get_lnurl(&self, wallet: &Wallet) -> Result<String> {
        let lnurlpay_id = wallet
            .default_lnurlpay_id
            .clone()
            .ok_or(anyhow::format_err!(
                "Failed to parse wallet default_lnurlpay_id field."
            ))?;
        let url = url::Url::parse(BASE_URL)?.join(&format!("lnurlp/{lnurlpay_id}"))?;
        let request = self.get(&url);

        #[derive(Debug, serde::Deserialize)]
        struct Response {
            lnurl_encoded: String,
        }
        let response: Response = self.send(request)?.json()?;
        Ok(response.lnurl_encoded)
    }

    pub fn pay_lnurl(
        &self,
        lnurl: &str,
        amount_msat: usize,
        wallet: &Wallet,
        _comment: &str,
    ) -> Result<serde_json::Value> {
        let invoice = self.invoice_from_lnurl(amount_msat, &lnurl)?;
        let transaction = self.pay_invoice(&invoice, &wallet)?;
        Ok(transaction)
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LnUrlProbe {
    #[serde(rename = "minSendable")]
    min_sendable: usize,

    #[serde(rename = "maxSendable")]
    max_sendable: usize,

    #[serde(rename = "commentAllowed", deserialize_with = "bool_from_int")]
    comment_allowed: bool,

    callback: String,
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Wallets(Vec<Wallet>);

impl std::ops::Deref for Wallets {
    type Target = Vec<Wallet>;

    fn deref(&self) -> &Vec<Wallet> {
        &self.0
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Wallet {
    // Wallet key. Example: wal_...
    #[serde(rename = "id")]
    key: String,
    user_label: String,
    balance: Option<usize>,
    default_lnurlpay_id: Option<String>,
}

pub fn pay(
    donation_splits: &Option<Vec<(openfare_lib::lock::payee::Payee, openfare_lib::price::Price)>>,
    _items: &Vec<openfare_lib::api::services::portal::basket::Item>,
    config: &crate::config::Config,
) -> Result<()> {
    let lnpay_config = config
        .services
        .lnpay.clone().ok_or(anyhow::format_err!("Failed to find LNPAY config under services. Add LNPAY service: openfare service add lnpay --api-key=<key>"))?;
    if let Some(donation_splits) = donation_splits {
        pay_splits(&donation_splits, &lnpay_config)?;
    }
    // TODO: Handle applicable compulsory payments.
    Ok(())
}

fn pay_splits(
    splits: &Vec<(openfare_lib::lock::payee::Payee, openfare_lib::price::Price)>,
    lnpay_config: &crate::config::services::lnpay::LnPay,
) -> Result<()> {
    let total_payment: rust_decimal::Decimal = splits.iter().map(|(_, price)| price.quantity).sum();
    let client = Client::new(&lnpay_config.api_key);

    loop {
        if let Some(wallet) = client.wallet(DEFAULT_WALLET_NAME)? {
            log::debug!("Found payment origin wallet: {:?}", wallet);
            let balance = wallet.balance.unwrap_or_default();
            let balance = rust_decimal::Decimal::from(balance);
            let lightning_network_fee_buffer = rust_decimal::Decimal::from(10 as i64);
            log::debug!(
                "Adding lightning network fee buffer: {}",
                lightning_network_fee_buffer
            );
            let remainder = (total_payment + lightning_network_fee_buffer) - balance;
            if remainder > rust_decimal::Decimal::from(0 as i64) {
                let retry = handle_insufficient_balance(&remainder, &balance, &wallet, &client)?;
                if !retry {
                    break;
                }
            } else {
                println!("Found sufficient funds in wallet: {:?}", wallet);
                for (payee, amount) in splits {
                    println!("Paying {amount} to payee:\n{:?}", payee);
                    let lnurl = get_lnurl(&payee.profile)?.ok_or(anyhow::format_err!(
                        "Code error: Failed to find LNURL for split payment."
                    ))?;
                    let amount = amount.quantity.to_usize().ok_or(anyhow::format_err!(
                        "Failed to parse amount quantity as usize."
                    ))?;
                    let amount_msat = amount * 1000;
                    // TODO: Add LNURL comment giving origin.
                    client.pay_lnurl(&lnurl, amount_msat, &wallet, "")?;
                }
                break;
            }
        }
    }
    Ok(())
}

fn handle_insufficient_balance(
    remainder: &rust_decimal::Decimal,
    balance: &rust_decimal::Decimal,
    wallet: &Wallet,
    client: &Client,
) -> Result<bool> {
    let lnurl = client.get_lnurl(&wallet)?;
    let remainder = remainder.to_usize().ok_or(anyhow::format_err!(
        "Code error: remainder sats cant be represented as usize."
    ))?;
    let invoice = client.invoice_from_lnurl(remainder * 1000, &lnurl)?;

    println!(
        "Wallet '{DEFAULT_WALLET_NAME}' does not contain enough SATS. Current balance: {balance}."
    );
    println!("Opening QR invoice for remainder (+ 10 sats network fee buffer): {remainder} SATS.");

    let tmp_dir = tempdir::TempDir::new("openfare_pay_invoice_qr")?;
    let tmp_directory_path = tmp_dir.path().to_path_buf();
    show_qr(&invoice, &tmp_directory_path)?;
    Ok(dialoguer::Confirm::new()
        .with_prompt("Retry after invoice paid?")
        .interact()?)
}

fn show_qr(invoice: &str, tmp_directory_path: &std::path::PathBuf) -> Result<()> {
    let code = qrcode::QrCode::new(invoice.as_bytes())?;
    let image = code.render::<image::Luma<u8>>().build();
    let image_path = tmp_directory_path.join("invoice_qr.jpeg");
    image.save(&image_path)?;
    open::that_in_background(&image_path);
    Ok(())
}

fn get_lnurl(profile: &openfare_lib::profile::Profile) -> Result<Option<String>> {
    let payment_methods = profile.payment_methods()?;
    let payment_method = payment_methods
        .iter()
        .filter(|pm| pm.method() == openfare_lib::profile::payment_methods::Methods::BtcLightning)
        .next();
    if let Some(payment_method) = payment_method {
        let payment_method = payment_method.to_serde_json_value()?;
        let payment_method: openfare_lib::profile::payment_methods::BtcLightning =
            serde_json::from_value(payment_method)?;
        Ok(Some(payment_method.lnurl))
    } else {
        Ok(None)
    }
}

pub fn is_payee_applicable(payee: &openfare_lib::lock::payee::Payee) -> Result<bool> {
    Ok(get_lnurl(&payee.profile)?.is_some())
}

pub fn lnurl_receive_address(config: &crate::config::Config) -> Result<String> {
    let lnpay_config = config.services.lnpay.clone().ok_or(anyhow::format_err!("Failed to find LNPAY config under services. Add LNPAY service: openfare service add lnpay --api-key=<key>"))?;
    let client = Client::new(&lnpay_config.api_key);
    let wallet = client.ensure_wallet(DEFAULT_WALLET_NAME)?;
    Ok(client.get_lnurl(&wallet)?)
}
