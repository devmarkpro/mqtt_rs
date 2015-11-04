use message;


pub struct Server {
    id: i32,
}


static CONNACK_OK : [u8; 4] = [32, 2, 0, 0];
static PING_RESP : [u8; 2] = [0xd0, 0];


impl Server {
    pub fn new() -> Self {
        Server { id: 4 }
    }

    pub fn new_message<C: Client>(&mut self, client: &mut C, bytes: &[u8]) {
        let message_type = message::message_type(bytes);
        match message_type {
            message::MqttType::Connect => {
                client.send(&CONNACK_OK);
            },
            message::MqttType::PingReq => {
                client.send(&PING_RESP);
            },
            _ => panic!("Unknown message type")
        }
    }
}

pub trait Client {
    fn send(&mut self, bytes: &[u8]);
}

struct TestClient {
    msgs: Vec<Vec<u8>>,
}

impl TestClient {
    fn new() -> Self {
        TestClient { msgs: vec!() }
    }

    fn last_msg(&self) -> &[u8] {
        self.msgs.last().unwrap()
    }
}

impl Client for TestClient {
    fn send(&mut self, bytes: &[u8]) {
        self.msgs.push(bytes.to_vec());
    }
}


#[test]
fn test_connect() {
    let connect_bytes = &[
        0x10u8, 0x2a, // fixed header
        0x00, 0x06, 'M' as u8, 'Q' as u8, 'I' as u8, 's' as u8, 'd' as u8, 'p' as u8,
        0x03, // protocol version
        0xcc, // connection flags 1100111x user, pw, !wr, w(01), w, !c, x
        0x00, 0x0a, // keepalive of 100
        0x00, 0x03, 'c' as u8, 'i' as u8, 'd' as u8, // client ID
        0x00, 0x04, 'w' as u8, 'i' as u8, 'l' as u8, 'l' as u8, // will topic
        0x00, 0x04, 'w' as u8, 'm' as u8, 's' as u8, 'g' as u8, // will msg
        0x00, 0x07, 'g' as u8, 'l' as u8, 'i' as u8, 'f' as u8, 't' as u8, 'e' as u8, 'l' as u8, // username
        0x00, 0x02, 'p' as u8, 'w' as u8, // password
        ][0..];

    let mut server = Server::new();
    let mut client = TestClient::new();

    server.new_message(&mut client, connect_bytes);
    assert_eq!(client.last_msg(), &CONNACK_OK);
}


#[test]
fn test_ping() {
    let ping_bytes =  &[0xc0u8, 0][0..];

    let mut server = Server::new();
    let mut client = TestClient::new();

    server.new_message(&mut client, ping_bytes);

    assert_eq!(client.last_msg(), &PING_RESP);
}


// #[test]
// fn test_pings() {
//     let ping_bytes =  &[0xc0u8, 0, 0xc0, 0, 0xc0, 0, 0xc0, 0][0..];

//     let mut server = Server::new();
//     let mut client = TestClient::new();

//     server.new_bytes(&mut client, ping_bytes);

//     assert_eq!(client.last_msg(), &PING_RESP);
// }
