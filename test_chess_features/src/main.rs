fn main() {
    let white_pawn = 1u64 << 8;
    print_bitboard(mask_pawn_attack(Sides::WHITE, SquareLabels::A8 as u8));

    // let mut bb = 0u64;
    // for x in 0..8 {
    //     for y in 0..8 {
    //         let square_index = (x << 3) + y;
    //         if (y < 6) {
    //             bb |= (1u64 << square_index);
    //         }
    //     }
    // }
    // println!("{}", bb);
    // print_bitboard(bb)
}
const not_a: u64 = 18374403900871474942;
const not_ab: u64 = 18229723555195321596;
const not_h: u64 = 9187201950435737471;
const not_gh: u64 = 4557430888798830399;

fn print_bitboard(bb: u64) {
    println!("works");
    for rank in 0..8 {
        print!("{} ", 8 - rank);
        for file in 0..8 {
            let square_index = (rank << 3) + file;
            print!(
                "{} ",
                if bb & (1u64 << square_index) != 0 {
                    1
                } else {
                    0
                }
            );
        }
        println!();
    }
    println!("  A B C D E F G H");
}
pub fn mask_pawn_attack(side: usize, square_index: u8) -> u64 {
    let mut pawn_attacks = 0u64;
    let mut bb = 0u64;
    bb |= (1u64 << square_index);
    print_bitboard(bb);
    if (side == Sides::WHITE) {
        // if()
    } else {
    }

    pawn_attacks
}
pub struct BitBoard(pub u64);

pub struct Sides;
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

pub struct Pieces;
impl Pieces {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}
pub struct Position {
    bb_sides: [BitBoard; 2],
    bb_pieces: [[BitBoard; 6]; 2],
}

pub enum SquareLabels {
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
}
