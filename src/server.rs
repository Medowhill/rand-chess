use std::collections::HashMap;

use actix::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Debug)]
pub struct Server {
    sessions: HashMap<usize, Recipient<Message>>,
    id: usize,
}

impl Server {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            id: 0,
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("connected");
        let id = self.id;
        self.id += 1;
        self.sessions.insert(id, msg.addr);
        id
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("disconnected");
        self.sessions.remove(&msg.id);
    }
}
