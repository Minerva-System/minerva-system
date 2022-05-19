use core::time::Duration;
use minerva_data::mongo;
use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    IndexModel,
};

pub async fn database_spinlock(server: &str) {
    let client = mongo::make_client(server).await;
    let mut lock = true;
    while lock {
        lock = client
            .database("admin")
            .run_command(doc! { "ping": 1 }, None)
            .await
            .is_err();
    }
}

pub async fn prepare_database(tenant: &str, server: &str) -> Result<(), mongodb::error::Error> {
    println!("{}: Connecting to MongoDB client...", tenant);
    let client = mongo::make_client(server).await;
    println!("{}: Connecting to MongoDB database...", tenant);
    let db = client.database(tenant);

    println!("{}: Preparing MongoDB collections...", tenant);

    // Prepare Session collection
    let collection = db.collection::<Document>("session");
    collection
        .create_index(
            IndexModel::builder()
                .keys(doc! { "creationDate": 1 })
                .options(
                    IndexOptions::builder()
                        .expire_after(Duration::from_secs(mongo::SESSION_DURATION))
                        .build(),
                )
                .build(),
            None,
        )
        .await?;

    Ok(())
}
