# account_transactions

This example enquires about account holder's own Co-operative Bank accounts' latest transactions for the specified account number and number of transactions to be returned..

## main.rs

This should contain below code:

```rust
mod enquiry {
    pub mod account_transactions;
}

// SANDBOX
const CONSUMER_KEY_SANDBOX: &str = "***";
const CONSUMER_SECRET_SANDBOX: &str = "***";

const ENVIRONMENT: &str = "sandbox";

#[tokio::main]
async fn main() {
    let consumer_key = CONSUMER_KEY_SANDBOX.to_string();
    let consumer_secret = CONSUMER_SECRET_SANDBOX.to_string();
    let _env = ENVIRONMENT.to_string();

    let x = enquiry::account_transactions::test_enquire_account_transactions(
        consumer_key,
        consumer_secret,
        _env,
    );
	
    x.await;
}
```

## account_transactions.rs

This module contains the function test_enquire_account_transactions:

```rust
use coop_connect_rust_sdk::models::models::TransactionsInputDetails;
use coop_connect_rust_sdk::CoopGateway;

pub async fn test_enquire_account_transactions(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = CoopGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(coop_connect) = _result {
        let message_reference = String::from("40ca18c6765086089a1");
        let account_number = String::from("***");
        let no_of_transactions = 5;

        let _result =
            TransactionsInputDetails::new(message_reference, account_number, no_of_transactions);

        if let Ok(account_details) = _result {
            let _output = coop_connect.enquire_account_transactions(account_details);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
            } else if let Err(e) = _result {
                println!("{:?}", e);
            } else {
                println!("Unexpected error occured during processing");
            }
        } else if let Err(e) = _result {
            println!("{:?}", e);
        } else {
            println!("Unexpected error occured during processing");
        }
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}
```
