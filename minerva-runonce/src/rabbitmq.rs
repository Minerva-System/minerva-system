//! This module wraps all functions related to operations that should be run
//! once to prepare the message broker, when the entire system starts.

use lapin::{options::QueueDeclareOptions, types::FieldTable};
use minerva_broker as broker;

/// Creates a virtual host for the current tenant on the message broker.
pub async fn create_virtual_host(
    tenant: &str,
    host: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}: Creating RabbitMQ virtual host...", tenant);
    match broker::make_virtual_host(host, tenant).await? {
        None => panic!(
            "Unable to create RabbitMQ virtual host for tenant {}",
            tenant
        ),
        Some(result) => {
            if result {
                println!("{}: RabbitMQ virtual host successfully created.", tenant);
            } else {
                println!("{}: RabbitMQ virtual host already exists.", tenant);
            }
        }
    }
    Ok(())
}

/// Awaits for message broker availability on a spinlock.
pub async fn broker_spinlock(host: &str) {
    while !broker::check_virtual_host(host)
        .await
        .expect("Could not query broker")
    {
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    }
}

/// Create default queues for a given virtual host on the message broker.
pub async fn create_default_queues(
    tenant: &str,
    host: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}: Creating default RabbitMQ queues...", tenant);

    let connection = broker::make_connection(host, Some(tenant)).await?;
    let channel = connection.create_channel().await?;

    for queue in broker::QUEUES {
        let options = QueueDeclareOptions {
            durable: true,
            ..QueueDeclareOptions::default()
        };

        let _ = channel
            .queue_declare(queue, options, FieldTable::default())
            .await?;
        println!("{}: Durable queue \"{}\" created.", tenant, queue);
    }

    Ok(())
}
