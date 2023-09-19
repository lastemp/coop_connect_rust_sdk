use reqwest::StatusCode;

use crate::{
    models::models::{
        AccountFundsTransferResponseData, FundsTransferPesalinkPhoneInputDetails,
        UnauthorizedErrorResponseData,
    },
    util::util::{build_account_funds_transfer_pesalink_phone_data, build_headers},
};

pub async fn transfer(
    account_details: FundsTransferPesalinkPhoneInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<
    (
        Option<AccountFundsTransferResponseData>,
        Option<UnauthorizedErrorResponseData>,
    ),
    String,
> {
    let message_reference = account_details.get_message_reference();
    let callback_url = account_details.get_callback_url();
    let _source = account_details.get_source();
    let _destinations = account_details.get_destinations();

    // Lets build the request params as a struct
    let transfer_data = build_account_funds_transfer_pesalink_phone_data(
        message_reference,
        callback_url,
        _source,
        _destinations,
    );

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&transfer_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            // 200-OK, 400-BAD_REQUEST, 403-FORBIDDEN, 409-CONFLICT
            StatusCode::OK
            | StatusCode::BAD_REQUEST
            | StatusCode::FORBIDDEN
            | StatusCode::CONFLICT => {
                match response.json::<AccountFundsTransferResponseData>().await {
                    Ok(account_transfer_response_data) => {
                        // Handle success case

                        let my_output = (Some(account_transfer_response_data), None);

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
                    Ok(account_transfer_error_response_data) => {
                        // Handle success case

                        let my_output = (None, Some(account_transfer_error_response_data));
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
