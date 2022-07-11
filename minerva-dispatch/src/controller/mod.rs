use crate::error::DispatchError;
use futures::stream::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use minerva_broker as broker;
use minerva_broker::LapinPool;
use minerva_data::db::DBPool;
use mongodb::Client as MongoClient;
use redis::Client as RedisClient;
use tokio::time::Duration;

mod session_management;

const QUEUE_PROCESSING_WAIT_SECS: u64 = 3;

pub async fn queue_consume(
    tenant: String,
    rabbitmq: LapinPool,
    _postgresql: DBPool,
    mongodb: MongoClient,
    _redis: RedisClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut handlers = vec![];
    for queue in broker::QUEUES {
        let tenant = tenant.clone();
        let rabbitmq = rabbitmq.clone();
        let mongodb = mongodb.clone();
        handlers.push(tokio::spawn(async move {
            loop {
                let consumer_name = format!("{}.{}.consumer", tenant, queue);

                let conn = rabbitmq
                    .get()
                    .await
                    .map_err(|_| DispatchError::ConnectionError {
                        consumer_name: consumer_name.clone(),
                        service_name: "RabbitMQ".to_string(),
                    })
                    .unwrap();

                let channel = conn
                    .create_channel()
                    .await
                    .map_err(|_| DispatchError::ConnectionError {
                        consumer_name: consumer_name.clone(),
                        service_name: "RabbitMQ channel".to_string(),
                    })
                    .unwrap();

                let mut consumer = channel
                    .basic_consume(
                        queue,
                        &consumer_name,
                        BasicConsumeOptions::default(),
                        FieldTable::default(),
                    )
                    .await
                    .map_err(|_| DispatchError::ConnectionError {
                        consumer_name: consumer_name.clone(),
                        service_name: "queue using a new consumer".to_string(),
                    })
                    .unwrap();

                while let Some(delivery) = consumer.next().await {
                    let delivery = delivery
                        .map_err(|_| DispatchError::DeliveryError {
                            consumer_name: consumer_name.clone(),
                        })
                        .unwrap();

                    match queue {
                        &"session_management" => {
                            if session_management::dispatch(
                                &tenant,
                                &consumer_name,
                                &mongodb,
                                &delivery.data,
                            )
                            .await
                            .unwrap()
                            {
                                // If message is known, then we send back an
                                // ack signal. If not, well... leave it to
                                // another consumer
                                delivery
                                    .ack(BasicAckOptions::default())
                                    .await
                                    .map_err(|_| DispatchError::AckError {
                                        consumer_name: consumer_name.clone(),
                                    })
                                    .unwrap();
                            }
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
        handle.await?;
    }

    Ok(())
}
