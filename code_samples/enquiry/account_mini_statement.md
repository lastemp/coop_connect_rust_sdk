# account_mini_statement

This example enquires about account holder's own Co-operative Bank accounts' Mini statement for the specified account number.

## main.rs

This should contain below code:

```rust
mod enquiry {
    pub mod account_mini_statement;
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

    let x = enquiry::account_mini_statement::test_enquire_account_mini_statement(
        consumer_key,
        consumer_secret,
        _env,
    );
	
    x.await;
}
```

## account_mini_statement.rs

This module contains the function test_enquire_account_mini_statement:

```rust
use coop_connect_rust_sdk::models::models::EnquiryInputDetails;
use coop_connect_rust_sdk::CoopGateway;

pub async fn test_enquire_account_mini_statement(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = CoopGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(coop_connect) = _result {
        let message_reference = String::from("40ca18c6765086089a1");
        let account_number = String::from("***");

        let _result = EnquiryInputDetails::new(message_reference, account_number);

        if let Ok(account_details) = _result {
            let _output = coop_connect.enquire_account_mini_statement(account_details);
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
