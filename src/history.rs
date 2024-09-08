use reqwest::blocking::Client;
use serde_json::Value;

pub fn get_transaction_history(public_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://horizon-testnet.stellar.org/accounts/{}/payments", public_key);

    let response = Client::new().get(&url).send()?;
    let json: Value = response.json()?;

    for payment in json["_embedded"]["records"].as_array().unwrap() {
        let amount = &payment["amount"];
        let memo = &payment["memo"];
        println!("Miktar: {}, Mesaj: {}", amount, memo);
    }

    Ok(())
}
