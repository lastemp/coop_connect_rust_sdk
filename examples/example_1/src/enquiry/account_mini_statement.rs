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
