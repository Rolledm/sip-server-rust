
pub enum MessageType {
    Request(RequestMethod),
    Response(String),
}

pub enum RequestMethod {
    Register,
    Invite,
    ACK,
    Cancel,
    Bye,
    Options,
}

pub struct Message {
    mtype: MessageType,

    // Mandatory for request headers:
    to: String,
    from: String,
    cceq: String,
    call_id: String,
    max_forwards: String,
    via: String,
}

impl Message {
    pub fn new(mtype: MessageType) -> Message {
        Message {
            mtype: mtype,
            to: String::new(),
            from: String::new(),
            cceq: String::new(),
            call_id: String::new(),
            max_forwards: String::new(),
            via: String::new(),
        }
    }

    pub fn to(&mut self, to: String) -> &mut Message {
        self.to = to;
        self
    }

    pub fn toString(&mut self) -> String {
        let start_line = match &self.mtype {
            MessageType::Request(method) => {
                format!("BYE sip:{}@atlanta.example.com SIP/2.0\r\n", self.to)
            },
            MessageType::Response(response) => {
                format!("SIP/2.0 {}", response)
            }
        };
        format!("{}Via: SIP/2.0/TCP client.chicago.example.com:5060;branch=z9hG4bKfgaw2\r
Max-Forwards: 70\r
Route: <sip:ss3.chicago.example.com;lr>\r
From: Bob <sip:bob@biloxi.example.com>;tag=314159\r
To: Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl\r
Call-ID: 2xTb9vxSit55XU7p8@atlanta.example.com\r
CSeq: 1 BYE\r
Content-Length: 0\r\n\r\n", start_line)
    }
}


/*
"BYE sip:alice@client.atlanta.example.com SIP/2.0\r
Via: SIP/2.0/TCP client.chicago.example.com:5060;branch=z9hG4bKfgaw2\r
Max-Forwards: 70\r
Route: <sip:ss3.chicago.example.com;lr>\r
From: Bob <sip:bob@biloxi.example.com>;tag=314159\r
To: Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl\r
Call-ID: 2xTb9vxSit55XU7p8@atlanta.example.com\r
CSeq: 1 BYE\r
Content-Length: 0\r\n\r\n";
*/