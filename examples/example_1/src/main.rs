mod enquiry {
    pub mod account_balance;
    pub mod account_full_statement;
    pub mod account_mini_statement;
    pub mod account_transactions;
    pub mod account_validation;
}

mod funds_transfer {
    pub mod internal {
        pub mod account_to_account;
    }
    pub mod external {
        pub mod pesalink {
            pub mod send_to_account;
            pub mod send_to_phone;
        }
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

    let x = enquiry::account_balance::test_enquire_account_balance(consumer_key, consumer_secret, _env);
    /*
    let x = enquiry::account_mini_statement::test_enquire_account_mini_statement(
        consumer_key,
        consumer_secret,
        _env,
    );
    */
    /*
    let x = enquiry::account_full_statement::test_enquire_account_full_statement(
        consumer_key,
        consumer_secret,
        _env,
    );
    */
    /*
    let x = enquiry::account_transactions::test_enquire_account_transactions(
        consumer_key,
        consumer_secret,
        _env,
    );
    */
    /*
    let x = enquiry::account_validation::test_enquire_account_validation(
        consumer_key,
        consumer_secret,
        _env,
    );
    */
    /*
    let x = funds_transfer::internal::account_to_account::test_a2a_transfer(
        consumer_key,
        consumer_secret,
        _env,
    );
    */
    /*
    let x = funds_transfer::external::pesalink::send_to_account::test_pesalink_send_to_account(
        consumer_key,
        consumer_secret,
        _env,
    );
    */
	/*
    let x = funds_transfer::external::pesalink::send_to_phone::test_pesalink_send_to_phone(
        consumer_key,
        consumer_secret,
        _env,
    );
	*/

    x.await;
}
