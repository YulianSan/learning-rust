use mini_redis::{client, Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

pub async fn using_lib() -> Result<()> {
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

pub async fn cli() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        println!("accepted connection");
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

pub async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
