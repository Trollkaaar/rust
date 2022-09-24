use rand::Rng;
use std::num::Wrapping;

fn main() {
    // let pawn_attack_table: [[u64; 64]; 2] = generate_pawn_attack_tables();
    // let knight_attack_table: [u64; 64] = generate_knight_attack_tables();
    // let king_attack_table: [u64; 64] = generate_king_attack_tables();
    // let bishop_attack_table: [u64; 64] = generate_bishop_attack_tables();
    // let white_pawn = 1u64 << 8;

    // let mut attack: u64 = mask_bishop_attack(SquareLabels::D4 as usize);
    // for x in 4000..4095 {
    //     let o: u64 = occupancy(x, count_bits(attack), attack);
    //     print_bitboard(o);
    // }

    // for x in 0..8 {
    //     for y in 0..8 {
    //         // let square_index = x * 8 + y;
    //         // print!("{},", count_bits(mask_rook_attack(square_index)))
    //     }
    //     println!("")
    // }
    // print_bitboard(get_random_u64_number_with_fewer_nonzero());
    // print_bitboard(mask_rook_attack(SquareLabels::D4 as usize));
    // let mut blocking_pieces: u64 = 0u64;
    // blocking_pieces |= (1u64 << SquareLabels::C4 as usize);
    // blocking_pieces |= (1u64 << SquareLabels::D7 as usize);
    // blocking_pieces |= (1u64 << SquareLabels::D2 as usize);
    // blocking_pieces |= (1u64 << SquareLabels::E4 as usize);
    // print_bitboard(blocking_pieces);
    // println!(
    //     "{:?}",
    //     Squares[get_index_of_least_significant_bit(blocking_pieces)]
    // );
    // println!(
    //     "{}",
    //     CONVERT_INDEX_COORDINATE[get_index_of_least_significant_bit(blocking_pieces)]
    // );

    // println!("{}", count_bits(blocking_pieces));
    // print_bitboard(mask_rook_attack_with_blocking_pieces(
    //     SquareLabels::D4 as usize,
    //     blocking_pieces,
    // ));
    // print_bitboard(mask_bishop_attack_with_blocking_pieces(x, 0u64));
    for square_index in 0..64 {
        println!(
            "{}u64,",
            find_magic_number(
                square_index,
                BISHOP_OCCUPANCY_BIT_COUNT[square_index],
                Pieces::BISHOP
            )
        );
    }
    // println!("{:?}", bishop_attack_table);

    // let mut bb = 0u64;
    // for x in (1..9).rev() {
    //     print!("\"A{}\",", x);
    //     print!("\"B{}\",", x);
    //     print!("\"C{}\",", x);
    //     print!("\"D{}\",", x);
    //     print!("\"E{}\",", x);
    //     print!("\"F{}\",", x);
    //     print!("\"G{}\",", x);
    //     print!("\"H{}\",", x);
    //     println!("");
    // }
    // println!("{}", bb);
    // print_bitboard(bb)
}

const NOT_A: u64 = 18374403900871474942;
const NOT_AB: u64 = 18229723555195321596;
const NOT_H: u64 = 9187201950435737471;
const NOT_GH: u64 = 4557430888798830399;

const CONVERT_INDEX_COORDINATE: [&str; 64] = [
    "A8", "B8", "C8", "D8", "E8", "F8", "G8", "H8", "A7", "B7", "C7", "D7", "E7", "F7", "G7", "H7",
    "A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6", "A5", "B5", "C5", "D5", "E5", "F5", "G5", "H5",
    "A4", "B4", "C4", "D4", "E4", "F4", "G4", "H4", "A3", "B3", "C3", "D3", "E3", "F3", "G3", "H3",
    "A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2", "A1", "B1", "C1", "D1", "E1", "F1", "G1", "H1",
];

const BISHOP_OCCUPANCY_BIT_COUNT: [usize; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

const ROOK_OCCUPANCY_BIT_COUNT: [usize; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

const ROOK_MAGIC_NUMBERS: [u64; 64] = [
    2305904590471561224u64,
    10376860958332026928u64,
    1152995176181923970u64,
    578713102946672672u64,
    36057388700140580u64,
    1297036796298789024u64,
    2305845242598916160u64,
    1134702728396804u64,
    1161964987745567104u64,
    1807438788545282067u64,
    703687450699264u64,
    44066365509898u64,
    13835493530608738305u64,
    2814818488680544u64,
    13835059017522611456u64,
    4634204575178688512u64,
    170019716455203073u64,
    1486187928571875332u64,
    144678688070308226u64,
    2407005100720128u64,
    11898514750999494656u64,
    577164647146229888u64,
    594897449178497072u64,
    576469548397494273u64,
    3603060045427769345u64,
    288232596662387200u64,
    2314852424939995152u64,
    1152930455318692864u64,
    576531292980527104u64,
    288230582478970912u64,
    36033332547436544u64,
    108376681453977665u64,
    378302682236388098u64,
    9034688119227266u64,
    1170937002653648896u64,
    10376328736722157568u64,
    85854405529698640u64,
    153263125905295360u64,
    2463047780143616u64,
    2315440096456999200u64,
    9009435072534528u64,
    434599702789620753u64,
    9250820245148434437u64,
    579820691520u64,
    1970324837011480u64,
    72075618441035776u64,
    2305985958611779616u64,
    2324069132431392800u64,
    2308094809094918672u64,
    2445863634993161u64,
    70369583300612u64,
    2305950761420488896u64,
    9007491362848784u64,
    9223583147416616961u64,
    5070982264922688u64,
    4615204601954697728u64,
    458496415907844u64,
    723201575258947584u64,
    92359110983785u64,
    1134081098240u64,
    11529223920562996096u64,
    576610285889004613u64,
    9241390086088564800u64,
    288239172282744864u64,
];

const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    1208095582204199042u64,
    4644341410829312u64,
    1450300917023768576u64,
    1310618934050882u64,
    216332765350594560u64,
    360375933938959363u64,
    9024800038060160u64,
    7205830949362484226u64,
    108099594629677120u64,
    3603060045427769345u64,
    36108649051455488u64,
    2315440096456999200u64,
    14781869086212098u64,
    1441151949546520576u64,
    103080268288u64,
    297263964448894986u64,
    4612899923023630338u64,
    4540436219707681u64,
    4612110447145910370u64,
    1522221072903055360u64,
    4503599636484224u64,
    2341911946997532674u64,
    5229242139883339848u64,
    2308094809094918672u64,
    5782640202002532128u64,
    54324688263849088u64,
    9979986669861864450u64,
    579838589465001984u64,
    441354962509007168u64,
    549906841600u64,
    36311543910113290u64,
    2954924409689473760u64,
    288663858612011520u64,
    157907642323845256u64,
    81135574489024640u64,
    324338338075246613u64,
    9007223426809858u64,
    42996550097698848u64,
    2305852081325478913u64,
    55177892769828928u64,
    23749592898111504u64,
    2305985466838548609u64,
    288234791424229380u64,
    7062780568500441138u64,
    2377938124123668996u64,
    1226104999793137937u64,
    2322237311163528u64,
    8646920082792939912u64,
    290905489053712448u64,
    9440180388246340096u64,
    4638777996748009472u64,
    5234308686440038408u64,
    5476526817903166017u64,
    144115778902556672u64,
    1299314883997745160u64,
    9228517819994408960u64,
    2598577538515484676u64,
    4684905016190902273u64,
    5188146779861950592u64,
    18612567598174240u64,
    2524285363720111104u64,
    1874060567275118624u64,
    15600471312574185732u64,
    72058143927978368u64,
];

fn print_bitboard(bb: u64) {
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
        println!("");
    }
    println!("  A B C D E F G H\n");
}

static mut seed: u32 = 1804289383;
fn get_random_u32_number() -> u32 {
    // let mut rng = rand::thread_rng();
    // let r: u32 = rng.gen();
    // r
    unsafe {
        let mut number = seed;
        number ^= number << 13;
        number ^= number >> 17;
        number ^= number << 5;

        seed = number;
        return number;
    }
}
///get random u64 by u32, algorithm courtesy of Tord Romstad
fn get_random_u64_number() -> u64 {
    let r1: u64;
    let r2: u64;
    let r3: u64;
    let r4: u64;

    r1 = (get_random_u32_number() as u64) & 0xFFFF;
    r2 = (get_random_u32_number() as u64) & 0xFFFF;
    r3 = (get_random_u32_number() as u64) & 0xFFFF;
    r4 = (get_random_u32_number() as u64) & 0xFFFF;

    return r1 | (r2 << 16) | (r3 << 32) | (r4 << 48);
}

///Courtesy of Tord Romstad
fn get_random_u64_number_with_fewer_nonzero() -> u64 {
    return get_random_u64_number() & get_random_u64_number() & get_random_u64_number();
}

///this is the most convoluted thing every but it works great, courtesy of Tord Romstad
fn find_magic_number(square_index: usize, bits: usize, bishop: usize) -> u64 {
    let mut occupancies: [u64; 4096] = [0; 4096];
    let mut attacks: [u64; 4096] = [0; 4096];
    let mut used_attacks: [u64; 4096] = [0u64; 4096];
    let attack_mask: u64 = if (bishop == Pieces::BISHOP) {
        mask_bishop_attack(square_index)
    } else {
        mask_rook_attack(square_index)
    };

    let occupancy_indicies = 1 << bits;

    let mut index = 0;
    while (index < occupancy_indicies) {
        occupancies[index] = occupancy(index, bits, attack_mask);
        attacks[index] = if (bishop == Pieces::BISHOP) {
            mask_bishop_attack_with_blocking_pieces(square_index, occupancies[index])
        } else {
            mask_rook_attack_with_blocking_pieces(square_index, occupancies[index])
        };

        index += 1;
    }

    let mut i = 0;
    while (i < 100000000) {
        let magic_number = get_random_u64_number_with_fewer_nonzero();

        if (count_bits((attack_mask * magic_number) & 0xFF00000000000000) < 6) {
            continue;
        }

        let mut index = 0;
        let mut fail = 0;
        while (fail == 0 && index < occupancy_indicies) {
            let magic_index = (occupancies[index] * magic_number) >> (64 - bits);

            let tmp: usize = magic_index.try_into().unwrap();

            if (used_attacks[tmp] == 0u64) {
                used_attacks[tmp] = attacks[index];
            } else if (used_attacks[tmp] != attacks[index]) {
                fail = 1;
            }
            index += 1;
        }

        if (fail != 0) {
            return magic_number;
        }

        i += 1;
    }
    println!("***ERROR***");
    return 0u64;
}

///Brian Kerninghnan's Algorithm
fn count_bits(bb: u64) -> usize {
    let mut bit_count: usize = 0;

    let mut tmp_bb = bb;
    while (tmp_bb > 0) {
        tmp_bb &= tmp_bb - 1;
        bit_count += 1;
    }
    bit_count
}

fn get_index_of_least_significant_bit(bb: u64) -> usize {
    if bb == 0 {
        return 0;
    }
    return count_bits(bb ^ (bb - 1));
}
///Generates an occupancy map depending on the given index/seed, bit_count and attack_mask, courtesy of https://www.chessprogramming.org/Magic_Bitboards
fn occupancy(index: usize, bit_count_in_mask: usize, mut attack_mask: u64) -> u64 {
    let mut occupancy_map = 0u64;

    let mut count = 0;
    while count < bit_count_in_mask {
        let square_index = get_index_of_least_significant_bit(attack_mask);

        if (attack_mask & (1u64 << square_index - 1) != 0) {
            attack_mask = attack_mask ^ (1u64 << square_index - 1);
        } else {
            0;
        }
        if (index & (1 << count) != 0) {
            occupancy_map |= (1u64 << square_index - 1);
        }

        count += 1;
    }

    occupancy_map
}

fn mask_pawn_attack(side: usize, square_index: usize) -> u64 {
    let mut pawn_attacks = 0u64;
    let mut bb = 0u64;
    bb |= 1u64 << square_index;
    if side == Sides::WHITE {
        if (bb >> 7) & NOT_A != 0 {
            pawn_attacks |= bb >> 7
        }
        if (bb >> 9) & NOT_H != 0 {
            pawn_attacks |= bb >> 9
        }
    } else {
        if (bb << 7) & NOT_H != 0 {
            pawn_attacks |= bb << 7
        }
        if (bb << 9) & NOT_A != 0 {
            pawn_attacks |= bb << 9
        }
    }

    pawn_attacks
}
fn mask_knight_attack(square_index: usize) -> u64 {
    let mut pawn_attacks = 0u64;
    let mut bb = 0u64;
    bb |= 1u64 << square_index;

    if (bb >> 17) & NOT_H != 0 {
        pawn_attacks |= bb >> 17
    }
    if (bb >> 15) & NOT_A != 0 {
        pawn_attacks |= bb >> 15
    }
    if (bb >> 10) & NOT_GH != 0 {
        pawn_attacks |= bb >> 10
    }
    if (bb >> 6) & NOT_AB != 0 {
        pawn_attacks |= bb >> 6
    }

    if (bb << 17) & NOT_A != 0 {
        pawn_attacks |= bb << 17
    }
    if (bb << 15) & NOT_H != 0 {
        pawn_attacks |= bb << 15
    }
    if (bb << 10) & NOT_AB != 0 {
        pawn_attacks |= bb << 10
    }
    if (bb << 6) & NOT_GH != 0 {
        pawn_attacks |= bb << 6
    }

    pawn_attacks
}

fn mask_king_attack(square_index: usize) -> u64 {
    let mut king_attacks = 0u64;
    let mut bb = 0u64;
    bb |= 1u64 << square_index;

    if (bb >> 9) & NOT_H != 0 {
        king_attacks |= bb >> 9
    }
    if (bb >> 8) != 0 {
        king_attacks |= bb >> 8
    }
    if (bb >> 7) & NOT_A != 0 {
        king_attacks |= bb >> 7
    }
    if (bb >> 1) & NOT_H != 0 {
        king_attacks |= bb >> 1
    }

    if (bb << 9) & NOT_A != 0 {
        king_attacks |= bb << 9
    }
    if (bb << 8) != 0 {
        king_attacks |= bb << 8
    }
    if (bb << 7) & NOT_H != 0 {
        king_attacks |= bb << 7
    }
    if (bb << 1) & NOT_A != 0 {
        king_attacks |= bb << 1
    }

    king_attacks
}

///Magic, https://www.chessprogramming.org/Magic_Bitboards
fn mask_bishop_attack(square_index: usize) -> u64 {
    let mut bishop_attacks = 0u64;
    let tmp: i8 = square_index.try_into().unwrap();
    let target_rank: i8 = tmp / 8;
    let target_file: i8 = tmp % 8;

    let mut r: i8;
    let mut f: i8;

    r = target_rank + 1;
    f = target_file + 1;
    while r <= 6 && f <= 6 {
        bishop_attacks |= 1u64 << (r * 8 + f);
        r += 1;
        f += 1;
    }

    r = target_rank - 1;
    f = target_file + 1;
    while r >= 1 && f <= 6 {
        bishop_attacks |= 1u64 << (r * 8 + f);
        r -= 1;
        f += 1;
    }

    r = target_rank + 1;
    f = target_file - 1;
    while r <= 6 && f >= 1 {
        bishop_attacks |= 1u64 << (r * 8 + f);
        r += 1;
        f -= 1;
    }

    r = target_rank - 1;
    f = target_file - 1;
    while r >= 1 && f >= 1 {
        bishop_attacks |= 1u64 << (r * 8 + f);
        r -= 1;
        f -= 1;
    }

    bishop_attacks
}

fn mask_bishop_attack_with_blocking_pieces(square_index: usize, blocking_pieces: u64) -> u64 {
    let mut bishop_attacks = 0u64;
    let tmp: i8 = square_index.try_into().unwrap();
    let target_rank: i8 = tmp / 8;
    let target_file: i8 = tmp % 8;

    let mut r: i8;
    let mut f: i8;

    r = target_rank + 1;
    f = target_file + 1;
    while r <= 7 && f <= 7 {
        bishop_attacks |= 1u64 << (r * 8 + f);
        if 1u64 << (r * 8 + f) & blocking_pieces != 0 {
            break;
        }
        r += 1;
        f += 1;
    }

    r = target_rank - 1;
    f = target_file + 1;
    while r >= 0 && f <= 7 {
        bishop_attacks |= 1u64 << (r * 8 + f);
        if (1u64 << (r * 8 + f) & blocking_pieces) != 0 {
            break;
        }
        r -= 1;
        f += 1;
    }

    r = target_rank + 1;
    f = target_file - 1;
    while r <= 7 && f >= 0 {
        bishop_attacks |= 1u64 << (r * 8 + f);
        if 1u64 << (r * 8 + f) & blocking_pieces != 0 {
            break;
        }
        r += 1;
        f -= 1;
    }

    r = target_rank - 1;
    f = target_file - 1;
    while r >= 0 && f >= 0 {
        bishop_attacks |= 1u64 << (r * 8 + f);
        if 1u64 << (r * 8 + f) & blocking_pieces != 0 {
            break;
        }
        r -= 1;
        f -= 1;
    }

    bishop_attacks
}

///Magic, https://www.chessprogramming.org/Magic_Bitboards
fn mask_rook_attack(square_index: usize) -> u64 {
    let mut rook_attacks = 0u64;
    let tmp: i8 = square_index.try_into().unwrap();
    let target_rank: i8 = tmp / 8;
    let target_file: i8 = tmp % 8;

    let mut r: i8;
    let mut f: i8;

    r = target_rank + 1;
    while r <= 6 {
        rook_attacks |= 1u64 << (r * 8 + target_file);
        r += 1;
    }

    r = target_rank - 1;
    while r >= 1 {
        rook_attacks |= 1u64 << (r * 8 + target_file);
        r -= 1;
    }

    f = target_file + 1;
    while f <= 6 {
        rook_attacks |= 1u64 << (target_rank * 8 + f);
        f += 1;
    }

    f = target_file - 1;
    while f >= 1 {
        rook_attacks |= 1u64 << (target_rank * 8 + f);
        f -= 1;
    }

    rook_attacks
}
fn mask_rook_attack_with_blocking_pieces(square_index: usize, blocking_pieces: u64) -> u64 {
    let mut rook_attacks = 0u64;
    let tmp: i8 = square_index.try_into().unwrap();
    let target_rank: i8 = tmp / 8;
    let target_file: i8 = tmp % 8;

    let mut r: i8;
    let mut f: i8;

    r = target_rank + 1;
    while r <= 7 {
        rook_attacks |= 1u64 << (r * 8 + target_file);
        if 1u64 << (r * 8 + target_file) & blocking_pieces != 0 {
            break;
        }
        r += 1;
    }

    r = target_rank - 1;
    while r >= 0 {
        rook_attacks |= 1u64 << (r * 8 + target_file);
        if 1u64 << (r * 8 + target_file) & blocking_pieces != 0 {
            break;
        }
        r -= 1;
    }

    f = target_file + 1;
    while f <= 7 {
        rook_attacks |= 1u64 << (target_rank * 8 + f);
        if 1u64 << (target_rank * 8 + f) & blocking_pieces != 0 {
            break;
        }
        f += 1;
    }

    f = target_file - 1;
    while f >= 0 {
        rook_attacks |= 1u64 << (target_rank * 8 + f);
        if 1u64 << (target_rank * 8 + f) & blocking_pieces != 0 {
            break;
        }
        f -= 1;
    }

    rook_attacks
}

fn generate_pawn_attack_tables() -> [[u64; 64]; 2] {
    let mut pawn_attack_table_white: [u64; 64] = [0; 64];
    let mut pawn_attack_table_black: [u64; 64] = [0; 64];
    for x in 0..64 {
        pawn_attack_table_white[x] = mask_pawn_attack(Sides::WHITE, x);
        pawn_attack_table_black[x] = mask_pawn_attack(Sides::BLACK, x);
    }
    [pawn_attack_table_white, pawn_attack_table_black]
}

fn generate_knight_attack_tables() -> [u64; 64] {
    let mut knight_attack_table: [u64; 64] = [0; 64];
    for x in 0..64 {
        knight_attack_table[x] = mask_knight_attack(x);
    }
    knight_attack_table
}
fn generate_king_attack_tables() -> [u64; 64] {
    let mut king_attack_table: [u64; 64] = [0; 64];
    for x in 0..64 {
        king_attack_table[x] = mask_king_attack(x);
    }
    king_attack_table
}
fn generate_bishop_attack_tables() -> [u64; 64] {
    let mut bishop_attack_table: [u64; 64] = [0; 64];
    for x in 0..64 {
        bishop_attack_table[x] = mask_bishop_attack(x);
    }
    bishop_attack_table
}

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
    bb_sides: [u64; 2],
    bb_pieces: [[u64; 6]; 2],
}

pub struct AttackTables {
    attack_table: [[u64; 64]; 2],
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
