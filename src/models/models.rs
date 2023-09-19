use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct EnquiryInputDetails {
    message_reference: String,
    account_number: String,
}

impl EnquiryInputDetails {
    pub fn new(message_reference: String, account_number: String) -> Result<Self, String> {
        if message_reference.is_empty() || message_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message reference is empty"));
        }
        // message_reference has a max length of 27 characters
        else if message_reference.trim().len() > 0 && message_reference.trim().len() <= 27 {
            // message_reference is valid
        } else {
            return Err(String::from("message reference has invalid length"));
        }

        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a length of 14 characters
        else if account_number.trim().len() == 14 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        Ok(Self {
            message_reference,
            account_number,
        })
    }

    pub fn get_message_reference(&self) -> String {
        let message_reference = &self.message_reference;
        message_reference.to_string()
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }
}

#[derive(Debug)]
pub struct FullStatementInputDetails {
    message_reference: String,
    account_number: String,
    start_date: String,
    end_date: String,
}

impl FullStatementInputDetails {
    pub fn new(
        message_reference: String,
        account_number: String,
        start_date: String,
        end_date: String,
    ) -> Result<Self, String> {
        if message_reference.is_empty() || message_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message reference is empty"));
        }
        // message_reference has a max length of 27 characters
        else if message_reference.trim().len() > 0 && message_reference.trim().len() <= 27 {
            // message_reference is valid
        } else {
            return Err(String::from("message reference has invalid length"));
        }

        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a length of 14 characters
        else if account_number.trim().len() == 14 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        if start_date.is_empty() || start_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("start date is empty"));
        }
        // start_date has a length of 10 characters
        else if start_date.trim().len() == 10 {
            // start_date is valid
        } else {
            return Err(String::from("start date has invalid length"));
        }

        if end_date.is_empty() || end_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("end date is empty"));
        }
        // end_date has a length of 10 characters
        else if end_date.trim().len() == 10 {
            // end_date is valid
        } else {
            return Err(String::from("end date has invalid length"));
        }

        Ok(Self {
            message_reference,
            account_number,
            start_date,
            end_date,
        })
    }

    pub fn get_message_reference(&self) -> String {
        let message_reference = &self.message_reference;
        message_reference.to_string()
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }

    pub fn get_start_date(&self) -> String {
        let start_date = &self.start_date;
        start_date.to_string()
    }

    pub fn get_end_date(&self) -> String {
        let end_date = &self.end_date;
        end_date.to_string()
    }
}

#[derive(Debug)]
pub struct TransactionsInputDetails {
    message_reference: String,
    account_number: String,
    no_of_transactions: u16,
}

impl TransactionsInputDetails {
    pub fn new(
        message_reference: String,
        account_number: String,
        no_of_transactions: u16,
    ) -> Result<Self, String> {
        if message_reference.is_empty() || message_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message reference is empty"));
        }
        // message_reference has a max length of 27 characters
        else if message_reference.trim().len() > 0 && message_reference.trim().len() <= 27 {
            // message_reference is valid
        } else {
            return Err(String::from("message reference has invalid length"));
        }

        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a length of 14 characters
        else if account_number.trim().len() == 14 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        if no_of_transactions == 0 {
            return Err(String::from("no of transactions has invalid value"));
        }

        Ok(Self {
            message_reference,
            account_number,
            no_of_transactions,
        })
    }

    pub fn get_message_reference(&self) -> String {
        let message_reference = &self.message_reference;
        message_reference.to_string()
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }

    pub fn get_no_of_transactions(&self) -> u16 {
        let no_of_transactions = &self.no_of_transactions;
        *no_of_transactions
    }
}

#[derive(Debug)]
pub struct AccountSourceDetails {
    account_number: String,
    _amount: u32,
    transaction_currency: String,
    _narration: String,
}

impl AccountSourceDetails {
    pub fn new(
        account_number: String,
        _amount: u32,
        transaction_currency: String,
        _narration: String,
    ) -> Result<Self, String> {
        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a length of 14 characters
        else if account_number.trim().len() == 14 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        if _amount == 0 {
            return Err(String::from("amount has invalid value"));
        }

        if transaction_currency.is_empty()
            || transaction_currency.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("transaction currency is empty"));
        }
        // transaction_currency has a length of 3 characters
        else if transaction_currency.trim().len() == 3 {
            // transaction_currency is valid
        } else {
            return Err(String::from("transaction currency has invalid length"));
        }

        if _narration.is_empty() || _narration.replace(" ", "").trim().len() == 0 {
            return Err(String::from("narration is empty"));
        }
        // _narration has a max length of 100 characters i.e our own max length
        else if _narration.trim().len() > 0 && _narration.trim().len() <= 100 {
            // _narration is valid
        } else {
            return Err(String::from("narration has invalid length"));
        }

        Ok(Self {
            account_number,
            _amount,
            transaction_currency,
            _narration,
        })
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_transaction_currency(&self) -> String {
        let transaction_currency = &self.transaction_currency;
        transaction_currency.to_string()
    }

    pub fn get_narration(&self) -> String {
        let _narration = &self._narration;
        _narration.to_string()
    }
}

#[derive(Debug)]
pub struct AccountDestinationDetails {
    reference_number: String,
    account_number: String,
    _amount: u32,
    transaction_currency: String,
    _narration: String,
}

impl AccountDestinationDetails {
    pub fn new(
        reference_number: String,
        account_number: String,
        _amount: u32,
        transaction_currency: String,
        _narration: String,
    ) -> Result<Self, String> {
        if reference_number.is_empty() || reference_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("reference number is empty"));
        }
        // reference_number has a max length of 27 characters i.e our own max length
        else if reference_number.trim().len() > 0 && reference_number.trim().len() <= 27 {
            // reference_number is valid
        } else {
            return Err(String::from("reference number has invalid length"));
        }

        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a length of 14 characters
        else if account_number.trim().len() == 14 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        if _amount == 0 {
            return Err(String::from("amount has invalid value"));
        }

        if transaction_currency.is_empty()
            || transaction_currency.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("transaction currency is empty"));
        }
        // transaction_currency has a length of 3 characters
        else if transaction_currency.trim().len() == 3 {
            // transaction_currency is valid
        } else {
            return Err(String::from("transaction currency has invalid length"));
        }

        if _narration.is_empty() || _narration.replace(" ", "").trim().len() == 0 {
            return Err(String::from("narration is empty"));
        }
        // _narration has a max length of 100 characters i.e our own max length
        else if _narration.trim().len() > 0 && _narration.trim().len() <= 100 {
            // _narration is valid
        } else {
            return Err(String::from("narration has invalid length"));
        }

        Ok(Self {
            reference_number,
            account_number,
            _amount,
            transaction_currency,
            _narration,
        })
    }

    pub fn get_reference_number(&self) -> String {
        let reference_number = &self.reference_number;
        reference_number.to_string()
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_transaction_currency(&self) -> String {
        let transaction_currency = &self.transaction_currency;
        transaction_currency.to_string()
    }

    pub fn get_narration(&self) -> String {
        let _narration = &self._narration;
        _narration.to_string()
    }
}

#[derive(Debug)]
pub struct FundsTransferInputDetails {
    message_reference: String,
    callback_url: String,
    _source: AccountSourceDetails,
    _destinations: Vec<AccountDestinationDetails>,
}

impl FundsTransferInputDetails {
    pub fn new(
        message_reference: String,
        callback_url: String,
        _source: AccountSourceDetails,
        _destinations: Vec<AccountDestinationDetails>,
    ) -> Result<Self, String> {
        if message_reference.is_empty() || message_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message reference is empty"));
        }
        // message_reference has a max length of 27 characters
        else if message_reference.trim().len() > 0 && message_reference.trim().len() <= 27 {
            // message_reference is valid
        } else {
            return Err(String::from("message reference has invalid length"));
        }

        if callback_url.is_empty() || callback_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("callback url is empty"));
        }

        if _destinations.len() == 0 {
            return Err(String::from("destinations is empty"));
        }

        Ok(Self {
            message_reference,
            callback_url,
            _source,
            _destinations,
        })
    }

    pub fn get_message_reference(&self) -> String {
        let message_reference = &self.message_reference;
        message_reference.to_string()
    }

    pub fn get_callback_url(&self) -> String {
        let callback_url = &self.callback_url;
        callback_url.to_string()
    }

    pub fn get_source(&self) -> &AccountSourceDetails {
        let _source = &self._source;
        _source
    }

    pub fn get_destinations(&self) -> &Vec<AccountDestinationDetails> {
        let _destinations = &self._destinations;
        _destinations
    }
}

#[derive(Debug)]
pub struct PesalinkAccountDestinationDetails {
    reference_number: String,
    account_number: String,
    bank_code: String,
    _amount: u32,
    transaction_currency: String,
    _narration: String,
}

impl PesalinkAccountDestinationDetails {
    pub fn new(
        reference_number: String,
        account_number: String,
        bank_code: String,
        _amount: u32,
        transaction_currency: String,
        _narration: String,
    ) -> Result<Self, String> {
        if reference_number.is_empty() || reference_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("reference number is empty"));
        }
        // reference_number has a max length of 27 characters i.e our own max length
        else if reference_number.trim().len() > 0 && reference_number.trim().len() <= 27 {
            // reference_number is valid
        } else {
            return Err(String::from("reference number has invalid length"));
        }

        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a length of 14 characters
        else if account_number.trim().len() == 14 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        if bank_code.trim().len() == 0 {
            return Err(String::from("bank code is empty"));
        }

        if _amount == 0 {
            return Err(String::from("amount has invalid value"));
        }

        if transaction_currency.is_empty()
            || transaction_currency.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("transaction currency is empty"));
        }
        // transaction_currency has a length of 3 characters
        else if transaction_currency.trim().len() == 3 {
            // transaction_currency is valid
        } else {
            return Err(String::from("transaction currency has invalid length"));
        }

        if _narration.is_empty() || _narration.replace(" ", "").trim().len() == 0 {
            return Err(String::from("narration is empty"));
        }
        // _narration has a max length of 100 characters i.e our own max length
        else if _narration.trim().len() > 0 && _narration.trim().len() <= 100 {
            // _narration is valid
        } else {
            return Err(String::from("narration has invalid length"));
        }

        Ok(Self {
            reference_number,
            account_number,
            bank_code,
            _amount,
            transaction_currency,
            _narration,
        })
    }

    pub fn get_reference_number(&self) -> String {
        let reference_number = &self.reference_number;
        reference_number.to_string()
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }

    pub fn get_bank_code(&self) -> String {
        let bank_code = &self.bank_code;
        bank_code.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_transaction_currency(&self) -> String {
        let transaction_currency = &self.transaction_currency;
        transaction_currency.to_string()
    }

    pub fn get_narration(&self) -> String {
        let _narration = &self._narration;
        _narration.to_string()
    }
}

#[derive(Debug)]
pub struct FundsTransferPesalinkAccountInputDetails {
    message_reference: String,
    callback_url: String,
    _source: AccountSourceDetails,
    _destinations: Vec<PesalinkAccountDestinationDetails>,
}

impl FundsTransferPesalinkAccountInputDetails {
    pub fn new(
        message_reference: String,
        callback_url: String,
        _source: AccountSourceDetails,
        _destinations: Vec<PesalinkAccountDestinationDetails>,
    ) -> Result<Self, String> {
        if message_reference.is_empty() || message_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message reference is empty"));
        }
        // message_reference has a max length of 27 characters
        else if message_reference.trim().len() > 0 && message_reference.trim().len() <= 27 {
            // message_reference is valid
        } else {
            return Err(String::from("message reference has invalid length"));
        }

        if callback_url.is_empty() || callback_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("callback url is empty"));
        }

        if _destinations.len() == 0 {
            return Err(String::from("destinations is empty"));
        }

        Ok(Self {
            message_reference,
            callback_url,
            _source,
            _destinations,
        })
    }

    pub fn get_message_reference(&self) -> String {
        let message_reference = &self.message_reference;
        message_reference.to_string()
    }

    pub fn get_callback_url(&self) -> String {
        let callback_url = &self.callback_url;
        callback_url.to_string()
    }

    pub fn get_source(&self) -> &AccountSourceDetails {
        let _source = &self._source;
        _source
    }

    pub fn get_destinations(&self) -> &Vec<PesalinkAccountDestinationDetails> {
        let _destinations = &self._destinations;
        _destinations
    }
}

#[derive(Debug)]
pub struct PesalinkPhoneDestinationDetails {
    reference_number: String,
    phone_number: String,
    _amount: u32,
    transaction_currency: String,
    _narration: String,
}

impl PesalinkPhoneDestinationDetails {
    pub fn new(
        reference_number: String,
        phone_number: String,
        _amount: u32,
        transaction_currency: String,
        _narration: String,
    ) -> Result<Self, String> {
        if reference_number.is_empty() || reference_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("reference number is empty"));
        }
        // reference_number has a max length of 27 characters i.e our own max length
        else if reference_number.trim().len() > 0 && reference_number.trim().len() <= 27 {
            // reference_number is valid
        } else {
            return Err(String::from("reference number has invalid length"));
        }

        if phone_number.is_empty() || phone_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("phone number is empty"));
        }
        // phone_number has a length of 10 characters
        else if phone_number.trim().len() == 10 {
            // phone_number is valid
        } else {
            return Err(String::from(
                "account number has invalid length. expected 07xxxxxxxx",
            ));
        }

        if _amount == 0 {
            return Err(String::from("amount has invalid value"));
        }

        if transaction_currency.is_empty()
            || transaction_currency.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("transaction currency is empty"));
        }
        // transaction_currency has a length of 3 characters
        else if transaction_currency.trim().len() == 3 {
            // transaction_currency is valid
        } else {
            return Err(String::from("transaction currency has invalid length"));
        }

        if _narration.is_empty() || _narration.replace(" ", "").trim().len() == 0 {
            return Err(String::from("narration is empty"));
        }
        // _narration has a max length of 100 characters i.e our own max length
        else if _narration.trim().len() > 0 && _narration.trim().len() <= 100 {
            // _narration is valid
        } else {
            return Err(String::from("narration has invalid length"));
        }

        Ok(Self {
            reference_number,
            phone_number,
            _amount,
            transaction_currency,
            _narration,
        })
    }

    pub fn get_reference_number(&self) -> String {
        let reference_number = &self.reference_number;
        reference_number.to_string()
    }

    pub fn get_phone_number(&self) -> String {
        let phone_number = &self.phone_number;
        phone_number.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_transaction_currency(&self) -> String {
        let transaction_currency = &self.transaction_currency;
        transaction_currency.to_string()
    }

    pub fn get_narration(&self) -> String {
        let _narration = &self._narration;
        _narration.to_string()
    }
}

#[derive(Debug)]
pub struct FundsTransferPesalinkPhoneInputDetails {
    message_reference: String,
    callback_url: String,
    _source: AccountSourceDetails,
    _destinations: Vec<PesalinkPhoneDestinationDetails>,
}

impl FundsTransferPesalinkPhoneInputDetails {
    pub fn new(
        message_reference: String,
        callback_url: String,
        _source: AccountSourceDetails,
        _destinations: Vec<PesalinkPhoneDestinationDetails>,
    ) -> Result<Self, String> {
        if message_reference.is_empty() || message_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message reference is empty"));
        }
        // message_reference has a max length of 27 characters
        else if message_reference.trim().len() > 0 && message_reference.trim().len() <= 27 {
            // message_reference is valid
        } else {
            return Err(String::from("message reference has invalid length"));
        }

        if callback_url.is_empty() || callback_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("callback url is empty"));
        }

        if _destinations.len() == 0 {
            return Err(String::from("destinations is empty"));
        }

        Ok(Self {
            message_reference,
            callback_url,
            _source,
            _destinations,
        })
    }

    pub fn get_message_reference(&self) -> String {
        let message_reference = &self.message_reference;
        message_reference.to_string()
    }

    pub fn get_callback_url(&self) -> String {
        let callback_url = &self.callback_url;
        callback_url.to_string()
    }

    pub fn get_source(&self) -> &AccountSourceDetails {
        let _source = &self._source;
        _source
    }

    pub fn get_destinations(&self) -> &Vec<PesalinkPhoneDestinationDetails> {
        let _destinations = &self._destinations;
        _destinations
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AccountData {
    pub MessageReference: String,
    pub AccountNumber: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AccountFullStatementData {
    pub MessageReference: String,
    pub AccountNumber: String,
    pub StartDate: String,
    pub EndDate: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AccountTransactionsData {
    pub MessageReference: String,
    pub AccountNumber: String,
    pub NoOfTransactions: u16,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AccountSourceData {
    pub AccountNumber: String,
    pub Amount: u32,
    pub TransactionCurrency: String,
    pub Narration: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AccountDestinationData {
    pub ReferenceNumber: String,
    pub AccountNumber: String,
    pub Amount: u32,
    pub TransactionCurrency: String,
    pub Narration: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct FundsTransferData {
    pub MessageReference: String,
    pub CallBackUrl: String,
    pub Source: AccountSourceData,
    pub Destinations: Vec<AccountDestinationData>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct PesalinkAccountDestinationData {
    pub ReferenceNumber: String,
    pub AccountNumber: String,
    pub BankCode: String,
    pub Amount: u32,
    pub TransactionCurrency: String,
    pub Narration: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct FundsTransferPesalinkAccountData {
    pub MessageReference: String,
    pub CallBackUrl: String,
    pub Source: AccountSourceData,
    pub Destinations: Vec<PesalinkAccountDestinationData>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct PesalinkPhoneDestinationData {
    pub ReferenceNumber: String,
    pub PhoneNumber: String,
    pub Amount: u32,
    pub TransactionCurrency: String,
    pub Narration: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct FundsTransferPesalinkPhoneData {
    pub MessageReference: String,
    pub CallBackUrl: String,
    pub Source: AccountSourceData,
    pub Destinations: Vec<PesalinkPhoneDestinationData>,
}

// response data

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MixedTypeValue {
    IntegerValue(i32),
    FloatValue(f32),
}

#[derive(Deserialize, Debug)]
pub struct AuthTokenResponseData {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<u32>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountBalanceResponseData {
    pub MessageReference: Option<String>,
    pub MessageDateTime: Option<String>,
    pub MessageCode: Option<String>,
    pub MessageDescription: Option<String>,
    pub AccountNumber: Option<String>,
    pub AccountName: Option<String>,
    pub Currency: Option<String>,
    pub ProductName: Option<String>,
    pub ClearedBalance: MixedTypeValue,
    pub BookedBalance: MixedTypeValue,
    pub BlockedBalance: MixedTypeValue,
    pub AvailableBalance: MixedTypeValue,
    pub ArrearsAmount: MixedTypeValue,
    pub BranchName: Option<String>,
    pub BranchSortCode: Option<String>,
    pub AverageBalance: MixedTypeValue,
    pub UnclearedBalance: MixedTypeValue,
    pub ODLimit: MixedTypeValue,
    pub CreditLimit: MixedTypeValue,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountValidationResponseData {
    pub MessageReference: Option<String>,
    pub MessageDateTime: Option<String>,
    pub MessageCode: Option<String>,
    pub MessageDescription: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TransactionsData {
    pub TransactionId: Option<String>,
    pub TransactionDate: Option<String>,
    pub ValueDate: Option<String>,
    pub Narration: Option<String>,
    pub TransactionType: Option<String>,
    pub ServicePoint: Option<String>,
    pub TransactionReference: Option<String>,
    pub CreditAmount: MixedTypeValue,
    pub DebitAmount: MixedTypeValue,
    pub RunningClearedBalance: MixedTypeValue,
    pub RunningBookBalance: MixedTypeValue,
    pub DebitLimit: MixedTypeValue,
    pub LimitExpiryDate: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountStatementResponseData {
    pub MessageReference: Option<String>,
    pub MessageDateTime: Option<String>,
    pub MessageCode: Option<String>,
    pub MessageDescription: Option<String>,
    pub AccountNumber: Option<String>,
    pub AccountName: Option<String>,
    pub Transactions: TransactionsData,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountTransactionsResponseData {
    pub MessageReference: Option<String>,
    pub MessageDateTime: Option<String>,
    pub MessageCode: Option<String>,
    pub MessageDescription: Option<String>,
    pub AccountNumber: Option<String>,
    pub AccountName: Option<String>,
    pub NoOfTransactions: u16,
    pub TotalCredits: MixedTypeValue,
    pub TotalDebits: MixedTypeValue,
    pub Transactions: TransactionsData,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountFundsTransferResponseData {
    pub MessageReference: Option<String>,
    pub MessageDateTime: Option<String>,
    pub MessageCode: Option<String>,
    pub MessageDescription: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct BadRequestErrorResponseData {
    pub MessageReference: Option<String>,
    pub MessageDateTime: Option<String>,
    pub MessageCode: Option<String>,
    pub MessageDescription: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UnauthorizedErrorData {
    pub code: Option<String>,
    pub message: Option<String>,
    pub description: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UnauthorizedErrorResponseData {
    pub fault: UnauthorizedErrorData,
}
