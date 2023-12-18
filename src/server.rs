use crate::*;
use actix::*;
use chess::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
enum Role {
    Player(Color),
    Spectator,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct Message {
    pieces: [[Option<Piece>; 8]; 8],
    moves: Vec<(Location, Vec<Move>)>,
    state: GameState,
    last: Option<Move>,
    last_event: Option<Event>,
    role: Role,
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
    white: Option<usize>,
    black: Option<usize>,
    board: Board,
    id: usize,
}

impl Server {
    fn send_state(&self) {
        let pieces = self.board.pieces;
        let moves = self.board.all_possible_moves();
        let state = self.board.game_state(&moves);
        let last = self.board.last_move.clone();
        let last_event = self.board.last_event;
        for (id, addr) in &self.sessions {
            let role = if Some(*id) == self.white {
                Role::Player(Color::White)
            } else if Some(*id) == self.black {
                Role::Player(Color::Black)
            } else {
                Role::Spectator
            };
            let moves = match role {
                Role::Player(color) if color == self.board.active => moves.clone(),
                _ => vec![],
            };
            addr.do_send(Message {
                pieces,
                moves,
                state,
                last: last.clone(),
                last_event,
                role,
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
        let id = self.id;
        println!("connected {}", id);
        self.id += 1;
        self.sessions.insert(id, msg.addr);
        if self.white.is_none() {
            self.white = Some(id);
        } else if self.black.is_none() {
            self.black = Some(id);
        }
        self.send_state();
        id
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("disconnected {}", msg.id);
        self.sessions.remove(&msg.id);
        if self.white == Some(msg.id) {
            self.white = None;
        } else if self.black == Some(msg.id) {
            self.black = None;
        }
    }
}

impl Handler<Request> for Server {
    type Result = ();

    fn handle(&mut self, msg: Request, _: &mut Context<Self>) {
        match msg {
            Request::Move(mv) => {
                self.board.move_piece(&mv);
                let ev = self.board.make_random_event();
                if let Some(ev) = ev {
                    self.board.apply_event(ev);
                }
                self.send_state();
            }
            Request::Restart => {
                self.board = Board::default();
                self.send_state();
            }
        }
    }
}
