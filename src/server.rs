use crate::*;
use actix::*;
use chess::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct Message {
    pieces: [[Option<Piece>; 8]; 8],
    moves: Vec<(Location, Vec<Move>)>,
    state: GameState,
}

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

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub enum Request {
    Move(Move),
    Restart,
}

#[derive(Debug, Default)]
pub struct Server {
    sessions: HashMap<usize, Recipient<Message>>,
    board: Board,
    id: usize,
}

impl Server {
    fn send_state(&self) {
        let pieces = self.board.pieces;
        let moves = self.board.all_possible_moves();
        let state = self.board.game_state(&moves);
        for addr in self.sessions.values() {
            addr.do_send(Message {
                pieces,
                moves: moves.clone(),
                state,
            });
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
        self.send_state();
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

impl Handler<Request> for Server {
    type Result = ();

    fn handle(&mut self, msg: Request, _: &mut Context<Self>) {
        match msg {
            Request::Move(mv) => {
                self.board.move_piece(&mv);
                self.send_state();
            }
            Request::Restart => {
                self.board = Board::default();
                self.send_state();
            }
        }
    }
}
