use reqwest::blocking::Client;
use serde_json::Value;

pub fn get_account_balance(public_key: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("https://horizon-testnet.stellar.org/accounts/{}", public_key);
    let response = Client::new().get(&url).send()?;
    let json: Value = response.json()?;

    let balance_str = &json["balances"][0]["balance"];
    let balance: f64 = balance_str.as_str().unwrap_or("0").parse()?;

    Ok(balance)
}
