use std::collections::HashMap;

use hex::FromHexError;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::utils::{parse_transaction, bank_account_to_id};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIErrorAnswer {
    pub error: APIError
}
#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    pub message: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveUserFromAddressAnswer {
    pub id: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveAddressFromUserAnswer {
    pub address: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub decimals: u8,
    pub token_id: String,
    pub name: String,
    pub currency: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub decimals: u8,
    pub token_id: String,
    pub name: String,
    pub currency: String,
    pub amount: String,
    pub amount_display: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub address: String,
    pub index: u32
}
pub type Balances = HashMap<String, String>;
#[derive(Debug, Serialize, Deserialize)]
pub struct GetBalancesAnswer {
    pub address: String,
    pub index: u32,
    pub balances: Balances
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    Send,
    Receive,
    Unknown
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub from: String,
    pub to: String,
    pub hash: String,
    pub amount: String,
    pub token_id: String,
    pub sender_handle: String,
    pub data: Vec<u8>
}
#[derive(Debug)]
pub struct TransactionRequest {
    pub from: BankAccount,
    pub to: String,
    pub amount: String,
    pub token_id: String,
    pub data: Vec<u8>
}
#[derive(Debug)]
pub enum BankAccount {
    Address(String),
    Index(u32)
}

#[derive(Debug)]
pub enum HttpError {
    Reqwest(reqwest::Error),
    API(APIError),
    Hex(FromHexError)
}
impl From<reqwest::Error> for HttpError {
    fn from(err: reqwest::Error) -> HttpError {
        HttpError::Reqwest(err)
    }
}
impl From<APIErrorAnswer> for HttpError {
    fn from(err: APIErrorAnswer) -> HttpError {
        HttpError::API(err.error)
    }
}
impl From<FromHexError> for HttpError {
    fn from(err: FromHexError) -> HttpError {
        HttpError::Hex(err)
    }
}

#[derive(Debug)]
pub struct Client {
    pub key: String,
    pub base_url: String,
    client: reqwest::Client
}

impl Client {
    pub fn new(key: String) -> Client {
        Client {
            key,
            base_url: "https://vitamin.tips/api".to_string(),
            client: reqwest::Client::new()
        }
    }

    pub async fn get(&self, path: String) -> Result<reqwest::Response, HttpError> {
        let res = self.client.get(self.base_url.to_owned() + path.as_str())
            .header("Authorization", self.key.clone())
            .send()
            .await?;
        if !res.status().is_success() {
            let body = res.json::<APIErrorAnswer>().await?;
            return Err(body.into());
        }

        Ok(res)
    }
    pub async fn post(&self, path: String, body: Value) -> Result<reqwest::Response, HttpError> {
        let res = self.client.post(self.base_url.to_owned() + path.as_str())
            .header("Authorization", self.key.clone())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        if !res.status().is_success() {
            let body = res.json::<APIErrorAnswer>().await?;
            return Err(body.into());
        }

        Ok(res)
    }

    pub async fn resolve_discord_user_from_address(&self, address: String) -> Result<String, HttpError> {
        let res = self.get("/address/resolve/discord/".to_owned() + address.as_str())
            .await?;
        let body = res.json::<ResolveUserFromAddressAnswer>().await?;
        Ok(body.id)
    }

    pub async fn get_discord_user_address(&self, id: String) -> Result<String, HttpError> {
        let res = self.get("/address/discord/".to_owned() + id.as_str())
            .await?;
        let body = res.json::<ResolveAddressFromUserAnswer>().await?;
        Ok(body.address)
    }

    pub async fn get_token(&self, ticker: String) -> Result<Token, HttpError> {
        let res = self.post("/vite/get_token".to_owned(), serde_json::json!({
            "ticker": ticker
        }))
            .await?;
        let body = res.json::<Token>().await?;
        Ok(body)
    }

    pub async fn parse_amount(&self, amount: String) -> Result<Amount, HttpError> {
        let res = self.post("/vite/parse_amount".to_owned(), serde_json::json!({
            "amount": amount
        }))
            .await?;
        let body = res.json::<Amount>().await?;
        Ok(body)
    }

    pub async fn get_addresses(&self) -> Result<Vec<Address>, HttpError> {
        let res = self.get("/bank/addresses".to_owned())
            .await?;
        let body = res.json::<Vec<String>>().await?;
        let mut addresses = Vec::new();
        for (i, address) in body.iter().enumerate() {
            addresses.push(Address {
                address: address.to_string(),
                index: i as u32
            });
        }
        Ok(addresses)
    }

    pub async fn get_balances(&self) -> Result<Vec<GetBalancesAnswer>, HttpError> {
        let res = self.get("/bank/balances".to_owned())
            .await?;
        let body = res.json::<Vec<GetBalancesAnswer>>().await?;
        Ok(body)
    }

    pub async fn get_balance(&self, account: BankAccount) -> Result<Balances, HttpError> {
        let id = match account {
            BankAccount::Address(address) => address.to_string(),
            BankAccount::Index(index) => index.to_string()
        };
        let res = self.get("/bank/balances/".to_owned() + id.as_str())
            .await?;
        let body = res.json::<Balances>().await?;
        Ok(body)
    }
    
    pub async fn new_address(&self) -> Result<Address, HttpError> {
        let res = self.post("/bank/addresses/new".to_owned(), serde_json::json!({}))
            .await?;
        let body = res.json::<Address>().await?;
        Ok(body)
    }

    pub async fn send_transaction(&self, transaction: TransactionRequest) -> Result<Transaction, HttpError> {
        let id = bank_account_to_id(transaction.from);
        let data = serde_json::json!({
            "to": transaction.to,
            "tokenId": transaction.token_id,
            "amount": transaction.amount,
            "data": hex::encode(transaction.data)
        });
        let res = self.post("/bank/send/".to_owned() + id.as_str(), data.clone())
            .await?;

        let body = res.json().await?;
        Ok(parse_transaction(body)?)
    }
}