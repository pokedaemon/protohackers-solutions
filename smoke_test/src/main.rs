use tokio::net::TcpSocket;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Task 1 - TCP Echo server(Smoke Test)
#[tokio::main]
async fn main() {
    let address = "127.0.0.1:8080".parse().unwrap();

    let socket = match TcpSocket::new_v4() {
        Ok(v) => v,
        Err(e) => {
            panic!("Socket init error: {e}")
        }
    };

    if let Err(e) = socket.bind(address) {
        panic!("Bind socket error: {e}")
    }

    let listener = match socket.listen(1024) {
            Ok(v) => v,
            Err(e) => panic!("Listener Error: {e}")
    };

    loop {
        let (mut socket, _) = match listener.accept().await {
            Ok(v) => v,
            Err(e) => panic!("Accept Error: {e}")
        };

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];

            
            match socket.read(&mut buffer).await {
                Ok(0) => return,
                Ok(_) => {
                    if let Err(e) = socket.write_all(&buffer).await {
                        eprintln!("Write error: {e}");
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("Read error: {e}");
                    return;
                }
           }
        });
    }
}
