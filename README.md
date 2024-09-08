This project is a simple payment and messaging system built using the Rust programming language. 
The system integrates with Stellar's testnet to allow users to send payments using Stellar's native token, XLM, while attaching short messages to transactions.
It provides core functionalities such as checking account balances, sending payments to multiple recipients, and viewing transaction history.
By leveraging the reqwest library for HTTP requests and serde for JSON parsing, this project demonstrates a seamless integration between Rust and the Stellar blockchain.

This phrase serves as a good transition before listing the files and their explanations in English. Let me know if you need further assistance!

1. main.rs
This is the entry point of the program where the main application logic is executed. It sets up the Stellar blockchain integration using the reqwest library and handles user input for making XLM transactions, checking balances, and viewing transaction history.

Main functionality: The program initializes by fetching account details, processing XLM transfers, and providing options to view balance and transaction history. It allows users to interact with Stellar's testnet through a simple command-line interface.
2. account.rs
This file contains the logic for interacting with Stellar accounts. It allows users to retrieve the account balance and check the validity of Stellar public keys.

Key functions:
get_account_balance: This function fetches the current balance of the user from the Stellar blockchain using their public key.
is_valid_stellar_address: Verifies whether a given Stellar public key is in the correct format.
3. transaction.rs
This file handles the logic for making XLM transfers between users and adding short messages to those transfers. It also supports the transfer of XLM to multiple recipients in a single transaction.

Key functions:
send_payment: Sends XLM from one account to another and attaches an optional message.
send_multiple_payments: Supports multiple recipients in a single transaction, where XLM can be sent to multiple addresses at once.
4. history.rs
This file manages the functionality for retrieving and displaying transaction history, including the attached messages. It communicates with the Stellar testnet to get the user's past transactions.

Key functions:
get_transaction_history: Retrieves and prints the user's past transactions and their associated messages, providing insight into previous transfers.
5. error.rs
This file defines custom error handling mechanisms for the application. Instead of panicking when something goes wrong, it provides meaningful error messages.

Key types:
AppError: A custom error type used throughout the program to handle different kinds of errors (e.g., network issues, invalid Stellar addresses).
Result<T>: A specialized result type for better error handling.
6. utils.rs
This file contains utility functions that are reused throughout the program, such as validating Stellar addresses and getting user input. These functions help improve code readability and reusability.

Key functions:
is_valid_stellar_address: Validates the format of a Stellar public key using regular expressions.
get_user_input: Prompts the user for input in the command-line interface and returns the trimmed string.
7. Cargo.toml
While not a Rust source file, this file contains metadata and dependencies for the project. It defines the libraries required for the application, such as reqwest for HTTP requests and serde for JSON parsing. Additionally, it specifies the project dependencies and their required features (e.g., blocking for synchronous requests).

Key dependencies:
reqwest: Handles HTTP requests to Stellar's Horizon API.
serde and serde_json: Used for serializing and deserializing JSON data.
These files together form the backbone of the Stellar-based payment and messaging system, enabling users to send payments, view balances, and interact with the Stellar testnet using Rust.
