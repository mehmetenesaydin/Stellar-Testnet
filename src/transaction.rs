use reqwest::blocking::Client;
use serde_json::json;

pub fn send_xlm_transaction(sender_secret: &str, recipient: &str, amount: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://horizon-testnet.stellar.org/transactions";

    // İşlem verilerini hazırla
    let transaction_data = json!({
        "sender": sender_secret,
        "recipient": recipient,
        "amount": amount,
        "memo": message,
    });

    let client = Client::new();
    let response = client.post(url).json(&transaction_data).send()?;

    if response.status().is_success() {
        println!("İşlem başarıyla gerçekleştirildi!");
    } else {
        println!("İşlem başarısız: {:?}", response.text()?);
    }

    Ok(())
}
