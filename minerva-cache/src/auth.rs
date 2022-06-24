use futures::prelude::*;
use redis::aio::Connection;
use redis::AsyncCommands;
use redis::Client;
use redis::ConnectionLike;
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

pub async fn save_session(
    client: &Client,
    tenant: &str,
    token: &str,
    json: &str,
) -> RedisResult<String> {
    let key = gen_session_key(tenant, token);
    let mut conn = client.get_async_connection().await?;
    redis::transaction(&mut conn, &[key], |conn: &mut Connection, pipe| {
        pipe.set(key, json).expire(key, EXPIRATION).query(conn)
    })
}

pub async fn get_or_save_session(
    client: &Client,
    tenant: &str,
    token: &str,
    fetch: &dyn Fn() -> String,
) -> RedisResult<String> {
    let result = get_session(client, tenant, token).await;
    if result.is_err() {
        let json = fetch();
        return save_session(client, tenant, token, &json).await;
    }
    result
}
