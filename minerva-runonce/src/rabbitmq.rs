use minerva_broker as broker;

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

pub async fn broker_spinlock(host: &str) {
    while !broker::check_virtual_host(host)
        .await
        .expect("Could not query broker")
    {
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    }
}
