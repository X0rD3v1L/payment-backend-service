use std::time::Duration;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};
use sea_orm::*;
use serde::Deserialize;

use crate::entities::txns::{self, Entity as Txns};

#[derive(Debug, Deserialize)]
pub struct TransactionKafkaPayload {
    pub txn_id: String,
    pub account_id: String,
    pub amount: f64,
    pub txn_type: String
}

pub async fn start(db: DatabaseConnection) {
    let consumer: StreamConsumer = create();
    consume(consumer, db).await;
}

fn create() -> StreamConsumer {
    let mut binding = ClientConfig::new();
    let config = binding
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .set("group.id", "test-group")
        .set("socket.timeout.ms", "4000");

    config.create().expect("Failed to create consumer")
}

async fn consume(consumer: StreamConsumer, db: DatabaseConnection) {
    consumer
        .subscribe(&["transaction-events"])
        .expect("cannot subscribe");

    loop {
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {:?}", e),
            Ok(message) => {
                match message.payload_view::<str>() {
                    None => println!("⚠️ Received empty message"),
                    Some(Ok(msg)) => {
                        println!("Message consumed: {}", msg);
                        if let Ok(payload) = serde_json::from_str::<TransactionKafkaPayload>(msg) {
                            tokio::spawn(handle_transaction(payload, db.clone()));
                        } else {
                            println!("Failed to deserialize message");
                        }
                    }
                    Some(Err(e)) => println!("Payload error: {:?}", e),
                }
                consumer
                    .commit_message(&message, CommitMode::Async)
                    .unwrap();
            }
        }
    }
}

async fn handle_transaction(txn: TransactionKafkaPayload, db: DatabaseConnection) {
    println!("Processing transaction: {}", txn.txn_id);

    tokio::time::sleep(Duration::from_secs(2)).await;

    let txn_result = Txns::find()
        .filter(txns::Column::TxnId.eq(txn.txn_id.clone()))
        .one(&db)
        .await;

    if let Err(e) = txn_result {
        eprintln!("DB query error for txn_id {}: {}", txn.txn_id, e);
        return;
    }

    let txn_model = match txn_result.unwrap() {
        Some(txn) => txn,
        None => {
            eprintln!("No transaction found in DB for txn_id: {}", txn.txn_id);
            return;
        }
    };

    if txn.txn_type == "purchase" {
        use crate::entities::account::{self, Entity as Accounts};

        let account_opt = Accounts::find()
            .filter(account::Column::AccountId.eq(txn.account_id.clone()))
            .one(&db)
            .await;

        match account_opt {
            Ok(Some(account_model)) => {
                let current_balance = account_model.balance;
                if current_balance < txn.amount as f32 {
                    let mut txn_am: txns::ActiveModel = txn_model.into();
                    txn_am.status = Set("failed".to_string());

                    if let Err(e) = txn_am.update(&db).await {
                        eprintln!("Failed to update transaction {}: {}", txn.txn_id, e);
                    } else {
                        println!("Transaction {} failed due to insufficient balance", txn.txn_id);
                    }
                    return;
                }
            }
            Ok(None) => {
                eprintln!("Account {} not found", txn.account_id);
                return;
            }
            Err(e) => {
                eprintln!("DB query error for account {}: {}", txn.account_id, e);
                return;
            }
        }
    }

    
    let new_status = if std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() % 2 == 0 
    {
        "success"
    } else {
        "failed"
    };


    let mut txn_am: txns::ActiveModel = txn_model.into();
    txn_am.status = Set(new_status.to_string());

    if let Err(e) = txn_am.update(&db).await {
        eprintln!("Failed to update transaction {}: {}", txn.txn_id, e);
    } else {
        println!("Transaction {} marked as {}", txn.txn_id, new_status);

        if new_status == "success" {
            if let Err(e) = update_account_balance(&txn, &db).await {
                eprintln!("Failed to update balance for account {}: {}", txn.account_id, e);
            } else {
                println!("Balance updated for account {}", txn.account_id);
            }
        }
    }
}

async fn update_account_balance(
    txn: &TransactionKafkaPayload,
    db: &DatabaseConnection,
) -> Result<(), sea_orm::DbErr> {
    use crate::entities::account::{self, Entity as Accounts};

    if let Some(account_model) = Accounts::find()
        .filter(account::Column::AccountId.eq(txn.account_id.clone()))
        .one(db)
        .await?
    {
        let mut account_am = account_model.into_active_model();
        let current_balance = account_am.balance.unwrap(); // Extract f32 from ActiveValue

        let updated_balance = match txn.txn_type.as_str() {
            "credit" => current_balance + txn.amount as f32,
            "purchase" => current_balance - txn.amount as f32,
            _ => current_balance,
        };

        account_am.balance = Set(updated_balance);
        account_am.update(db).await?;
    } else {
        eprintln!("Account {} not found to update balance", txn.account_id);
    }

    Ok(())
}