use coop_connect_rust_sdk::models::models::{
    AccountSourceDetails, FundsTransferPesalinkAccountInputDetails,
    PesalinkAccountDestinationDetails,
};
use coop_connect_rust_sdk::CoopGateway;

pub async fn test_pesalink_send_to_account(
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
            let account_number = String::from("***");
            let bank_code = String::from("***");
            let _amount: u32 = 500;
            let transaction_currency = String::from("KES");
            let _narration = String::from("Electricity Payment");

            let _result = PesalinkAccountDestinationDetails::new(
                reference_number,
                account_number,
                bank_code,
                _amount,
                transaction_currency,
                _narration,
            );

            if let Ok(_destination) = _result {
                let mut _destinations = Vec::new();
                _destinations.push(_destination);

                let message_reference = String::from("40ca18c6765086089a1");
                let callback_url = String::from("https://yourdomain.com/ft-callback");

                let _result = FundsTransferPesalinkAccountInputDetails::new(
                    message_reference,
                    callback_url,
                    _source,
                    _destinations,
                );

                if let Ok(account_details) = _result {
                    let _output = coop_connect.pesalink_send_to_account(account_details);
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
