use chrono::{DateTime, Utc};
use core::str;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, SystemTime},
};

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
};

use clap::Parser;
use tokio::{net::TcpListener, time::timeout};

enum Messages {
    ClientConnected(Arc<Mutex<TcpStream>>),
    Message {
        author: SocketAddr,
        bytes: Vec<u8>,
        time: SystemTime,
    },
    ClientDisconnected(Arc<Mutex<TcpStream>>),
}
trait Formattable {
    fn format(&self) -> String;
}

#[derive(Clone)]
struct FormatMessage {
    author: SocketAddr,
    bytes: Vec<u8>,
    time: SystemTime,
}
impl FormatMessage {
    fn new(author: SocketAddr, bytes: Vec<u8>, time: SystemTime) -> Self {
        return Self {
            author,
            bytes,
            time,
        };
    }
}
impl Formattable for FormatMessage {
    fn format(&self) -> String {
        let date_string: DateTime<Utc> = DateTime::from(self.time);
        let format_date = date_string.format("%d/%m/%Y %H:%M").to_string();
        let message = str::from_utf8(&self.bytes)
            .map_err(|err| eprint!("Parsing utf-8 error {err}"))
            .unwrap();

        let author: String = self.author.to_string();
        format!("{format_date} [{author}]: {message}")
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// Хост
    #[clap(long, default_value = "127.0.0.1")]
    host: String,

    /// Порт для подключения
    #[clap(short, long, default_value_t = 8080)]
    port: u16,

    /// Timeout соединения для клиента
    #[clap(long, default_value = "10s")]
    timeout: String,
}

struct Client {
    connection: Arc<Mutex<TcpStream>>,
}

async fn handle_tcp_listener() -> Result<TcpListener, Box<dyn std::error::Error>> {
    let args = Args::parse();
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse().unwrap();
    let timeout_seconds = args.timeout.split("s").collect::<Vec<&str>>()[0]
        .parse::<u64>()
        .unwrap_or(10);

    let tcp_connection = TcpListener::bind(addr);
    let timeout_duration = Duration::from_secs(timeout_seconds);

    return match timeout(timeout_duration, tcp_connection).await? {
        Ok(stream) => {
            println!("Successfully connected to {addr}");
            Ok(stream)
        }
        Err(e) => {
            println!("Did not receive answer within {timeout_seconds} seconds");
            return Err(Box::new(e));
        }
    };
}

async fn handle_host_terminal_connection(
    stream: Arc<Mutex<TcpStream>>,
    sender: Arc<Sender<Messages>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut buffer = String::new();

    loop {
        buffer.clear();
        reader.read_line(&mut buffer).await?;

        if buffer.trim().is_empty() {
            continue;
        }

        let client = stream.lock().await;
        let client_address = client.peer_addr()?;

        sender
            .send(Messages::Message {
                author: client_address,
                bytes: buffer.as_bytes().to_vec(),
                time: SystemTime::now(),
            })
            .await?;
    }
}

async fn handle_client_connected(
    client: Arc<Mutex<TcpStream>>,
    clients_map: Arc<Mutex<HashMap<SocketAddr, Client>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client_address = client.lock().await.peer_addr()?;
    let new_client = Client { connection: client };

    clients_map.lock().await.insert(client_address, new_client);

    let message = format!("Client connected: {client_address}");

    let client_map_ref = Arc::clone(&clients_map);
    broadcast_message(&message, Some(client_address), client_map_ref).await?;

    Ok(())
}

async fn handle_client_disconnected(
    client: Arc<Mutex<TcpStream>>,
    clients_map: Arc<Mutex<HashMap<SocketAddr, Client>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    clients_map
        .lock()
        .await
        .remove(&client.lock().await.peer_addr()?);

    let client_address = client.lock().await.peer_addr().unwrap();
    let message_str = format!("Client disconnected: {client_address}");

    let client_map_ref = Arc::clone(&clients_map);
    broadcast_message(&message_str, Some(client_address), client_map_ref).await?;

    Ok(())
}

async fn handle_client_message(
    author: SocketAddr,
    bytes: Vec<u8>,
    message: &str,
    clients_map: Arc<Mutex<HashMap<SocketAddr, Client>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    broadcast_message(message, Some(author), clients_map)
        .await
        .unwrap();

    println!(
        "message recieved from {}: {} ",
        author,
        String::from_utf8(bytes).unwrap()
    );

    Ok(())
}

async fn broadcast_message(
    message: &str,
    exclude: Option<SocketAddr>,
    clients_map: Arc<Mutex<HashMap<SocketAddr, Client>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let message_bytes = message.as_bytes();

    let clients_map_ref = Arc::clone(&clients_map);
    let clients_map = clients_map_ref.lock().await;

    for (addr, client) in clients_map.iter() {
        if exclude.is_some() && *addr == exclude.unwrap() {
            continue;
        }

        let client_connection = Arc::clone(&client.connection);
        let message = message_bytes.to_vec();

        tokio::spawn(async move {
            let mut connection = client_connection.lock().await;
            if let Err(e) = connection.write_all(&message).await {
                eprintln!("Failed to send message to {:?}", e);
            }
        });
    }

    Ok(())
}

async fn handle_server_messages(
    receiver: Arc<Mutex<Receiver<Messages>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let clients_map: HashMap<SocketAddr, Client> = HashMap::new();
    let clients_map_ref = Arc::new(Mutex::new(clients_map));

    while let Some(message) = receiver.lock().await.recv().await {
        match message {
            Messages::ClientConnected(client) => {
                let clients_map_ref: Arc<Mutex<HashMap<SocketAddr, Client>>> =
                    Arc::clone(&clients_map_ref);
                handle_client_connected(client, clients_map_ref)
                    .await
                    .unwrap();
            }
            Messages::Message {
                author,
                bytes,
                time,
            } => {
                let format_message = FormatMessage::new(author, bytes.clone(), time);
                let clients_map_ref = Arc::clone(&clients_map_ref);

                handle_client_message(
                    author,
                    bytes,
                    format_message.format().as_str(),
                    clients_map_ref,
                )
                .await
                .unwrap();
            }
            Messages::ClientDisconnected(client) => {
                let clients_map_ref: Arc<Mutex<HashMap<SocketAddr, Client>>> =
                    Arc::clone(&clients_map_ref);
                handle_client_disconnected(client, clients_map_ref)
                    .await
                    .unwrap();
            }
        }
    }
    return Ok(());
}

async fn handle_client_connection(
    client: Arc<Client>,
    sender: Arc<Sender<Messages>>,
) -> Result<(), Box<dyn std::error::Error>> {
    sender
        .send(Messages::ClientConnected(Arc::clone(&client.connection)))
        .await?;

    let mut message_buffer = [0; 64];
    loop {
        let mut connection = client.connection.lock().await;
        match connection.read(&mut message_buffer).await {
            Ok(0) => {
                sender
                    .send(Messages::ClientDisconnected(Arc::clone(&client.connection)))
                    .await?;
                break;
            }
            Ok(n) => {
                let message = Messages::Message {
                    author: connection.peer_addr()?,
                    bytes: message_buffer[..n].to_vec(),
                    time: SystemTime::now(),
                };

                sender.send(message).await?;
            }
            Err(e) => {
                eprintln!("Failed to read from client: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse().unwrap();

    let tcp_listener = tokio::spawn(async move { handle_tcp_listener() })
        .await
        .unwrap()
        .await
        .unwrap();
    let tcp_listener_ref = Arc::new(tcp_listener);

    let (sender, recieiver) = channel::<Messages>(100);
    let message_sender_ref = Arc::new(sender);
    let recieiver_ref = Arc::new(Mutex::new(recieiver));

    tokio::spawn(async move {
        handle_server_messages(recieiver_ref).await.unwrap();
    });

    // Вызов соединение текущего терминала
    TcpStream::connect(addr).await.unwrap();

    let message_sender_ref = Arc::clone(&message_sender_ref);
    let mut is_first_connection = true;
    loop {
        let tcp_listener_ref: Arc<TcpListener> = Arc::clone(&tcp_listener_ref);
        match tcp_listener_ref.accept().await {
            Ok((socket, _)) => {
                let client = Client {
                    connection: Arc::new(Mutex::new(socket)),
                };
                let client_ref = Arc::new(client);
                let message_sender_ref = Arc::clone(&message_sender_ref);

                if is_first_connection {
                    is_first_connection = false;

                    let client_ref = Arc::clone(&client_ref);
                    let message_sender_ref = Arc::clone(&message_sender_ref);

                    tokio::spawn(async move {
                        handle_host_terminal_connection(
                            Arc::clone(&client_ref.connection),
                            message_sender_ref,
                        )
                        .await
                        .unwrap();
                    });
                }

                tokio::spawn(async move {
                    handle_client_connection(client_ref, message_sender_ref)
                        .await
                        .unwrap();
                });
            }
            Err(err) => {
                eprintln!("ERROR: could not init connection {err}");
                return Ok(());
            }
        }
    }
}
