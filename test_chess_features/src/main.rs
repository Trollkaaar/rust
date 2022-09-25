use rand::Rng;
use std::num::Wrapping;

fn main() {
    // let pawn_attack_table: [[u64; 64]; 2] = generate_pawn_attack_tables();
    // let knight_attack_table: [u64; 64] = generate_knight_attack_tables();
    // let king_attack_table: [u64; 64] = generate_king_attack_tables();
    let mut occupancy = 0u64;
    occupancy |= (1u64 << SquareLabels::C4 as usize);
    occupancy |= (1u64 << SquareLabels::C4 as usize);
    occupancy |= (1u64 << SquareLabels::C4 as usize);
    let bishop_masks: [u64; 64] = generate_bishop_masks();
    let bishop_attack_table: [[u64; 64]; 512] = generate_bishop_attack_tables();
    println!(
        "{}",
        get_bishop_attacks(
            SquareLabels::D4 as usize,
            occupancy,
            bishop_masks,
            bishop_attack_table
        )
    );

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
    // for square_index in 0..64 {
    //     println!(
    //         "{}u64,",
    //         find_magic_number(
    //             square_index,
    //             ROOK_OCCUPANCY_BIT_COUNT[square_index],
    //             Pieces::ROOK
    //         )
    //     );
    // }
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
    36037630228037648u64,
    4774378559799033856u64,
    149322495817191424u64,
    153263141998823429u64,
    8797286826048u64,
    13582404589602818u64,
    2308130004438876188u64,
    579416239559589960u64,
    2533858905949761u64,
    35261752803361u64,
    73202202823790592u64,
    144150995755352064u64,
    1163248669991895112u64,
    288232592360079489u64,
    9333710365164834816u64,
    1128101094375436u64,
    2305846376608563200u64,
    1155182101051552904u64,
    220677756264906784u64,
    2305844108726370312u64,
    2414356010916513954u64,
    297312685794592002u64,
    594475331750068608u64,
    4755994720549732864u64,
    19149095650460160u64,
    36028801322319904u64,
    141321704579072u64,
    650923768561792u64,
    71519795414016u64,
    2308094895199388672u64,
    4725050342322995200u64,
    70437476368672u64,
    864691128731959297u64,
    9241949385319780632u64,
    601725744451600u64,
    149474482896240972u64,
    18014398517903360u64,
    2378050137120346114u64,
    144256012537561124u64,
    1154047408817045568u64,
    1157993551783575561u64,
    73746443898454016u64,
    288265577777102848u64,
    9223373205390516481u64,
    297237575540679168u64,
    13835339532544278533u64,
    4415268323673u64,
    2200147492864u64,
    4503771426652544u64,
    306247008044187852u64,
    72409437792372000u64,
    153141087620304896u64,
    2306405961855926530u64,
    6088886693707186179u64,
    40532405597241376u64,
    2344124707168583706u64,
    583216289183467008u64,
    144475914862870528u64,
    2342197301402928424u64,
    4512464574353412u64,
    6931338893988990979u64,
    162129655304849410u64,
    319793046092776448u64,
    90073710802911491u64,
];

const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    576461174284165120u64,
    842090422529u64,
    4785624362287108u64,
    602772906967112u64,
    9262788705200898576u64,
    2306054252901957633u64,
    288082787043330u64,
    2456714146620805137u64,
    81069745392370720u64,
    79132539047905696u64,
    2449958764848087200u64,
    1152980878789083136u64,
    9229001538028963488u64,
    306808965881266176u64,
    4621825718995649632u64,
    1008948704494878784u64,
    2395995354442006560u64,
    9245995725618151488u64,
    4644895466323977u64,
    864691129998639113u64,
    144467049649539108u64,
    9511887186586244136u64,
    36184927670902792u64,
    4647930320262267073u64,
    225399902149019652u64,
    2310488170963271809u64,
    145720759812488u64,
    10380797424559084544u64,
    4710835581792293408u64,
    5375232u64,
    4790606524317760u64,
    37163493556368192u64,
    1306659637776367624u64,
    422212466119713u64,
    144132780841241088u64,
    1160820396143935617u64,
    9223394323508527104u64,
    2179760362797148160u64,
    577204022163832992u64,
    72057594643136528u64,
    914804418019361u64,
    9710927901442048u64,
    282678641368064u64,
    5209574190906081344u64,
    126260287742410752u64,
    5514740130816u64,
    586031056199647632u64,
    39878771397638u64,
    9369761601508352128u64,
    1164288290286731269u64,
    4616225627664613376u64,
    13979314393165512848u64,
    9944230002009833608u64,
    39706973438464u64,
    153298379160420416u64,
    844699909226496u64,
    4611686568452162768u64,
    72075195619346450u64,
    288234783064990720u64,
    4538801187889152u64,
    4648700051182356528u64,
    73791532469191808u64,
    1155178819160244230u64,
    225207469178101904u64,
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

fn get_random_u32_number() -> u32 {
    let mut rng = rand::thread_rng();
    let r: u32 = rng.gen();
    r
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
fn generate_bishop_masks() -> [u64; 64] {
    let mut bishop_masks: [u64; 64] = [0; 64];
    for x in 0..64 {
        bishop_masks[x] = mask_bishop_attack(x);
    }
    bishop_masks
}
fn generate_bishop_attack_tables() -> [[u64; 64]; 512] {
    let mut bishop_attacks: [[u64; 64]; 512] = [[0u64; 64]; 512];
    let mut bishop_masks: [u64; 64] = [0u64; 64];
    for square_index in 0..64 {
        bishop_masks[square_index] = mask_bishop_attack(square_index);
        let attack_mask = bishop_masks[square_index];
        let bit_count_in_mask = count_bits(attack_mask);
        let occupancy_indicies = (1 << bit_count_in_mask);

        let index = 0;
        while (index < occupancy_indicies) {
            let occupancy = occupancy(index, bit_count_in_mask, attack_mask);
            let magic_index = (occupancy * BISHOP_MAGIC_NUMBERS[square_index])
                >> (64 - BISHOP_OCCUPANCY_BIT_COUNT[square_index]);
            let tmp: usize = magic_index.try_into().unwrap();
            bishop_attacks[square_index][tmp] =
                mask_bishop_attack_with_blocking_pieces(square_index, occupancy);
        }
    }
    bishop_attacks
}
fn get_bishop_attacks(
    square: usize,
    occupancy: u64,
    bishop_masks: [u64; 64],
    bishop_attack_table: [[u64; 64]; 512],
) -> u64 {
    occupancy &= bishop_masks[square];
    occupancy *= BISHOP_MAGIC_NUMBERS[square];
    occupancy >>= 64 - BISHOP_OCCUPANCY_BIT_COUNT[square];

    let tmp: usize = occupancy.try_into().unwrap();
    return bishop_attack_table[square][tmp];
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
