use futures::future::FutureExt;
use futures::stream::{StreamExt, TryStreamExt};
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use minerva_broker::model::SessionMessage;
use minerva_broker::LapinPool;
use minerva_data::db::DBPool;
use minerva_rpc as rpc;
use mongodb::bson::{doc, Document};
use mongodb::Client as MongoClient;
use mongodb::Database as MongoDatabase;
use redis::Client as RedisClient;
use tokio::time::Duration;

const QUEUES: &'static [&str] = &["session_management"];

const QUEUE_PROCESSING_WAIT_SECS: u64 = 3;

pub async fn queue_consume(
    tenant: String,
    rabbitmq: LapinPool,
    _postgresql: DBPool,
    mongodb: MongoClient,
    _redis: RedisClient,
) {
    let mut handlers = vec![];
    for queue in QUEUES {
        let tenant = tenant.clone();
        let rabbitmq = rabbitmq.clone();
        let mongodb = mongodb.clone();
        handlers.push(tokio::spawn(async move {
            loop {
                let consumer_name = format!("{}_{}_consumer", tenant, queue);
                let conn = rabbitmq.get().await.expect(&format!(
                    "{}: Unable to retrieve RabbitMQ connection",
                    consumer_name
                ));
                let channel = conn.create_channel().await.expect(&format!(
                    "{}: Unable to open RabbitMQ channel",
                    consumer_name
                ));
                let mut consumer = channel
                    .basic_consume(
                        queue,
                        &consumer_name,
                        BasicConsumeOptions::default(),
                        FieldTable::default(),
                    )
                    .await
                    .expect(&format!(
                        "{}: Unable to create queue consumer",
                        consumer_name
                    ));

                while let Some(delivery) = consumer.next().await {
                    let delivery = delivery.expect(&format!(
                        "{}: Error while delivering message",
                        consumer_name
                    ));

                    match queue {
                        &"session_management" => {
                            // `SESSION` already implements session removal, including
                            // un-caching.
                            // We should dispatch to that instead of relying on direct
                            // database operations. It doesn't matter that we still have
                            // access to connections to other services, since those
                            // clients are asynchronous in nature and will only open
                            // connections on demand.

                            let message = SessionMessage::from(
                                std::str::from_utf8(&delivery.data).unwrap().to_string(),
                            );

                            match message {
                                SessionMessage::Remove { user } => {
                                    println!(
                                        "{}: Remove session from user \"{}\"",
                                        consumer_name, user
                                    );
                                    remove_user_sessions(
                                        &mongodb.database(&tenant),
                                        &tenant,
                                        &user,
                                    )
                                    .await;
                                }
                            }

                            delivery
                                .ack(BasicAckOptions::default())
                                .await
                                .expect(&format!("{}: Unable to ACK delivery", consumer_name));
                        }
                        _ => {}
                    }
                }

                // End of queue processing. Let's wait for a while.
                println!("{}: Processed.", consumer_name);
                tokio::time::sleep(Duration::from_secs(QUEUE_PROCESSING_WAIT_SECS)).await;
            }
        }))
    }

    for handle in handlers {
        let _ = handle.catch_unwind().await;
    }
}

pub async fn remove_user_sessions(mongodb: &MongoDatabase, tenant: &str, username: &str) {
    use tonic::Request;

    // 1. Fetch all sessions from the given user;
    let collection = mongodb.collection::<Document>("session");
    let filter = doc! { "login": username };
    let mut cursor = collection
        .find(filter, None)
        .await
        .expect("Error while fetching documents");

    let mut sessions = vec![];
    while let Some(document) = cursor.try_next().await.unwrap() {
        sessions.push(rpc::messages::SessionToken {
            token: base64::encode(
                document
                    .get("_id")
                    .unwrap()
                    .as_object_id()
                    .unwrap()
                    .to_hex(),
            ),
        });
    }

    println!("Removing {} sessions.", sessions.len());

    // 2. Ask `SESSION` to kill them all.
    let server =
        std::env::var("SESSION_SERVICE_SERVER").expect("Unable to read SESSION_SERVICE_SERVER");
    let port = std::env::var("SESSION_SERVICE_PORT").expect("Unable to read SESSION_SERVICE_PORT");
    let endpoint = format!("http://{}:{}", server, port);

    let mut client = rpc::session::make_client(endpoint, tenant.into(), "DISPATCH".into())
        .await
        .expect("Unable to connect to `SESSION` service");

    for session in sessions {
        client.remove(Request::new(session)).await.unwrap();
    }
}
