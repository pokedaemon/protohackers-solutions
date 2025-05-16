use tokio::{io::AsyncReadExt, net::TcpSocket};

/*  
 *  Task 2 - TCP Server with
 *  request  = { "method":"isPrime", "number":123 }
 *  responce = { "method":"isPrime", "prime": false }
*/

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:8080".parse().unwrap();

    let socket = match TcpSocket::new_v4() {
        Ok(v) => v,
        Err(e) => panic!("Socket spawn error: {e}\n")
    };

    if let Err(e) = socket.bind(address) {
        panic!("Socket bind error: {e}\n")
    };

    let listener = match socket.listen(1024) {
        Ok(v) => v,
        Err(e) => panic!("TcpListener create error: {e}")
    };

    loop {
        let mut stream = match listener.accept().await {
            Ok((v, _)) => v,
            Err(e) => panic!("Listener accept error: {e}")
        };

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];

            // TODO: bufread, bufwrite, json
        });
    }
}
