use crate::client::{Transaction, HttpError, TransactionType, BankAccount};

pub fn parse_transaction(json: serde_json::Value) -> Result<Transaction, HttpError> {
    let transaction = Transaction {
        transaction_type: match json.get("type").unwrap().as_str().unwrap() {
            "send" => TransactionType::Send,
            "receive" => TransactionType::Receive,
            _ => TransactionType::Unknown

        },
        from: json.get("from").unwrap().as_str().unwrap().to_string(),
        to: json.get("to").unwrap().as_str().unwrap().to_string(),
        hash: json.get("hash").unwrap().as_str().unwrap().to_string(),
        amount: json.get("amount").unwrap().as_str().unwrap().to_string(),
        token_id: json.get("token_id").unwrap().as_str().unwrap().to_string(),
        sender_handle: json.get("sender_handle").unwrap().as_str().unwrap().to_string(),
        data: hex::decode(json.get("data").unwrap().as_str().unwrap().to_string())?,
    };
    Ok(transaction)
}

pub fn bank_account_to_id(account: BankAccount) -> String {
    match account {
        BankAccount::Address(address) => address.to_string(),
        BankAccount::Index(index) => index.to_string()
    }
}