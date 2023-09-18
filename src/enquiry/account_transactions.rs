use reqwest::StatusCode;

use crate::{
    models::models::{
        AccountTransactionsResponseData, BadRequestErrorResponseData, TransactionsInputDetails,
        UnauthorizedErrorResponseData,
    },
    util::util::{build_account_transactions_data, build_headers},
};

pub async fn enquire(
    account_details: TransactionsInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<
    (
        Option<AccountTransactionsResponseData>,
        Option<BadRequestErrorResponseData>,
        Option<UnauthorizedErrorResponseData>,
    ),
    String,
> {
    let message_reference: String = account_details.get_message_reference();
    let account_number = account_details.get_account_number();
    let no_of_transactions = account_details.get_no_of_transactions();

    // Lets build the request params as a struct
    let account_data =
        build_account_transactions_data(message_reference, account_number, no_of_transactions);

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&account_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                match response.json::<AccountTransactionsResponseData>().await {
                    Ok(account_balance_response_data) => {
                        // Handle success case

                        let my_output = (Some(account_balance_response_data), None, None);

                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            StatusCode::BAD_REQUEST => {
                match response.json::<BadRequestErrorResponseData>().await {
                    Ok(account_balance_error_response_data) => {
                        // Handle success case

                        let my_output = (None, Some(account_balance_error_response_data), None);
                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            StatusCode::UNAUTHORIZED => {
                match response.json::<UnauthorizedErrorResponseData>().await {
                    Ok(account_balance_error_response_data) => {
                        // Handle success case

                        let my_output = (None, None, Some(account_balance_error_response_data));
                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
