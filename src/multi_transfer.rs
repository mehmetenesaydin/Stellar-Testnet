use reqwest::blocking::Client;
use serde_json::json;

pub fn send_multiple_xlm_transactions(sender_secret: &str, recipients: Vec<&str>, amount: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://horizon-testnet.stellar.org/transactions";

    for recipient in recipients {
        let transaction_data = json!({
            "sender": sender_secret,
            "recipient": recipient,
            "amount": amount,
            "memo": "Çoklu Transfer",
        });

        let client = Client::new();
        let response = client.post(url).json(&transaction_data).send()?;

        if response.status().is_success() {
            println!("{} adresine işlem başarıyla gerçekleştirildi!", recipient);
        } else {
            println!("{} adresine işlem başarısız: {:?}", recipient, response.text()?);
        }
    }

    Ok(())
}
