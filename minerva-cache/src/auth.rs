use core::future::Future;
use redis::AsyncCommands;
use redis::Client;
use redis::RedisResult;

/// Session data expiration time. Counts exactly one day.
const EXPIRATION: usize = 24 * 60 * 60;

fn gen_session_key(tenant: &str, token: &str) -> String {
    let clean_info = |info: &str| info.replace("$", "-").replace(":", "_");
    format!("{}$SESSION:{}", clean_info(tenant), clean_info(token))
}

pub async fn get_session(client: &Client, tenant: &str, token: &str) -> RedisResult<String> {
    let key = gen_session_key(tenant, token);
    let mut conn = client.get_async_connection().await?;
    conn.get(key).await
}

pub fn save_session(client: &Client, tenant: &str, token: &str, json: &str) -> RedisResult<()> {
    let key = gen_session_key(tenant, token);
    let mut conn = client.get_connection()?;
    redis::transaction(&mut conn, &[key.clone()], |conn, pipe| {
        pipe.set(key.clone(), json)
            .expire(key.clone(), EXPIRATION)
            .query(conn)
    })
}

pub async fn remove_session(client: &Client, tenant: &str, token: &str) -> RedisResult<()> {
    let key = gen_session_key(tenant, token);
    let mut conn = client.get_async_connection().await?;
    conn.del(key).await
}

#[cfg(test)]
mod tests {
    use super::*;

    // Salvamento e remoção normal
    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn save_then_remove() {
        let server = "localhost:6379";
        let token = "save_then_remove";
        let json = "{\"foo\": \"bar\"}";
        let tenant = "teste";

        let client = crate::build_client(server).unwrap();

        // Remove session before anything. Should not error.
        // Running it thrice as a guarantee.
        remove_session(&client, tenant, token).await.unwrap();
        remove_session(&client, tenant, token).await.unwrap();
        remove_session(&client, tenant, token).await.unwrap();

        // Make sure session does not exist on server
        assert!(get_session(&client, tenant, token).await.is_err());

        // Save session, expect non-failure result
        let _ = save_session(&client, tenant, token, json).unwrap();

        // Fetch saved JSON and expect it to be equals to what we sent
        let json_return = get_session(&client, tenant, token).await.unwrap();
        assert_eq!(json, json_return);

        // Remove session expecting success
        let _ = remove_session(&client, tenant, token).await.unwrap();
    }
}
