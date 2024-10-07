# Stellar Payment and Messaging System

## Project Name
Stellar Payment and Messaging System

## Who am I
- **Name:** Mehmet Enes Aydın
- **Title:** Software Developer Developer
- **Role:** Smart Contract and Rust Developer
- **Experience:** Stellar Blockchain Development, Smart Contracts, and Decentralized Applications


## Project Details
The Stellar Payment and Messaging System allows users to perform various actions such as checking balances, making XLM transfers, viewing transaction history, and performing multi-recipient transfers. The application is built using Rust and utilizes the Stellar Blockchain's testnet for transaction and balance inquiries.

## Vision
Our project aims to streamline digital payments and integrate a messaging feature within each transaction, making blockchain technology more accessible and user-friendly. We believe this project will enable seamless payments and open new possibilities for messaging in decentralized environments, creating a more connected and efficient ecosystem for users and businesses alike.

## Installation and Setup

1. **Clone the Repository:**
    ```bash
    git clone https://github.com/your-username/stellar-payment-system.git
    cd stellar-payment-system
    ```

2. **Install Dependencies:**
   - Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
   - Install `reqwest` and `serde` crates using `cargo`:
    ```bash
    cargo install reqwest serde
    ```

3. **Run the Project:**
   ```bash
   cargo run

Project Structure
- **main.rs** - Entry point of the application with main menu options.
- **stellar.rs** - Contains functions to interact with Stellar Blockchain.
- **transaction.rs** - Handles XLM transfers and message sending.
- **multi_transfer.rs** - Allows sending XLM to multiple recipients.
- **history.rs** - Validates Stellar addresses and manages transaction history.
- **utils.rs** - Helper functions for input and validation.

**Usage**
Once the project is running, you will see a menu with the following options:

- **Check Balance** - Enter a Stellar public key to view the balance.
- **Make XLM Transfer** - Send XLM to a single recipient with an optional message.
- **View Transaction History** - Display the history of completed transactions.
- **Multi-Recipient Transfer** - Perform a transfer to multiple addresses at once.
- **Contributing**
Feel free to open an issue or submit a pull request if you want to contribute to this project.

**License**
This project is licensed under the MIT License.
