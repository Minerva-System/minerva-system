use crate::error::DispatchError;
use futures::stream::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use log::{debug, error, info, trace};
use minerva_broker as broker;
use minerva_broker::LapinPool;
use minerva_data::db::DBPool;
use mongodb::Client as MongoClient;
use redis::Client as RedisClient;
use tokio::time::Duration;

mod session_management;

/// Number of seconds to wait for each task to reprocess
const QUEUE_PROCESSING_WAIT_SECS: u64 = 3;

/// Primary entry point for the consuming of messages from the message broker.
///
/// # Note
/// This procedure contain A LOT of `unwrap`'s. This is by design. This procedure
/// is supposed to run asynchronously for each queue that is supposed to be listened
/// to by the DISPATCH service, and by tenant as well. If any step on this fails,
/// the listener should also fail, and maybe be restarted.
pub async fn queue_consume(
    tenant: String,
    rabbitmq: LapinPool,
    _postgresql: DBPool,
    mongodb: MongoClient,
    _redis: RedisClient,
) -> Result<(), Box<dyn std::error::Error>> {
    trace!("Start queue consumer for {}", tenant);
    let mut handlers = vec![];
    for queue in broker::QUEUES {
        let tenant = tenant.clone();
        let rabbitmq = rabbitmq.clone();
        let mongodb = mongodb.clone();

        handlers.push(tokio::spawn(async move {
            let consumer_name = format!("{}.{}.consumer", tenant, queue);
            debug!("Starting consumer {}", consumer_name);
            loop {
                trace!("Begin broker connection loop");
                let conn = rabbitmq
                    .get()
                    .await
                    .map_err(|_| {
                        let error = DispatchError::Connection {
                            consumer_name: consumer_name.clone(),
                            service_name: "RabbitMQ".to_string(),
                        };
                        error!("{}", error);
                        error
                    })
                    .unwrap();

                debug!("Create channel");
                let channel = conn
                    .create_channel()
                    .await
                    .map_err(|_| {
                        let error = DispatchError::Connection {
                            consumer_name: consumer_name.clone(),
                            service_name: "RabbitMQ channel".to_string(),
                        };
                        error!("{}", error);
                        error
                    })
                    .unwrap();

                debug!("Create basic consumer");
                let mut consumer = channel
                    .basic_consume(
                        queue,
                        &consumer_name,
                        BasicConsumeOptions::default(),
                        FieldTable::default(),
                    )
                    .await
                    .map_err(|_| {
                        let error = DispatchError::Connection {
                            consumer_name: consumer_name.clone(),
                            service_name: "queue using a new consumer".to_string(),
                        };
                        error!("{}", error);
                        error
                    })
                    .unwrap();

                debug!("Await next delivery");
                while let Some(delivery) = consumer.next().await {
                    let delivery = delivery
                        .map_err(|_| {
                            let error = DispatchError::Delivery {
                                consumer_name: consumer_name.clone(),
                            };
                            error!("{}", error);
                            error
                        })
                        .unwrap();

                    debug!("Process delivered message");
                    if (queue == &"session_management")
                        && session_management::dispatch(
                            &tenant,
                            &consumer_name,
                            &mongodb,
                            &delivery.data,
                        )
                        .await
                        .unwrap()
                    {
                        trace!("Ack session_management delivery");
                        // If message is known, then we send back an
                        // ack signal. If not, well... leave it to
                        // another consumer
                        delivery
                            .ack(BasicAckOptions::default())
                            .await
                            .map_err(|_| {
                                let error = DispatchError::Ack {
                                    consumer_name: consumer_name.clone(),
                                };
                                error!("{}", error);
                                error
                            })
                            .unwrap();
                    }
                }

                // End of queue processing. Let's wait for a while.
                info!("{}: Processed.", consumer_name);
                tokio::time::sleep(Duration::from_secs(QUEUE_PROCESSING_WAIT_SECS)).await;
            }
        }))
    }

    for handle in handlers {
        handle.await?;
    }

    info!("All handlers terminated.");
    Ok(())
}
