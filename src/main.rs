use tungstenite::connect;
use tungstenite::Message;
use std::string;
use std::thread;
use std::time::Duration;
use std::vec;

fn main() {
    println!("Hello, world!");

    // Connect to the WebSocket server
    let (mut socket, _) = connect("ws://localhost:5000/").expect("Failed to connect");

    loop {
        socket.write_message(Message::Text("KEYPAIR1024".to_string())).expect("Failed to send message");
        let message = socket.read_message().expect("Failed to receive message");
        match message {
            Message::Text(text) => {
                println!("Received message: {}", text);

                let mut tokens: Vec<String> = vec![];
                for token in text.split('|') {
                    tokens.push(token.to_string());
                }
                let tokensCount = tokens.len();
                if  tokensCount== 3 && tokens[0]=="KEYPAIR1024" {
                    
                    let publicKeyB64 = tokens[1].clone();
                    let privateKeyB64 = tokens[2].clone();

                    println!("Keypair B64: {}", publicKeyB64);
                    println!("Signature B64: {}", privateKeyB64);
                    let publicKeyBytes = base64::decode(publicKeyB64).expect("Failed to decode keypair");
                    let privateKeyBytes = base64::decode(privateKeyB64).expect("Failed to decode signature");
                    
                    let publicKeyUTF8 = String::from_utf8(publicKeyBytes).expect("Failed to convert public key to UTF8");
                    let privateKeyUTF8 = String::from_utf8(privateKeyBytes).expect("Failed to convert private key to UTF8");
                    println!("Public Key:\n {:?}", publicKeyUTF8);
                    println!("Private Key:\n {:?}", privateKeyUTF8);

                }   
            }
            Message::Close(_) => {
                println!("Connection closed");
                break;
            }
             _ => {}
        }
        thread::sleep(Duration::from_secs(3));
    }
    
    // Close the WebSocket connection
    socket.close(None).expect("Failed to close connection");
}
   