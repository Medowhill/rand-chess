use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
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
    fn is_white(self) -> bool {
        self == Color::White
    }

    #[inline]
    fn is_black(self) -> bool {
        self == Color::Black
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Piece {
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Location {
    rank: i8,
    file: i8,
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
        Self::new(self.rank + x.0, self.file + x.1)
    }
}

impl std::ops::Add<&(i8, i8)> for Location {
    type Output = Self;

    fn add(self, x: &(i8, i8)) -> Self::Output {
        Self::new(self.rank + x.0, self.file + x.1)
    }
}

impl Location {
    #[inline]
    const fn new(rank: i8, file: i8) -> Self {
        Self { rank, file }
    }

    #[inline]
    fn is_valid(self) -> bool {
        let range = 0..8;
        range.contains(&self.rank) && range.contains(&self.file)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LocationState {
    Invalid,
    Empty,
    Opponent(Piece),
    Friendly,
}

#[derive(Debug)]
struct Move {
    piece: Piece,
    from: Location,
    to: Location,
    attack: Option<(Location, Piece)>,
    castle: Option<(Location, Location)>,
    promotion: Option<PieceType>,
}

impl Move {
    fn new(piece: Piece, from: Location, to: Location) -> Self {
        Self {
            piece,
            from,
            to,
            attack: None,
            castle: None,
            promotion: None,
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

    fn with_promotion(mut self, typ: PieceType) -> Self {
        self.promotion = Some(typ);
        self
    }
}

#[derive(Debug, Clone)]
struct Board {
    pieces: [[Option<Piece>; 8]; 8],
    active: Color,
    wk_castle: bool,
    wq_castle: bool,
    bk_castle: bool,
    bq_castle: bool,
    en_passant: Option<Location>,
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

impl Board {
    fn new() -> Self {
        Self {
            pieces: INIT_PIECES,
            active: Color::White,
            wk_castle: true,
            wq_castle: true,
            bk_castle: true,
            bq_castle: true,
            en_passant: None,
        }
    }

    fn iter_locations(&self) -> impl Iterator<Item = (Location, Option<&Piece>)> {
        self.pieces.iter().enumerate().flat_map(|(rank, pieces)| {
            pieces.iter().enumerate().map(move |(pile, piece)| {
                let loc = Location::new(rank as i8, pile as i8);
                (loc, piece.as_ref())
            })
        })
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

    fn piece(&self, loc: Location) -> Option<Piece> {
        self.pieces[loc.rank as usize][loc.file as usize]
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

    fn can_attack_king(&self, color: Color) -> bool {
        for (loc, piece) in self.iter_pieces_of(color) {
            let moves = self.possible_moves(loc, *piece, false);
            for mv in moves {
                if let Some((_, piece)) = mv.attack {
                    if piece.typ == PieceType::King {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn reachable_locations(&self, color: Color) -> HashSet<Location> {
        let mut locs = HashSet::new();
        for (loc, piece) in self.iter_pieces_of(color) {
            let moves = self.possible_moves(loc, *piece, false);
            for mv in moves {
                locs.insert(mv.to);
            }
        }
        locs
    }

    fn all_possible_moves(&self, color: Color, safe: bool) -> HashMap<Location, Vec<Move>> {
        self.iter_pieces_of(color)
            .map(|(loc, piece)| (loc, self.possible_moves(loc, *piece, safe)))
            .collect()
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
                let dirs = all_directions(1, 1);
                let mut moves = self.moves_to_directions(loc, piece, &dirs, false);
                let kingside = if color.is_white() {
                    self.wk_castle
                } else {
                    self.bk_castle
                };
                if kingside
                    && self.get_state(loc + (0, 1), color) == LocationState::Empty
                    && self.get_state(loc + (0, 2), color) == LocationState::Empty
                {
                    moves.push(
                        Move::new(piece, loc, loc + (0, 2)).with_castle(loc + (0, 3), loc + (0, 1)),
                    );
                }
                let queenside = if color.is_white() {
                    self.wq_castle
                } else {
                    self.bq_castle
                };
                if queenside
                    && self.get_state(loc + (0, -1), color) == LocationState::Empty
                    && self.get_state(loc + (0, -2), color) == LocationState::Empty
                {
                    moves.push(
                        Move::new(piece, loc, loc + (0, -2))
                            .with_castle(loc + (0, -4), loc + (0, -1)),
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
                    let reachables = self.reachable_locations(opponent);
                    if reachables.contains(&mv.from) {
                        return false;
                    }
                    let dx = mv.to.file - mv.from.file;
                    if reachables.contains(&(mv.from + (0, dx / 2))) {
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
                match self.get_state(loc, piece.color) {
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

    fn piece_moved(&self, mv: &Move) -> Board {
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
                if piece.color.is_white() {
                    if mv.from.rank == 0 {
                        if mv.from.file == 0 {
                            new_board.wq_castle = false;
                        } else if mv.from.file == 7 {
                            new_board.wk_castle = false;
                        }
                    }
                } else if mv.from.rank == 7 {
                    if mv.from.file == 0 {
                        new_board.bq_castle = false;
                    } else if mv.from.file == 7 {
                        new_board.bk_castle = false;
                    }
                }
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
        if let Some(typ) = mv.promotion {
            piece.typ = typ;
        }
        new_board.set_piece(mv.to, Some(piece));
        new_board
    }

    fn move_piece(&mut self, mv: &Move) {
        *self = self.piece_moved(mv);
    }
}

fn all_directions(dx: i8, dy: i8) -> Vec<(i8, i8)> {
    let mut dirs = vec![];
    let swap = dx != dy;
    dirs.push((dx, dy));
    if swap {
        dirs.push((dy, dx));
    }
    if dx != 0 {
        dirs.push((-dx, dy));
        if swap {
            dirs.push((dy, -dx));
        }
    }
    if dy != 0 {
        dirs.push((dx, -dy));
        if swap {
            dirs.push((dy, dx));
        }
    }
    if dx != 0 && dy != 0 {
        dirs.push((-dx, -dy));
        if swap {
            dirs.push((-dy, -dx));
        }
    }
    dirs
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

pub fn run() {
    let game = Board::new();
    println!("{}", game);
}
