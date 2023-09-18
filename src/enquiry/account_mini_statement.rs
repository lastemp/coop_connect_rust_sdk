use reqwest::StatusCode;

use crate::{
    models::models::{
        AccountStatementResponseData, BadRequestErrorResponseData, EnquiryInputDetails,
        UnauthorizedErrorResponseData,
    },
    util::util::{build_account_enquiry_data, build_headers},
};

pub async fn enquire(
    account_details: EnquiryInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<
    (
        Option<AccountStatementResponseData>,
        Option<BadRequestErrorResponseData>,
        Option<UnauthorizedErrorResponseData>,
    ),
    String,
> {
    let message_reference: String = account_details.get_message_reference();
    let account_number = account_details.get_account_number();

    // Lets build the request params as a struct
    let account_data = build_account_enquiry_data(message_reference, account_number);

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
                match response.json::<AccountStatementResponseData>().await {
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
