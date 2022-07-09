use futures::future::FutureExt;
use futures::stream::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use minerva_broker::LapinPool;
use minerva_data::db::DBPool;
use mongodb::Client as MongoClient;
use redis::Client as RedisClient;
use tokio::time::Duration;

const QUEUES: &'static [&str] = &["session_management"];

const QUEUE_PROCESSING_WAIT_SECS: u64 = 3;

pub async fn queue_consume(
    tenant: String,
    rabbitmq: LapinPool,
    postgresql: DBPool,
    mongodb: MongoClient,
    redis: RedisClient,
) {
    let mut handlers = vec![];
    for queue in QUEUES {
        let tenant = tenant.clone();
        let rabbitmq = rabbitmq.clone();
        handlers.push(tokio::spawn(async move {
            loop {
                let consumer_name = format!("{}_{}_consumer", tenant, queue);
                let conn = rabbitmq.get().await.expect(&format!(
                    "{}: Unable to retrieve RabbitMQ connection",
                    consumer_name
                ));
                let mut channel = conn.create_channel().await.expect(&format!(
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

                    // TODO
                    println!(
                        "{}: Received: {}",
                        consumer_name,
                        std::str::from_utf8(&delivery.data).unwrap()
                    );

                    delivery
                        .ack(BasicAckOptions::default())
                        .await
                        .expect(&format!("{}: Unable to ACK delivery", consumer_name));
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
