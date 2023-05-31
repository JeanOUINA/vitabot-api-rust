# Client
```rs
#[tokio::main]
async fn main(){
    let mut client = vitabot_api::Client::new("Your api key from vitamin.tips".to_string());
    
    let token1 = client.get_token("VITC".to_string()).await.unwrap();
    let token2 = client.get_token("VITC-000".to_string()).await.unwrap();
    let token3 = client.get_token("BAN-001".to_string()).await.unwrap();

    let amount1 = client.parse_amount("1 vitc".to_string()).await.unwrap();
    let amount2 = client.parse_amount("$10 vitc".to_string()).await.unwrap();
    let amount2 = client.parse_amount("129k VITE".to_string()).await.unwrap();

    // in a discord bot
    let recipient = client.get_discord_user_address("696481194443014174".to_string()).await.unwrap(); // 696481194443014174 is your user id
    let user_id = client.resolve_discord_user_from_address(recipient.clone()).await.unwrap();

    let addresses = client.get_addresses().await.unwrap();
    let balances = client.get_balances().await.unwrap();
    let balance1 = client.get_balance(vitabot_api::BankAccount::Index(0)).await.unwrap();
    let balance2 = client.get_balance(vitabot_api::BankAccount::Address("vite_xxxxxx".to_string())).await.unwrap();
    let address = client.new_address().await.unwrap();
    let transaction = client.send_transaction(vitabot_api::TransactionRequest {
        from: vitabot_api::BankAccount::Index(0),
        to: recipient,
        amount: amount1.amount,
        token_id: amount1.token_id,
        data: vec![] // empty data field
    });
}
```

# Discord Faucet Example
> This example was extended from the [Serenity Example Bot](https://github.com/serenity-rs/serenity/tree/current#example-bot).
```rs
use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[macro_use]
extern crate lazy_static;

#[group]
#[commands(faucet)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

lazy_static! {
    static ref VITABOT:vitabot_api::Client = {
        let key = env::var("VITABOT_KEY").expect("VITABOT_KEY");
        vitabot_api::Client::new(key)
    };
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn faucet(ctx: &Context, msg: &Message) -> CommandResult {
    let amount = VITABOT.parse_amount("1 vitc".to_string()).await.unwrap();
    let addresses = VITABOT.get_addresses().await.unwrap();
    let recipient = VITABOT.get_discord_user_address(msg.author.id.0.to_string()).await.unwrap();
    let transaction = VITABOT.send_transaction(vitabot_api::TransactionRequest {
        from: vitabot_api::BankAccount::Address(addresses[0].address.clone()),
        to: recipient,
        amount: amount.amount,
        token_id: amount.token_id,
        data: vec![]
    }).await.unwrap();

    msg.reply_ping(&ctx.http, format!("https://vitcscan.com/tx/{}", transaction.hash)).await?;

    Ok(())
}
```
