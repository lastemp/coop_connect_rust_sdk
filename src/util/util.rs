use crate::models::models::{
    AccountData, AccountDestinationData, AccountDestinationDetails, AccountFullStatementData,
    AccountSourceData, AccountSourceDetails, AccountTransactionsData, FundsTransferData,
    FundsTransferPesalinkAccountData, FundsTransferPesalinkPhoneData,
    PesalinkAccountDestinationData, PesalinkAccountDestinationDetails,
    PesalinkPhoneDestinationData, PesalinkPhoneDestinationDetails,
};
use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

pub fn build_headers(access_token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", access_token.parse().unwrap());

    headers
}

pub fn build_account_enquiry_data(
    message_reference: String,
    account_number: String,
) -> AccountData {
    AccountData {
        MessageReference: message_reference,
        AccountNumber: account_number,
    }
}

pub fn build_account_full_statement_data(
    message_reference: String,
    account_number: String,
    start_date: String,
    end_date: String,
) -> AccountFullStatementData {
    AccountFullStatementData {
        MessageReference: message_reference,
        AccountNumber: account_number,
        StartDate: start_date,
        EndDate: end_date,
    }
}

pub fn build_account_transactions_data(
    message_reference: String,
    account_number: String,
    no_of_transactions: u16,
) -> AccountTransactionsData {
    AccountTransactionsData {
        MessageReference: message_reference,
        AccountNumber: account_number,
        NoOfTransactions: no_of_transactions,
    }
}

pub fn build_account_funds_transfer_data(
    message_reference: String,
    callback_url: String,
    _source: &AccountSourceDetails,
    _destinations: &Vec<AccountDestinationDetails>,
) -> FundsTransferData {
    let source_data = AccountSourceData {
        AccountNumber: _source.get_account_number(),
        Amount: _source.get_amount(),
        TransactionCurrency: _source.get_transaction_currency(),
        Narration: _source.get_narration(),
    };

    let mut destination_data: Vec<AccountDestinationData> = Vec::new();

    for _destination in _destinations.iter() {
        let account_data = AccountDestinationData {
            ReferenceNumber: _destination.get_reference_number(),
            AccountNumber: _destination.get_account_number(),
            Amount: _destination.get_amount(),
            TransactionCurrency: _destination.get_transaction_currency(),
            Narration: _destination.get_narration(),
        };

        destination_data.push(account_data)
    }

    FundsTransferData {
        MessageReference: message_reference,
        CallBackUrl: callback_url,
        Source: source_data,
        Destinations: destination_data,
    }
}

pub fn build_account_funds_transfer_pesalink_account_data(
    message_reference: String,
    callback_url: String,
    _source: &AccountSourceDetails,
    _destinations: &Vec<PesalinkAccountDestinationDetails>,
) -> FundsTransferPesalinkAccountData {
    let source_data = AccountSourceData {
        AccountNumber: _source.get_account_number(),
        Amount: _source.get_amount(),
        TransactionCurrency: _source.get_transaction_currency(),
        Narration: _source.get_narration(),
    };

    let mut destination_data: Vec<PesalinkAccountDestinationData> = Vec::new();

    for _destination in _destinations.iter() {
        let account_data = PesalinkAccountDestinationData {
            ReferenceNumber: _destination.get_reference_number(),
            AccountNumber: _destination.get_account_number(),
            BankCode: _destination.get_bank_code(),
            Amount: _destination.get_amount(),
            TransactionCurrency: _destination.get_transaction_currency(),
            Narration: _destination.get_narration(),
        };

        destination_data.push(account_data)
    }

    FundsTransferPesalinkAccountData {
        MessageReference: message_reference,
        CallBackUrl: callback_url,
        Source: source_data,
        Destinations: destination_data,
    }
}

pub fn build_account_funds_transfer_pesalink_phone_data(
    message_reference: String,
    callback_url: String,
    _source: &AccountSourceDetails,
    _destinations: &Vec<PesalinkPhoneDestinationDetails>,
) -> FundsTransferPesalinkPhoneData {
    let source_data = AccountSourceData {
        AccountNumber: _source.get_account_number(),
        Amount: _source.get_amount(),
        TransactionCurrency: _source.get_transaction_currency(),
        Narration: _source.get_narration(),
    };

    let mut destination_data: Vec<PesalinkPhoneDestinationData> = Vec::new();

    for _destination in _destinations.iter() {
        let account_data = PesalinkPhoneDestinationData {
            ReferenceNumber: _destination.get_reference_number(),
            PhoneNumber: _destination.get_phone_number(),
            Amount: _destination.get_amount(),
            TransactionCurrency: _destination.get_transaction_currency(),
            Narration: _destination.get_narration(),
        };

        destination_data.push(account_data)
    }

    FundsTransferPesalinkPhoneData {
        MessageReference: message_reference,
        CallBackUrl: callback_url,
        Source: source_data,
        Destinations: destination_data,
    }
}

pub fn build_headers_generate_auth_token(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", api_key.parse().unwrap());

    headers
}
