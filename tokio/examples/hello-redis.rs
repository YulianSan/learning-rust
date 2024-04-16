use mini_redis::{client, Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;
    let value = client.get("hello").await?;

    assert_eq!(value, Some(b"world".to_vec().into()));
    println!(
        "got value from the server: {}",
        value
            .unwrap()
            .iter()
            .map(|b| *b as char)
            .collect::<String>()
    );

    Ok(())
}
