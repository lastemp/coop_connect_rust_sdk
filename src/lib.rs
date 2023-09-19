pub mod models {
    pub mod models;
}
mod util {
    pub mod util;
}
mod authorization {
    pub mod generate_auth_token;
}
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
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use models::models::{
    AccountBalanceResponseData, AccountFundsTransferResponseData, AccountStatementResponseData,
    AccountTransactionsResponseData, AccountValidationResponseData, BadRequestErrorResponseData,
    EnquiryInputDetails, FullStatementInputDetails, FundsTransferInputDetails,
    FundsTransferPesalinkAccountInputDetails, FundsTransferPesalinkPhoneInputDetails,
    TransactionsInputDetails, UnauthorizedErrorResponseData,
};

const AUTHORISATION_BEARER: &str = "Bearer";
const GRANT_TYPE: &str = "client_credentials";

const AUTH_TOKEN_URL_SANDBOX: &str = "https://developer.co-opbank.co.ke/token";
const AUTH_TOKEN_URL_PROD: &str = "https://developer.co-opbank.co.ke/token";

const ACCOUNT_BALANCE_URL_SANDBOX: &str =
    "https://developer.co-opbank.co.ke/Enquiry/AccountBalance/1.0.0";
const ACCOUNT_BALANCE_URL_PROD: &str =
    "https://developer.co-opbank.co.ke/Enquiry/AccountBalance/1.0.0";
const ACCOUNT_VALIDATION_URL_SANDBOX: &str =
    "https://developer.co-opbank.co.ke/Enquiry/Validation/Account/1.0.0";
const ACCOUNT_VALIDATION_URL_PROD: &str =
    "https://developer.co-opbank.co.ke/Enquiry/Validation/Account/1.0.0";

const ACCOUNT_MINI_STATEMENT_URL_SANDBOX: &str =
    "https://developer.co-opbank.co.ke/Enquiry/MiniStatement/Account/1.0.0";
const ACCOUNT_MINI_STATEMENT_URL_PROD: &str =
    "https://developer.co-opbank.co.ke/Enquiry/MiniStatement/Account/1.0.0";

const ACCOUNT_FULL_STATEMENT_URL_SANDBOX: &str =
    "https://developer.co-opbank.co.ke/Enquiry/FullStatement/Account/1.0.0";
const ACCOUNT_FULL_STATEMENT_URL_PROD: &str =
    "https://developer.co-opbank.co.ke/Enquiry/FullStatement/Account/1.0.0";

const ACCOUNT_TRANSACTIONS_URL_SANDBOX: &str =
    "https://developer.co-opbank.co.ke/Enquiry/AccountTransactions/1.0.0";
const ACCOUNT_TRANSACTIONS_URL_PROD: &str =
    "https://developer.co-opbank.co.ke/Enquiry/AccountTransactions/1.0.0";
const ACCOUNT_TO_ACCOUNT_TRANSFER_URL_SANDBOX: &str =
    "https://developer.co-opbank.co.ke/FundsTransfer/Internal/A2A/2.0.0";
const ACCOUNT_TO_ACCOUNT_TRANSFER_URL_PROD: &str =
    "https://developer.co-opbank.co.ke/FundsTransfer/Internal/A2A/2.0.0";
const PESALINK_SEND_TO_ACCOUNT_URL_SANDBOX: &str =
    "https://developer.co-opbank.co.ke/FundsTransfer/External/A2A/PesaLink/1.0.0";
const PESALINK_SEND_TO_ACCOUNT_URL_PROD: &str =
    "https://developer.co-opbank.co.ke/FundsTransfer/External/A2A/PesaLink/1.0.0";
const PESALINK_SEND_TO_PHONE_URL_SANDBOX: &str =
    "https://developer.co-opbank.co.ke/FundsTransfer/External/A2M/PesaLink/1.0.0";
const PESALINK_SEND_TO_PHONE_URL_PROD: &str =
    "https://developer.co-opbank.co.ke/FundsTransfer/External/A2M/PesaLink/1.0.0";

#[derive(Debug)]
pub struct CoopGateway {
    grant_type: String,
    consumer_key: String,
    consumer_secret: String,
    auth_token_url: String,
    account_balance_url: String,
    account_validation_url: String,
    account_mini_statement_url: String,
    account_full_statement_url: String,
    account_transactions_url: String,
    account_to_account_transfer_url: String,
    pesalink_send_to_account_url: String,
    pesalink_send_to_phone_url: String,
}

impl CoopGateway {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        _env: String,
    ) -> Result<Self, String> {
        if consumer_key.is_empty() || consumer_key.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer key is empty"));
        }

        if consumer_secret.is_empty() || consumer_secret.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer secret is empty"));
        }

        if _env.is_empty() || _env.replace(" ", "").trim().len() == 0 {
            return Err(String::from("_env is empty"));
        }

        if _env.eq_ignore_ascii_case(&String::from("sandbox"))
            || _env.eq_ignore_ascii_case(&String::from("prod"))
        {
            // valid _env
        } else {
            return Err(String::from("invalid env"));
        }

        let grant_type = GRANT_TYPE.to_string();

        let auth_token_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            AUTH_TOKEN_URL_PROD.to_string()
        } else {
            AUTH_TOKEN_URL_SANDBOX.to_string()
        };

        let account_balance_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            ACCOUNT_BALANCE_URL_PROD.to_string()
        } else {
            ACCOUNT_BALANCE_URL_SANDBOX.to_string()
        };

        let account_validation_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            ACCOUNT_VALIDATION_URL_PROD.to_string()
        } else {
            ACCOUNT_VALIDATION_URL_SANDBOX.to_string()
        };

        let account_mini_statement_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            ACCOUNT_MINI_STATEMENT_URL_PROD.to_string()
        } else {
            ACCOUNT_MINI_STATEMENT_URL_SANDBOX.to_string()
        };

        let account_full_statement_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            ACCOUNT_FULL_STATEMENT_URL_PROD.to_string()
        } else {
            ACCOUNT_FULL_STATEMENT_URL_SANDBOX.to_string()
        };

        let account_transactions_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            ACCOUNT_TRANSACTIONS_URL_PROD.to_string()
        } else {
            ACCOUNT_TRANSACTIONS_URL_SANDBOX.to_string()
        };

        let account_to_account_transfer_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            ACCOUNT_TO_ACCOUNT_TRANSFER_URL_PROD.to_string()
        } else {
            ACCOUNT_TO_ACCOUNT_TRANSFER_URL_SANDBOX.to_string()
        };

        let pesalink_send_to_account_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            PESALINK_SEND_TO_ACCOUNT_URL_PROD.to_string()
        } else {
            PESALINK_SEND_TO_ACCOUNT_URL_SANDBOX.to_string()
        };

        let pesalink_send_to_phone_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            PESALINK_SEND_TO_PHONE_URL_PROD.to_string()
        } else {
            PESALINK_SEND_TO_PHONE_URL_SANDBOX.to_string()
        };

        Ok(Self {
            grant_type,
            consumer_key,
            consumer_secret,
            auth_token_url,
            account_balance_url,
            account_validation_url,
            account_mini_statement_url,
            account_full_statement_url,
            account_transactions_url,
            account_to_account_transfer_url,
            pesalink_send_to_account_url,
            pesalink_send_to_phone_url,
        })
    }

    fn get_api_key(&self) -> String {
        let consumer_key = &self.consumer_key;
        let consumer_secret = &self.consumer_secret;
        let mut password: String = consumer_key.to_string();
        let k = ":"; // Separator
        password.push_str(k);
        password.push_str(&consumer_secret);
        let encodedpassword = general_purpose::STANDARD.encode(password);

        let mut api_key = String::from("Basic");
        let k = " "; // Separator
        api_key.push_str(k);
        api_key.push_str(&encodedpassword);

        api_key
    }

    fn parse_auth_token(&self, access_token_result: String) -> String {
        let access_token: String = if !access_token_result.is_empty()
            && access_token_result.replace(" ", "").trim().len() > 0
        {
            let mut access_token = AUTHORISATION_BEARER.to_string();
            let k = " "; // Separator
            access_token.push_str(k);
            access_token.push_str(&access_token_result);

            access_token
        } else {
            String::from("")
        };

        access_token
    }

    async fn get_auth_token(&self) -> std::result::Result<String, String> {
        let grant_type = &self.grant_type;
        let api_key = &self.get_api_key();
        let api_url = &self.auth_token_url;

        let _result = authorization::generate_auth_token::get_auth_token(
            grant_type.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        )
        .await;

        _result
    }

    pub async fn enquire_account_balance(
        &self,
        account_details: EnquiryInputDetails,
    ) -> std::result::Result<
        (
            Option<AccountBalanceResponseData>,
            Option<BadRequestErrorResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                println!("access_token_result {:?}", access_token_result);
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_balance_url;

                let _result = enquiry::account_balance::enquire(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_account_validation(
        &self,
        account_details: EnquiryInputDetails,
    ) -> std::result::Result<
        (
            Option<AccountValidationResponseData>,
            Option<BadRequestErrorResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_validation_url;

                let _result = enquiry::account_validation::enquire(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_account_mini_statement(
        &self,
        account_details: EnquiryInputDetails,
    ) -> std::result::Result<
        (
            Option<AccountStatementResponseData>,
            Option<BadRequestErrorResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_mini_statement_url;

                let _result = enquiry::account_mini_statement::enquire(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_account_full_statement(
        &self,
        account_details: FullStatementInputDetails,
    ) -> std::result::Result<
        (
            Option<AccountStatementResponseData>,
            Option<BadRequestErrorResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_full_statement_url;

                let _result = enquiry::account_full_statement::enquire(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_account_transactions(
        &self,
        account_details: TransactionsInputDetails,
    ) -> std::result::Result<
        (
            Option<AccountTransactionsResponseData>,
            Option<BadRequestErrorResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_transactions_url;

                let _result = enquiry::account_transactions::enquire(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn a2a_transfer(
        &self,
        account_details: FundsTransferInputDetails,
    ) -> std::result::Result<
        (
            Option<AccountFundsTransferResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_to_account_transfer_url;

                let _result = funds_transfer::internal::account_to_account::transfer(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn pesalink_send_to_account(
        &self,
        account_details: FundsTransferPesalinkAccountInputDetails,
    ) -> std::result::Result<
        (
            Option<AccountFundsTransferResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.pesalink_send_to_account_url;

                let _result = funds_transfer::external::pesalink::send_to_account::transfer(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn pesalink_send_to_phone(
        &self,
        account_details: FundsTransferPesalinkPhoneInputDetails,
    ) -> std::result::Result<
        (
            Option<AccountFundsTransferResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.pesalink_send_to_phone_url;

                let _result = funds_transfer::external::pesalink::send_to_phone::transfer(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coop_gateway() {
        let consumer_key = String::from("***");
        let consumer_secret = String::from("***");
        let _env = String::from("sandbox");

        let _result = CoopGateway::new(consumer_key, consumer_secret, _env);
        assert_eq!(_result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_enquire_account_balance() {
        let consumer_key = String::from("***");
        let consumer_secret = String::from("***");
        let _env = String::from("sandbox");

        let _result = CoopGateway::new(consumer_key, consumer_secret, _env);

        if let Ok(coop_connect) = _result {
            let message_reference = String::from("***");
            let account_number = String::from("***");

            let _result = EnquiryInputDetails::new(message_reference, account_number);

            if let Ok(account_details) = _result {
                // Initiate the request through the sdk
                let _output = coop_connect.enquire_account_balance(account_details);
                let _result = _output.await;
                assert_eq!(_result.is_ok(), true);
            }
        }
    }
}
