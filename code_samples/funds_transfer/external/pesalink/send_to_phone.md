# send_to_phone

PesaLink Send to Phone Funds Transfer API will enable you to transfer funds from your own Co-operative Bank account to a Phone Number(s) linked to a Bank account in an IPSL participating bank.

## main.rs

This should contain below code:

```rust
pub mod external {
	pub mod pesalink {
		pub mod send_to_phone;
	}
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

    let x = funds_transfer::external::pesalink::send_to_phone::test_pesalink_send_to_phone(
        consumer_key,
        consumer_secret,
        _env,
    );
	
    x.await;
}
```

## send_to_phone.rs

This module contains the function test_pesalink_send_to_phone:

```rust
use coop_connect_rust_sdk::models::models::{
    AccountSourceDetails, FundsTransferPesalinkPhoneInputDetails, PesalinkPhoneDestinationDetails,
};
use coop_connect_rust_sdk::CoopGateway;

pub async fn test_pesalink_send_to_phone(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = CoopGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(coop_connect) = _result {
        let account_number = String::from("***");
        let _amount: u32 = 500;
        let transaction_currency = String::from("KES");
        let _narration = String::from("Supplier Payment");
        let _result =
            AccountSourceDetails::new(account_number, _amount, transaction_currency, _narration);

        if let Ok(_source) = _result {
            let reference_number = String::from("40ca18c6765086089a1_1");
            let phone_number = String::from("07xxxxxxxx"); // 07xxxxxxxx
            let _amount: u32 = 500;
            let transaction_currency = String::from("KES");
            let _narration = String::from("Electricity Payment");

            let _result = PesalinkPhoneDestinationDetails::new(
                reference_number,
                phone_number,
                _amount,
                transaction_currency,
                _narration,
            );

            if let Ok(_destination) = _result {
                let mut _destinations = Vec::new();
                _destinations.push(_destination);

                let message_reference = String::from("40ca18c6765086089a1");
                let callback_url = String::from("https://yourdomain.com/ft-callback");

                let _result = FundsTransferPesalinkPhoneInputDetails::new(
                    message_reference,
                    callback_url,
                    _source,
                    _destinations,
                );

                if let Ok(account_details) = _result {
                    let _output = coop_connect.pesalink_send_to_phone(account_details);
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
            }
        }
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}
```
