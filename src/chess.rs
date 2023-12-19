use rand::{seq::SliceRandom, *};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Color {
    White,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.is_white() { 'w' } else { 'b' };
        write!(f, "{}", c)
    }
}

impl Color {
    #[inline]
    fn other(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub fn is_white(self) -> bool {
        self == Color::White
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Piece {
    typ: PieceType,
    color: Color,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.typ {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        let c = if self.color.is_white() {
            c.to_ascii_uppercase()
        } else {
            c
        };
        write!(f, "{}", c)
    }
}

impl Piece {
    #[inline]
    const fn new(typ: PieceType, color: Color) -> Self {
        Self { typ, color }
    }

    #[inline]
    fn is_pawn(self) -> bool {
        self.typ == PieceType::Pawn
    }

    #[inline]
    fn is_knight(self) -> bool {
        self.typ == PieceType::Knight
    }

    #[inline]
    fn is_bishop(self) -> bool {
        self.typ == PieceType::Bishop
    }

    #[inline]
    fn is_rook(self) -> bool {
        self.typ == PieceType::Rook
    }

    #[inline]
    fn is_queen(self) -> bool {
        self.typ == PieceType::Queen
    }

    #[inline]
    fn is_king(self) -> bool {
        self.typ == PieceType::King
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Location {
    file: i8,
    rank: i8,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = self.rank + 1;
        let file = (self.file + 'a' as i8) as u8 as char;
        write!(f, "{}{}", file, rank)
    }
}

impl std::ops::Add<(i8, i8)> for Location {
    type Output = Self;

    fn add(self, x: (i8, i8)) -> Self::Output {
        Self::new(self.file + x.0, self.rank + x.1)
    }
}

impl std::ops::Add<&(i8, i8)> for Location {
    type Output = Self;

    fn add(self, x: &(i8, i8)) -> Self::Output {
        Self::new(self.file + x.0, self.rank + x.1)
    }
}

impl Location {
    #[inline]
    const fn new(file: i8, rank: i8) -> Self {
        Self { file, rank }
    }

    #[inline]
    fn is_valid(self) -> bool {
        let range = 0..8;
        range.contains(&self.file) && range.contains(&self.rank)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LocationState {
    Invalid,
    Empty,
    Opponent(Piece),
    Friendly,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Move {
    piece: Piece,
    from: Location,
    to: Location,
    attack: Option<(Location, Piece)>,
    castle: Option<(Location, Location)>,
    is_promotion: bool,
    promote_to: Option<PieceType>,
}

impl Move {
    fn new(piece: Piece, from: Location, to: Location) -> Self {
        Self {
            piece,
            from,
            to,
            attack: None,
            castle: None,
            is_promotion: false,
            promote_to: None,
        }
    }

    fn with_attack(mut self, loc: Location, piece: Piece) -> Self {
        self.attack = Some((loc, piece));
        self
    }

    fn with_castle(mut self, rook_from: Location, rook_to: Location) -> Self {
        self.castle = Some((rook_from, rook_to));
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum GameState {
    Normal,
    Checkmate(Color),
    Stalemate,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Event {
    Swap(Location, Location),
    KnightToBishop(Location),
    BishopToKnight(Location),
    RooksToQueen(Location, Location),
    QueenToRooks(Location, Location),
    PawnRun(Location, Location),
    PawnsToQueen([Location; 8]),
    QueenToPawns(Location, i8),
    Rotate(Location, Location),
    KingMove(Location, Location),
}

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
    pub active: Color,
    wk_castle: bool,
    wq_castle: bool,
    bk_castle: bool,
    bq_castle: bool,
    en_passant: Option<Location>,
    pub half_moves: usize,
    pub last_move: Option<Move>,
    pub last_event: Option<Event>,
    pub last_card: Option<usize>,
    pub white_cards: Vec<usize>,
    pub black_cards: Vec<usize>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, rank) in self.pieces.iter().enumerate().rev() {
            let mut empty = 0;
            for piece in rank.iter() {
                match piece {
                    Some(piece) => {
                        if empty > 0 {
                            write!(f, "{}", empty)?;
                            empty = 0;
                        }
                        write!(f, "{}", piece)?;
                    }
                    None => empty += 1,
                }
            }
            if empty > 0 {
                write!(f, "{}", empty)?;
            }
            if i > 0 {
                write!(f, "/")?;
            }
        }
        write!(f, " {}", self.active)?;
        let wk_castle = if self.wk_castle { 'K' } else { '-' };
        let wq_castle = if self.wq_castle { 'Q' } else { '-' };
        let bk_castle = if self.bk_castle { 'k' } else { '-' };
        let bq_castle = if self.bq_castle { 'q' } else { '-' };
        write!(f, " {}{}{}{}", wk_castle, wq_castle, bk_castle, bq_castle)?;
        let en_passant = match self.en_passant {
            Some(loc) => loc.to_string(),
            None => "-".to_string(),
        };
        write!(f, " {} 0 1", en_passant)
    }
}

impl Default for Board {
    #[inline]
    fn default() -> Self {
        Self {
            pieces: INIT_PIECES,
            active: Color::White,
            wk_castle: true,
            wq_castle: true,
            bk_castle: true,
            bq_castle: true,
            en_passant: None,
            half_moves: 0,
            last_move: None,
            last_event: None,
            last_card: None,
            white_cards: vec![0, 0],
            black_cards: vec![0, 0],
        }
    }
}

impl Board {
    fn iter_locations(&self) -> impl Iterator<Item = (Location, Option<&Piece>)> {
        self.pieces.iter().enumerate().flat_map(|(rank, pieces)| {
            pieces.iter().enumerate().map(move |(file, piece)| {
                let loc = Location::new(file as i8, rank as i8);
                (loc, piece.as_ref())
            })
        })
    }

    fn iter_empty_locations(&self) -> impl Iterator<Item = (Location, Option<&Piece>)> {
        self.iter_locations().filter(|(_, piece)| piece.is_none())
    }

    fn iter_pieces(&self) -> impl Iterator<Item = (Location, &Piece)> {
        self.iter_locations().filter_map(|(loc, piece)| {
            let piece = piece?;
            Some((loc, piece))
        })
    }

    fn iter_pieces_of(&self, color: Color) -> impl Iterator<Item = (Location, &Piece)> {
        self.iter_pieces()
            .filter(move |(_, piece)| piece.color == color)
    }

    fn iter_active_pieces(&self) -> impl Iterator<Item = (Location, &Piece)> {
        self.iter_pieces_of(self.active)
    }

    fn piece(&self, loc: Location) -> Option<Piece> {
        self.pieces[loc.rank as usize][loc.file as usize]
    }

    fn is_empty(&self, loc: Location) -> bool {
        self.piece(loc).is_none()
    }

    fn set_piece(&mut self, loc: Location, piece: Option<Piece>) {
        self.pieces[loc.rank as usize][loc.file as usize] = piece;
    }

    fn get_state(&self, loc: Location, color: Color) -> LocationState {
        if !loc.is_valid() {
            return LocationState::Invalid;
        }
        match self.piece(loc) {
            Some(piece) => {
                if piece.color == color {
                    LocationState::Friendly
                } else {
                    LocationState::Opponent(piece)
                }
            }
            None => LocationState::Empty,
        }
    }

    pub fn get_check(&self) -> Option<Location> {
        for (loc, piece) in self.iter_pieces_of(self.active.other()) {
            let moves = self.possible_moves(loc, *piece, false);
            for mv in moves {
                if let Some((loc, piece)) = mv.attack {
                    if piece.is_king() {
                        return Some(loc);
                    }
                }
            }
        }
        None
    }

    fn can_attack_king(&self, color: Color) -> bool {
        for (loc, piece) in self.iter_pieces_of(color) {
            let moves = self.possible_moves(loc, *piece, false);
            for mv in moves {
                if let Some((_, piece)) = mv.attack {
                    if piece.is_king() {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn all_possible_moves(&self) -> Vec<(Location, Vec<Move>)> {
        self.iter_pieces_of(self.active)
            .map(|(loc, piece)| (loc, self.possible_moves(loc, *piece, true)))
            .filter(|(_, moves)| !moves.is_empty())
            .collect()
    }

    pub fn game_state(&self, possible_moves: &[(Location, Vec<Move>)]) -> GameState {
        if possible_moves.is_empty() {
            let other = self.active.other();
            if self.can_attack_king(other) {
                GameState::Checkmate(other)
            } else {
                GameState::Stalemate
            }
        } else {
            GameState::Normal
        }
    }

    pub fn is_game_over(&self) -> bool {
        let possible_moves = self.all_possible_moves();
        let st = self.game_state(&possible_moves);
        st != GameState::Normal
    }

    fn possible_moves(&self, loc: Location, piece: Piece, safe: bool) -> Vec<Move> {
        let Piece { typ, color } = piece;
        let mut moves = match typ {
            PieceType::Pawn => {
                let mut moves = vec![];
                let dir = if color.is_white() { 1 } else { -1 };
                let nloc = loc + (0, dir);
                if self.get_state(nloc, color) == LocationState::Empty {
                    moves.push(Move::new(piece, loc, nloc));

                    let second_rank = if color.is_white() { 1 } else { 6 };
                    if loc.rank == second_rank {
                        let nloc = nloc + (0, dir);
                        if self.get_state(nloc, color) == LocationState::Empty {
                            moves.push(Move::new(piece, loc, nloc));
                        }
                    }
                }
                for dx in [1, -1] {
                    let nloc = loc + (dx, dir);
                    match self.get_state(nloc, color) {
                        LocationState::Empty => {
                            if let Some(en_passant) = self.en_passant {
                                if en_passant == nloc {
                                    let aloc = nloc + (0, -dir);
                                    let opponent = Piece::new(PieceType::Pawn, color.other());
                                    moves.push(
                                        Move::new(piece, loc, nloc).with_attack(aloc, opponent),
                                    );
                                }
                            }
                        }
                        LocationState::Opponent(opponent) => {
                            moves.push(Move::new(piece, loc, nloc).with_attack(nloc, opponent));
                        }
                        _ => {}
                    }
                }
                for mv in &mut moves {
                    let eighth_rank = if color.is_white() { 7 } else { 0 };
                    if mv.to.rank == eighth_rank {
                        mv.is_promotion = true;
                    }
                }
                moves
            }
            PieceType::Knight => {
                let dirs = all_directions(1, 2);
                self.moves_to_directions(loc, piece, &dirs, false)
            }
            PieceType::Bishop => {
                let dirs = all_directions(1, 1);
                self.moves_to_directions(loc, piece, &dirs, true)
            }
            PieceType::Rook => {
                let dirs = all_directions(1, 0);
                self.moves_to_directions(loc, piece, &dirs, true)
            }
            PieceType::Queen => {
                let mut dirs = all_directions(1, 1);
                dirs.extend(all_directions(1, 0));
                self.moves_to_directions(loc, piece, &dirs, true)
            }
            PieceType::King => {
                let mut dirs = all_directions(1, 1);
                dirs.extend(all_directions(1, 0));
                let mut moves = self.moves_to_directions(loc, piece, &dirs, false);
                let kingside = if color.is_white() {
                    self.wk_castle
                } else {
                    self.bk_castle
                };
                if kingside
                    && self.get_state(loc + (1, 0), color) == LocationState::Empty
                    && self.get_state(loc + (2, 0), color) == LocationState::Empty
                {
                    moves.push(
                        Move::new(piece, loc, loc + (2, 0)).with_castle(loc + (3, 0), loc + (1, 0)),
                    );
                }
                let queenside = if color.is_white() {
                    self.wq_castle
                } else {
                    self.bq_castle
                };
                if queenside
                    && self.get_state(loc + (-1, 0), color) == LocationState::Empty
                    && self.get_state(loc + (-2, 0), color) == LocationState::Empty
                {
                    moves.push(
                        Move::new(piece, loc, loc + (-2, 0))
                            .with_castle(loc + (-4, 0), loc + (-1, 0)),
                    );
                }
                moves
            }
        };
        if safe {
            moves.retain(|mv| {
                let new_board = self.piece_moved(mv);
                let opponent = color.other();
                if new_board.can_attack_king(opponent) {
                    return false;
                }
                if mv.castle.is_some() {
                    if self.can_attack_king(opponent) {
                        return false;
                    }
                    let dx = mv.to.file - mv.from.file;
                    let middle_mv = Move::new(mv.piece, mv.from, mv.from + (dx / 2, 0));
                    let new_board = self.piece_moved(&middle_mv);
                    if new_board.can_attack_king(opponent) {
                        return false;
                    }
                }
                true
            });
        }
        moves
    }

    fn moves_to_directions(
        &self,
        loc: Location,
        piece: Piece,
        directions: &[(i8, i8)],
        repeat: bool,
    ) -> Vec<Move> {
        let mut moves = vec![];
        for dir in directions {
            let mut nloc = loc;
            loop {
                nloc = nloc + dir;
                match self.get_state(nloc, piece.color) {
                    LocationState::Empty => {
                        moves.push(Move::new(piece, loc, nloc));
                    }
                    LocationState::Opponent(opponent) => {
                        moves.push(Move::new(piece, loc, nloc).with_attack(nloc, opponent));
                        break;
                    }
                    _ => break,
                }
                if !repeat {
                    break;
                }
            }
        }
        moves
    }

    fn update_castle(&mut self, loc: Location, color: Color) {
        if color.is_white() {
            if loc.rank == 0 {
                if loc.file == 0 {
                    self.wq_castle = false;
                } else if loc.file == 7 {
                    self.wk_castle = false;
                }
            }
        } else if loc.rank == 7 {
            if loc.file == 0 {
                self.bq_castle = false;
            } else if loc.file == 7 {
                self.bk_castle = false;
            }
        }
    }

    fn piece_moved(&self, mv: &Move) -> Self {
        let mut new_board = self.clone();
        new_board.en_passant = None;
        if let Some((loc, _)) = mv.attack {
            new_board.set_piece(loc, None);
        }
        new_board.set_piece(mv.from, None);
        if let Some((from, to)) = mv.castle {
            new_board.set_piece(from, None);
            new_board.set_piece(to, Some(Piece::new(PieceType::Rook, mv.piece.color)));
        }
        let mut piece = mv.piece;
        match piece.typ {
            PieceType::Pawn => {
                let dy = mv.to.rank - mv.from.rank;
                if dy == 2 || dy == -2 {
                    new_board.en_passant = Some(mv.from + (0, dy / 2));
                }
            }
            PieceType::Rook => {
                new_board.update_castle(mv.from, piece.color);
            }
            PieceType::King => {
                if piece.color.is_white() {
                    new_board.wk_castle = false;
                    new_board.wq_castle = false;
                } else {
                    new_board.bk_castle = false;
                    new_board.bq_castle = false;
                }
            }
            _ => {}
        }
        if let Some((loc, attacked)) = mv.attack {
            new_board.update_castle(loc, attacked.color);
        }
        if let Some(typ) = mv.promote_to {
            piece.typ = typ;
        }
        new_board.set_piece(mv.to, Some(piece));
        new_board.last_move = Some(mv.clone());
        new_board.last_event = None;
        new_board.active = new_board.active.other();
        new_board.half_moves += 1;
        new_board
    }

    pub fn move_piece(&mut self, mv: &Move) {
        *self = self.piece_moved(mv);
    }

    fn final_rank(&self) -> i8 {
        if self.active.is_white() {
            7
        } else {
            0
        }
    }

    pub fn draw_card(&mut self) {
        let cards = if self.active.is_white() {
            &mut self.white_cards
        } else {
            &mut self.black_cards
        };
        let card = cards.pop().unwrap();
        self.last_card = Some(card);
        if cards.is_empty() {
            *cards = generate_cards(5);
        }
        let ev = match card {
            0 => None,
            1 => self.make_b2n(),
            2 => self.make_n2b(),
            3 => self.make_r2q(),
            4 => self.make_q2r(),
            5 => self.make_p2q(),
            6 => self.make_q2p(),
            7 => self.make_swap(),
            8 => self.make_rotate(),
            9 => self.make_pawn_run(),
            _ => self.make_king_move(),
        };
        if let Some(ev) = ev {
            self.apply_event(ev);
        }
    }

    fn gen_events<F: FnOnce(&[(Location, &Piece)], &mut Vec<Event>)>(&self, f: F) -> Option<Event> {
        let pieces: Vec<_> = self.iter_active_pieces().collect();
        let mut cands = vec![];
        f(&pieces, &mut cands);
        cands.retain(|e| self.is_valid_event(*e));
        cands.shuffle(&mut thread_rng());
        cands.into_iter().next()
    }

    fn make_swap(&self) -> Option<Event> {
        self.gen_events(|pieces, cands| {
            for (i, (l1, p1)) in pieces.iter().enumerate() {
                for (l2, p2) in pieces.iter().take(i) {
                    if p1.typ == p2.typ || p1.is_king() || p2.is_king() {
                        continue;
                    }
                    if p1.is_pawn() && l2.rank == self.final_rank() {
                        continue;
                    }
                    if p2.is_pawn() && l1.rank == self.final_rank() {
                        continue;
                    }
                    cands.push(Event::Swap(*l1, *l2));
                }
            }
        })
    }

    fn make_n2b(&self) -> Option<Event> {
        self.gen_events(|pieces, cands| {
            for (l, _) in pieces.iter().filter(|(_, p)| p.is_knight()) {
                cands.push(Event::KnightToBishop(*l));
            }
        })
    }

    fn make_b2n(&self) -> Option<Event> {
        self.gen_events(|pieces, cands| {
            for (l, _) in pieces.iter().filter(|(_, p)| p.is_bishop()) {
                cands.push(Event::BishopToKnight(*l));
            }
        })
    }

    fn make_r2q(&self) -> Option<Event> {
        self.gen_events(|pieces, cands| {
            let rooks: Vec<_> = pieces.iter().filter(|(_, p)| p.is_rook()).collect();
            for (l1, _) in rooks.iter() {
                for (l2, _) in rooks.iter() {
                    if l1 != l2 {
                        cands.push(Event::RooksToQueen(*l1, *l2));
                    }
                }
            }
        })
    }

    fn make_q2r(&self) -> Option<Event> {
        self.gen_events(|pieces, cands| {
            let emptys: Vec<_> = self.iter_empty_locations().collect();
            for (l1, _) in pieces.iter().filter(|(_, p)| p.is_queen()) {
                for (l2, _) in emptys.iter() {
                    cands.push(Event::QueenToRooks(*l1, *l2));
                }
            }
        })
    }

    fn make_pawn_run(&self) -> Option<Event> {
        self.gen_events(|pieces, cands| {
            let dy = if self.active.is_white() { 1 } else { -1 };
            for (l, _) in pieces.iter().filter(|(_, p)| p.is_pawn()) {
                let mut nloc = *l;
                let nloc = loop {
                    let nnloc = nloc + (0, dy);
                    if !self.is_empty(nnloc) || nnloc.rank == self.final_rank() {
                        break nloc;
                    }
                    nloc = nnloc;
                };
                if *l != nloc {
                    cands.push(Event::PawnRun(*l, nloc));
                }
            }
        })
    }

    fn make_p2q(&self) -> Option<Event> {
        self.gen_events(|pieces, cands| {
            let pawns: Vec<_> = pieces
                .iter()
                .filter_map(|(l, p)| if p.is_pawn() { Some(*l) } else { None })
                .take(8)
                .collect();
            if pawns.len() < 8 {
                return;
            }
            for i in 0..8 {
                let mut v = pawns.clone();
                v.swap(0, i);
                cands.push(Event::PawnsToQueen(v.try_into().unwrap()));
            }
        })
    }

    fn make_q2p(&self) -> Option<Event> {
        self.gen_events(|pieces, cands| {
            for (l, _) in pieces.iter().filter(|(_, p)| p.is_queen()) {
                for rank in 0..8 {
                    if rank == self.final_rank() {
                        continue;
                    }
                    let mut clean = true;
                    for file in 0..8 {
                        let location = Location::new(file, rank);
                        if location != *l && !self.is_empty(location) {
                            clean = false;
                            break;
                        }
                    }
                    if clean {
                        cands.push(Event::QueenToPawns(*l, rank));
                    }
                }
            }
        })
    }

    fn make_rotate(&self) -> Option<Event> {
        self.gen_events(|pieces, cands| {
            for (l, _) in pieces.iter() {
                match l.file {
                    0 => {
                        let loc = Location::new(7, l.rank);
                        if self.is_empty(loc) {
                            cands.push(Event::Rotate(*l, loc));
                        }
                    }
                    7 => {
                        let loc = Location::new(0, l.rank);
                        if self.is_empty(loc) {
                            cands.push(Event::Rotate(*l, loc));
                        }
                    }
                    _ => {}
                }
            }
        })
    }

    fn make_king_move(&self) -> Option<Event> {
        self.gen_events(|pieces, cands| {
            let (l, _) = pieces.iter().find(|(_, p)| p.is_king()).unwrap();
            let dx = if l.file < 4 { 1 } else { -1 };
            let mut nloc = *l;
            let nloc = loop {
                let nnloc = nloc + (dx, 0);
                if nnloc.file < 0 || nnloc.file > 7 || !self.is_empty(nnloc) {
                    break nloc;
                }
                nloc = nnloc;
            };
            if *l != nloc {
                cands.push(Event::KingMove(*l, nloc));
            }
        })
    }

    fn is_valid_event(&self, event: Event) -> bool {
        let board = self.event_applied(event);
        !board.can_attack_king(board.active) && !board.all_possible_moves().is_empty()
    }

    fn event_applied(&self, event: Event) -> Self {
        let mut new_board = self.clone();
        match event {
            Event::Swap(l1, l2) => {
                let p1 = self.piece(l1);
                new_board.set_piece(l1, self.piece(l2));
                new_board.set_piece(l2, p1);
                new_board.update_castle(l1, new_board.active);
                new_board.update_castle(l2, new_board.active);
            }
            Event::KnightToBishop(l) => {
                let mut piece = self.piece(l).unwrap();
                piece.typ = PieceType::Bishop;
                new_board.set_piece(l, Some(piece));
            }
            Event::BishopToKnight(l) => {
                let mut piece = self.piece(l).unwrap();
                piece.typ = PieceType::Knight;
                new_board.set_piece(l, Some(piece));
            }
            Event::RooksToQueen(l1, l2) => {
                let mut piece = self.piece(l1).unwrap();
                piece.typ = PieceType::Queen;
                new_board.set_piece(l1, Some(piece));
                new_board.set_piece(l2, None);
                if new_board.active.is_white() {
                    new_board.wq_castle = false;
                    new_board.wk_castle = false;
                } else {
                    new_board.bq_castle = false;
                    new_board.bk_castle = false;
                }
            }
            Event::QueenToRooks(l1, l2) => {
                let mut piece = self.piece(l1).unwrap();
                piece.typ = PieceType::Rook;
                new_board.set_piece(l1, Some(piece));
                new_board.set_piece(l2, Some(piece));
            }
            Event::PawnRun(l1, l2) => {
                let piece = self.piece(l1).unwrap();
                new_board.set_piece(l1, None);
                new_board.set_piece(l2, Some(piece));
            }
            Event::PawnsToQueen(ls) => {
                let mut piece = self.piece(ls[0]).unwrap();
                piece.typ = PieceType::Queen;
                new_board.set_piece(ls[0], Some(piece));
                for l in ls.iter().skip(1) {
                    new_board.set_piece(*l, None);
                }
            }
            Event::QueenToPawns(l, rank) => {
                let mut piece = self.piece(l).unwrap();
                piece.typ = PieceType::Pawn;
                new_board.set_piece(l, None);
                for i in 0..8 {
                    let loc = Location::new(i as _, rank);
                    new_board.set_piece(loc, Some(piece));
                }
            }
            Event::Rotate(l1, l2) => {
                let piece = self.piece(l1).unwrap();
                new_board.set_piece(l1, None);
                new_board.set_piece(l2, Some(piece));
                new_board.update_castle(l1, new_board.active);
            }
            Event::KingMove(l1, l2) => {
                let piece = self.piece(l1).unwrap();
                new_board.set_piece(l1, None);
                new_board.set_piece(l2, Some(piece));
                if new_board.active.is_white() {
                    new_board.wk_castle = false;
                    new_board.wq_castle = false;
                } else {
                    new_board.bk_castle = false;
                    new_board.bq_castle = false;
                }
            }
        }
        new_board.last_event = Some(event);
        new_board
    }

    fn apply_event(&mut self, event: Event) {
        *self = self.event_applied(event);
    }
}

fn all_directions(dfile: i8, drank: i8) -> Vec<(i8, i8)> {
    let mut dirs = vec![];
    let swap = dfile != drank;
    dirs.push((dfile, drank));
    if swap {
        dirs.push((drank, dfile));
    }
    if dfile != 0 {
        dirs.push((-dfile, drank));
        if swap {
            dirs.push((drank, -dfile));
        }
    }
    if drank != 0 {
        dirs.push((dfile, -drank));
        if swap {
            dirs.push((-drank, dfile));
        }
    }
    if dfile != 0 && drank != 0 {
        dirs.push((-dfile, -drank));
        if swap {
            dirs.push((-drank, -dfile));
        }
    }
    dirs
}

fn generate_cards(len: usize) -> Vec<usize> {
    (0..len)
        .map(|_| {
            if thread_rng().gen_bool(0.5) {
                0
            } else {
                thread_rng().gen_range(1..=10)
            }
        })
        .collect()
}

const INIT_PIECES: [[Option<Piece>; 8]; 8] = [
    [
        Some(Piece::new(PieceType::Rook, Color::White)),
        Some(Piece::new(PieceType::Knight, Color::White)),
        Some(Piece::new(PieceType::Bishop, Color::White)),
        Some(Piece::new(PieceType::Queen, Color::White)),
        Some(Piece::new(PieceType::King, Color::White)),
        Some(Piece::new(PieceType::Bishop, Color::White)),
        Some(Piece::new(PieceType::Knight, Color::White)),
        Some(Piece::new(PieceType::Rook, Color::White)),
    ],
    [Some(Piece::new(PieceType::Pawn, Color::White)); 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [Some(Piece::new(PieceType::Pawn, Color::Black)); 8],
    [
        Some(Piece::new(PieceType::Rook, Color::Black)),
        Some(Piece::new(PieceType::Knight, Color::Black)),
        Some(Piece::new(PieceType::Bishop, Color::Black)),
        Some(Piece::new(PieceType::Queen, Color::Black)),
        Some(Piece::new(PieceType::King, Color::Black)),
        Some(Piece::new(PieceType::Bishop, Color::Black)),
        Some(Piece::new(PieceType::Knight, Color::Black)),
        Some(Piece::new(PieceType::Rook, Color::Black)),
    ],
];
