use rand::Rng;
use std::{io, num::Wrapping};

fn main() {
    let mut game = Game::new();
    game.init_bitboard();
    game.update_occupancy();
    let moves = game.get_possible_moves();
    assert_eq!(moves.len(), 20)
    // let mut game = Game::new();
    // game.init_bitboard();
    // game.update_occupancy();
    // game.print_board();

    // // while (game.state == GameState::InProgress) {
    // //     let mut input: String = String::new();
    // //     io::stdin().read_line(&mut input).expect("Error");
    // //     let parsed_move = parse_move(input.as_str(), &game);
    // //     if (parsed_move._move != 0) {
    // //         internal_make_move(parsed_move, &mut game);
    // //     }
    // //     game.print_board();
    // // }
    // let tmp = game.get_possible_moves();
    // for i in tmp {
    //     println!("{:?}", i.print());
    // }
    // // game.make_move("A2A4");
    // // game.print_board();
}

const ASCII_PIECES: [char; 13] = [
    'P', 'B', 'N', 'R', 'Q', 'K', 'p', 'b', 'n', 'r', 'q', 'k', '0',
];

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

/**
I will shamelessly admit that these aren't my own magic numbers since i just could not get it to work, don't say i didn't try tho cuz god knows i sat till 3 am trying
https://github.com/mvanthoor/rustic/blob/master/src/movegen/magics.rs
*/
const my_own_ROOK_MAGIC_NUMBERS: [u64; 64] = [
    3459345060259299340u64,
    9268408187748384800u64,
    1152956710994837570u64,
    148619200024281251u64,
    72623851183277056u64,
    306244774669582976u64,
    562949999559430u64,
    1369164662985261952u64,
    703758846689408u64,
    3171730478260895747u64,
    18304738701352974u64,
    1729525193708160000u64,
    288230513796350049u64,
    4688810162214928640u64,
    9096538886637312u64,
    180566215142408200u64,
    9223409425350395906u64,
    36736178401083456u64,
    2468043033262981184u64,
    343603742920u64,
    1209216508547317936u64,
    72057741274792208u64,
    4774454421281112064u64,
    2612655269331124224u64,
    164395405172539968u64,
    11529215046367316227u64,
    5531987614240u64,
    324822127557476928u64,
    15763149055590404u64,
    5350561150157210698u64,
    7147633018898u64,
    5769111226822950920u64,
    4921345128880144384u64,
    879614672175104u64,
    2252358167823364u64,
    5633897582952449u64,
    1140855808u64,
    144150419709886596u64,
    3386496887914528u64,
    881808661086211u64,
    1136912271147265u64,
    36882310200559620u64,
    565220381065216u64,
    1161928704935567904u64,
    9007199255035904u64,
    6917670899005030452u64,
    72058024621256704u64,
    4503599898428450u64,
    9367909446188007488u64,
    1442560338886656u64,
    4539334695452738u64,
    18142018011144u64,
    35201652633698u64,
    1152930300706162693u64,
    39582905139264u64,
    2375119187969u64,
    35244635848748u64,
    70557991437576u64,
    2382474846720901120u64,
    2382439404431475072u64,
    91216171969282368u64,
    1125909033683264u64,
    18304706088534080u64,
    621496749281771556u64,
];
const ROOK_MAGIC_NUMBERS: [u64; 64] = [
    324259448050975248u64,
    162139001189302336u64,
    4647750006529359880u64,
    144121785691422736u64,
    16176938657641660544u64,
    9367489423970945072u64,
    36051338366288384u64,
    36029147746665088u64,
    3518447965192208u64,
    4614078830617822340u64,
    9241949523864129664u64,
    11540615780106252u64,
    730287067600519297u64,
    144819425575437312u64,
    1225261127674627584u64,
    40814017656160512u64,
    594475700577118276u64,
    283675082228259u64,
    148058037853261952u64,
    14411662294658320384u64,
    2394186703782912u64,
    1157847866488718336u64,
    2306407062973841412u64,
    4576167411597460u64,
    2323857959626489888u64,
    18860477004136448u64,
    621497027752297522u64,
    3027553647748714496u64,
    9241953785514295424u64,
    1970363492082688u64,
    1729664285938024960u64,
    4836870457972064321u64,
    141012374650913u64,
    4652253601601699840u64,
    58687601506263040u64,
    281543780081672u64,
    1157433900411130112u64,
    81628378934806544u64,
    2310366730829959192u64,
    2900476768907429780u64,
    36558770110480u64,
    9042384969023488u64,
    180425597514743824u64,
    5487636764434923528u64,
    5766860422494879764u64,
    9224498487624761348u64,
    41702298761822218u64,
    45599234000551940u64,
    70370891935872u64,
    19210671497487104u64,
    387030266675328u64,
    289215847808893056u64,
    576469550545240192u64,
    1153216449143113729u64,
    9350715278336u64,
    288521763922764288u64,
    282782794268833u64,
    595672521157161122u64,
    436884352794689609u64,
    9241667927690743809u64,
    5188428314494240769u64,
    1157988067282792450u64,
    1152939243166828548u64,
    4611967569673330817u64,
];

const my_own_BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    883275074265825316u64,
    1153343888881156112u64,
    1226250134087008260u64,
    424694977142784u64,
    4759267008890046480u64,
    149252142979356928u64,
    13606590611456u64,
    9053398204679168u64,
    13835081154692252824u64,
    1152921504944750689u64,
    36049138790432834u64,
    9007406491636224u64,
    9223380970525171712u64,
    721702394609664004u64,
    597299538555830548u64,
    563508303446093u64,
    153140116955913344u64,
    1156299207014137856u64,
    577023702256861708u64,
    597018467773710337u64,
    1155173308986032130u64,
    27024073826500640u64,
    145258681914228738u64,
    4611826756888887296u64,
    4647769860285268544u64,
    3026419228869314560u64,
    4630898910422900768u64,
    595040369583063073u64,
    290768051138208768u64,
    70677984583696u64,
    20974872560665474u64,
    4909064881229013044u64,
    2315131856317653002u64,
    326590426926612482u64,
    2322203999866880u64,
    1152921590506206208u64,
    9223451340209981440u64,
    288372218654621908u64,
    734122063219917440u64,
    147169748993u64,
    29308582017766016u64,
    1127295780683776u64,
    2316540304976777280u64,
    73218953199007744u64,
    1155174409032581188u64,
    92325992462290992u64,
    1732197059022422032u64,
    8865355923584u64,
    397461401771279104u64,
    288327169733558273u64,
    145245494621831200u64,
    2377900611862593728u64,
    3783164426644097024u64,
    9511637602832494872u64,
    14123292279733242022u64,
    153193033135816704u64,
    1226104998771851528u64,
    1180506135407378432u64,
    2666130979545952272u64,
    562950088204800u64,
    369295206094211328u64,
    41131218166876228u64,
    288377711784427520u64,
    585476749849528358u64,
];

const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    2310454429704290569u64,
    37163502750244928u64,
    145330200115150856u64,
    573953659699200u64,
    9845999220824211456u64,
    574016004032512u64,
    10093699283674480640u64,
    2306407060834902016u64,
    2883575003184432136u64,
    1747410678824308864u64,
    9259405249167245312u64,
    936784527773139074u64,
    4629702641998381057u64,
    201028145628315697u64,
    4899992295377881088u64,
    4630405483133404688u64,
    153474299838154784u64,
    2286992943744036u64,
    434597432802681416u64,
    865817269052115456u64,
    9156750026475656u64,
    599823317909770240u64,
    4578375142474880u64,
    2308525819264500224u64,
    18596057879421451u64,
    18331093560345096u64,
    2305880392877736000u64,
    56602859688444160u64,
    5382084129205534724u64,
    5767422822691897608u64,
    283691220206592u64,
    144398865845093376u64,
    1163523824685120u64,
    20267333288223264u64,
    325489801822240u64,
    4755836425302245636u64,
    594475563668865152u64,
    1162496335329427604u64,
    9244765235704371236u64,
    576667461564269056u64,
    146371454722771202u64,
    426679365288452u64,
    13724105480340736u64,
    1152922330050364928u64,
    4620737202526097424u64,
    1316476062695166464u64,
    13981996823661781640u64,
    12430506881068303489u64,
    5193780677221351424u64,
    426612797737280u64,
    37445932288049152u64,
    1171147012042137601u64,
    504403227018657856u64,
    4629845569785954560u64,
    4686013077882208273u64,
    1154056209263894528u64,
    613054853085794304u64,
    9025075185721408u64,
    9571249324951568u64,
    10999715432448u64,
    290408795603472u64,
    10664524198170591488u64,
    5924513492108288u64,
    90511840181764112u64,
];

// static mut SIDE: usize = 0;
const SIDES: [&str; 2] = ["White", "Black"];

pub fn GameLoop() {}

fn parse_move(input: &str, game: &Game) -> LocalMove {
    let mut move_list = MoveList::init();
    move_list = generate_moves(move_list, game);

    let (head, tail) = input.trim().split_at(2);
    let tail_head: &str = "";
    let tail_tail: &str = "";

    let (source_sq_head, source_sq_tail) = head.split_at(1);
    let source_head_number: usize = get_value_from_letter(source_sq_head.trim());
    let source_sq_number = source_sq_tail.to_string().parse::<usize>().unwrap();

    let (tail_head, tail_tail) = tail.split_at(2);
    let (tail_head_head, tail_head_tail) = tail_head.split_at(1);
    let target_head_number: usize = get_value_from_letter(tail_head_head);
    let target_sq_number = tail_head_tail.to_string().parse::<usize>().unwrap();
    let mut pro_piece: usize = 12;
    if (input.len() > 4) {
        pro_piece = get_value_from_letter(tail_tail.trim());
    }

    let source_sq = source_head_number + (8 - source_sq_number) * 8;
    let target_sq = target_head_number + (8 - target_sq_number) * 8;
    for i in 0..move_list.count {
        let _move = move_list.moves[i];

        if (source_sq == _move.get_move_source()
            && target_sq == _move.get_move_target()
            && pro_piece == _move.get_move_promoted())
        {
            return _move;
        }
        if (source_sq == _move.get_move_source() && target_sq == _move.get_move_target()) {
            return _move;
        }
    }
    return LocalMove { _move: 0 };
}
fn get_value_from_letter(input: &str) -> usize {
    match input {
        "A" => return 0,
        "B" => return 1,
        "C" => return 2,
        "D" => return 3,
        "E" => return 4,
        "F" => return 5,
        "G" => return 6,
        "H" => return 7,
        "a" => return 0,
        "b" => return 1,
        "c" => return 2,
        "d" => return 3,
        "e" => return 4,
        "f" => return 5,
        "g" => return 6,
        "h" => return 7,
        "q" => return Pieces::queen as usize,
        "b" => return Pieces::bishop as usize,
        "n" => return Pieces::knight as usize,
        "r" => return Pieces::rook as usize,
        "Q" => return Pieces::QUEEN as usize,
        "B" => return Pieces::BISHOP as usize,
        "N" => return Pieces::KNIGHT as usize,
        "R" => return Pieces::ROOK as usize,
        _ => return 0,
    };
}
///largely inspired by https://github.com/jordanbray/chess, https://github.com/mkandalf/crust/tree/master/src and https://github.com/bluefeversoft/Vice_Chess_Engine/blob/master/Ch36.zip
fn generate_moves(mut move_list: MoveList, game: &Game) -> MoveList {
    let mut source_sq: usize = 0;
    let mut target_sq: usize = 0;
    let mut _bitboard: u64 = 0;
    let mut _attacks: u64 = 0;

    for piece in 0..12 {
        let mut current_move = LocalMove::init();
        _bitboard = game.bitboards[piece];

        if game.side == Sides::WHITE {
            if piece == Pieces::PAWN {
                while (_bitboard != 0) {
                    source_sq = get_index_of_least_significant_bit(_bitboard);
                    target_sq = source_sq - 8;

                    if ((target_sq > SquareLabels::A8 as usize)
                        && ((game.occupancies[Sides::BOTH] & (1u64 << target_sq)) == 0))
                    {
                        if (source_sq >= SquareLabels::A7 as usize
                            && source_sq <= SquareLabels::H7 as usize)
                        {
                            // println!(
                            //     "Promote Pawn: {} -> {}\nQueen\nRook\nBishop\nKnight",
                            //     CONVERT_INDEX_COORDINATE[source_sq],
                            //     CONVERT_INDEX_COORDINATE[target_sq]
                            // );
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::QUEEN,
                                0,
                                0,
                            );
                            move_list.add(current_move);

                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::KNIGHT,
                                0,
                                0,
                            );
                            move_list.add(current_move);

                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::BISHOP,
                                0,
                                0,
                            );
                            move_list.add(current_move);

                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::ROOK,
                                0,
                                0,
                            );
                            move_list.add(current_move);
                        } else {
                            // println!(
                            //     "Push Pawn: {} -> {}",
                            //     CONVERT_INDEX_COORDINATE[source_sq],
                            //     CONVERT_INDEX_COORDINATE[target_sq]
                            // );
                            current_move.encode_move(source_sq, target_sq, piece, 12, 0, 0);
                            move_list.add(current_move);

                            if ((source_sq >= SquareLabels::A2 as usize
                                && source_sq <= SquareLabels::H2 as usize)
                                && ((game.occupancies[Sides::BOTH] & (1u64 << target_sq - 8)) == 0))
                            {
                                // println!(
                                //     "MEGA PAWN RUSHUUUU: {} -> {}",
                                //     CONVERT_INDEX_COORDINATE[source_sq],
                                //     CONVERT_INDEX_COORDINATE[target_sq - 8]
                                // );
                                current_move.encode_move(source_sq, target_sq - 8, piece, 12, 0, 1);
                                move_list.add(current_move);
                            }
                        }
                    }
                    _attacks = game.attack_tables.pawn_attacks[game.side][source_sq]
                        & game.occupancies[Sides::BLACK];

                    while (_attacks != 0) {
                        target_sq = get_index_of_least_significant_bit(_attacks);

                        if (source_sq >= SquareLabels::A7 as usize
                            && source_sq <= SquareLabels::H7 as usize)
                        {
                            // println!(
                            //     "Promote Capture Pawn: {} -> {}\nQueen\nRook\nBishop\nKnight",
                            //     CONVERT_INDEX_COORDINATE[source_sq],
                            //     CONVERT_INDEX_COORDINATE[target_sq]
                            // );
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::QUEEN,
                                1,
                                0,
                            );
                            move_list.add(current_move);
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::KNIGHT,
                                1,
                                0,
                            );
                            move_list.add(current_move);
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::BISHOP,
                                1,
                                0,
                            );
                            move_list.add(current_move);
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::ROOK,
                                1,
                                0,
                            );
                            move_list.add(current_move);
                        } else {
                            // println!(
                            //     "Capture Pawn: {} -> {}",
                            //     CONVERT_INDEX_COORDINATE[source_sq],
                            //     CONVERT_INDEX_COORDINATE[target_sq]
                            // );
                            current_move.encode_move(source_sq, target_sq, piece, 12, 1, 0);
                            move_list.add(current_move);
                        }

                        if (_attacks & (1u64 << target_sq) != 0) {
                            _attacks ^= (1u64 << target_sq);
                        } else {
                            0;
                        }
                    }

                    if (_bitboard & (1u64 << source_sq) != 0) {
                        _bitboard ^= (1u64 << source_sq);
                    } else {
                        0;
                    }
                }
            }
        } else {
            if piece == Pieces::pawn {
                while (_bitboard != 0) {
                    source_sq = get_index_of_least_significant_bit(_bitboard);
                    target_sq = source_sq + 8;

                    if (!(target_sq > SquareLabels::H1 as usize)
                        && ((game.occupancies[Sides::BOTH] & (1u64 << target_sq)) == 0))
                    {
                        if (source_sq >= SquareLabels::A2 as usize
                            && source_sq <= SquareLabels::H2 as usize)
                        {
                            // println!(
                            //     "Promote Pawn: {} -> {}\nQueen\nRook\nBishop\nKnight",
                            //     CONVERT_INDEX_COORDINATE[source_sq],
                            //     CONVERT_INDEX_COORDINATE[target_sq]
                            // );
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::queen,
                                0,
                                0,
                            );
                            move_list.add(current_move);
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::knight,
                                0,
                                0,
                            );
                            move_list.add(current_move);
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::bishop,
                                0,
                                0,
                            );
                            move_list.add(current_move);
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::rook,
                                0,
                                0,
                            );
                            move_list.add(current_move);
                        } else {
                            // println!(
                            //     "Push Pawn: {} -> {}",
                            //     CONVERT_INDEX_COORDINATE[source_sq],
                            //     CONVERT_INDEX_COORDINATE[target_sq]
                            // );
                            current_move.encode_move(source_sq, target_sq, piece, 12, 0, 0);
                            move_list.add(current_move);

                            if ((source_sq >= SquareLabels::A7 as usize
                                && source_sq <= SquareLabels::H7 as usize)
                                && ((game.occupancies[Sides::BOTH] & (1u64 << target_sq + 8)) == 0))
                            {
                                // println!(
                                //     "MEGA PAWN RUSHUUUU: {} -> {}",
                                //     CONVERT_INDEX_COORDINATE[source_sq],
                                //     CONVERT_INDEX_COORDINATE[target_sq + 8]
                                // );
                                current_move.encode_move(source_sq, target_sq + 8, piece, 12, 0, 1);
                                move_list.add(current_move);
                            }
                        }
                    }
                    _attacks = game.attack_tables.pawn_attacks[game.side][source_sq]
                        & game.occupancies[Sides::WHITE];

                    while (_attacks != 0) {
                        target_sq = get_index_of_least_significant_bit(_attacks);

                        if (source_sq >= SquareLabels::A2 as usize
                            && source_sq <= SquareLabels::H2 as usize)
                        {
                            // println!(
                            //     "Promote Capture Pawn: {} -> {}\nQueen\nRook\nBishop\nKnight",
                            //     CONVERT_INDEX_COORDINATE[source_sq],
                            //     CONVERT_INDEX_COORDINATE[target_sq]
                            // );
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::queen,
                                1,
                                0,
                            );
                            move_list.add(current_move);
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::knight,
                                1,
                                0,
                            );
                            move_list.add(current_move);
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::rook,
                                1,
                                0,
                            );
                            move_list.add(current_move);
                            current_move.encode_move(
                                source_sq,
                                target_sq,
                                piece,
                                Pieces::bishop,
                                1,
                                0,
                            );
                            move_list.add(current_move);
                        } else {
                            // println!(
                            //     "Capture Pawn: {} -> {}",
                            //     CONVERT_INDEX_COORDINATE[source_sq],
                            //     CONVERT_INDEX_COORDINATE[target_sq]
                            // );
                            current_move.encode_move(source_sq, target_sq, piece, 12, 1, 0);
                            move_list.add(current_move);
                        }
                        if (_attacks & (1u64 << target_sq) != 0) {
                            _attacks ^= (1u64 << target_sq);
                        } else {
                            0;
                        }
                    }
                    if (_bitboard & (1u64 << source_sq) != 0) {
                        _bitboard ^= (1u64 << source_sq);
                    } else {
                        0;
                    }
                }
            }
        }
        if (if (game.side == Sides::WHITE) {
            piece == Pieces::KNIGHT
        } else {
            piece == Pieces::knight
        }) {
            while (_bitboard != 0) {
                source_sq = get_index_of_least_significant_bit(_bitboard);

                _attacks = game.attack_tables.knight_attacks[source_sq]
                    & (if (game.side == Sides::WHITE) {
                        !game.occupancies[Sides::WHITE]
                    } else {
                        !game.occupancies[Sides::BLACK]
                    });

                while (_attacks != 0) {
                    target_sq = get_index_of_least_significant_bit(_attacks);

                    if ((if (game.side == Sides::WHITE) {
                        game.occupancies[Sides::BLACK]
                    } else {
                        game.occupancies[Sides::WHITE]
                    } & (1u64 << target_sq))
                        == 0)
                    {
                        // println!(
                        //     "Piece Move: {} -> {}",
                        //     CONVERT_INDEX_COORDINATE[source_sq],
                        //     CONVERT_INDEX_COORDINATE[target_sq]
                        // );
                        current_move.encode_move(source_sq, target_sq, piece, 12, 0, 0);
                        move_list.add(current_move);
                    } else {
                        // println!(
                        //     "Piece Capture: {} -> {}",
                        //     CONVERT_INDEX_COORDINATE[source_sq],
                        //     CONVERT_INDEX_COORDINATE[target_sq]
                        // );
                        current_move.encode_move(source_sq, target_sq, piece, 12, 1, 0);
                        move_list.add(current_move);
                    }

                    if (_attacks & (1u64 << target_sq) != 0) {
                        _attacks ^= (1u64 << target_sq);
                    } else {
                        0;
                    }
                }

                if (_bitboard & (1u64 << source_sq) != 0) {
                    _bitboard ^= (1u64 << source_sq);
                } else {
                    0;
                }
            }
        }
        if (if (game.side == Sides::WHITE) {
            piece == Pieces::BISHOP
        } else {
            piece == Pieces::bishop
        }) {
            while (_bitboard != 0) {
                source_sq = get_index_of_least_significant_bit(_bitboard);

                _attacks = get_bishop_attacks(source_sq, game.occupancies[Sides::BOTH], game)
                    & (if (game.side == Sides::WHITE) {
                        !game.occupancies[Sides::WHITE]
                    } else {
                        !game.occupancies[Sides::BLACK]
                    });

                while (_attacks != 0) {
                    target_sq = get_index_of_least_significant_bit(_attacks);

                    if ((if (game.side == Sides::WHITE) {
                        game.occupancies[Sides::BLACK]
                    } else {
                        game.occupancies[Sides::WHITE]
                    } & (1u64 << target_sq))
                        == 0)
                    {
                        // println!(
                        //     "Piece Move: {} -> {}",
                        //     CONVERT_INDEX_COORDINATE[source_sq],
                        //     CONVERT_INDEX_COORDINATE[target_sq]
                        // );
                        current_move.encode_move(source_sq, target_sq, piece, 12, 0, 0);
                        move_list.add(current_move);
                    } else {
                        // println!(
                        //     "Piece Capture: {} -> {}",
                        //     CONVERT_INDEX_COORDINATE[source_sq],
                        //     CONVERT_INDEX_COORDINATE[target_sq]
                        // );
                        current_move.encode_move(source_sq, target_sq, piece, 12, 1, 0);
                        move_list.add(current_move);
                    }

                    if (_attacks & (1u64 << target_sq) != 0) {
                        _attacks ^= (1u64 << target_sq);
                    } else {
                        0;
                    }
                }

                if (_bitboard & (1u64 << source_sq) != 0) {
                    _bitboard ^= (1u64 << source_sq);
                } else {
                    0;
                }
            }
        }
        if (if (game.side == Sides::WHITE) {
            piece == Pieces::ROOK
        } else {
            piece == Pieces::rook
        }) {
            while (_bitboard != 0) {
                source_sq = get_index_of_least_significant_bit(_bitboard);

                _attacks = get_rook_attacks(source_sq, game.occupancies[Sides::BOTH], game)
                    & (if (game.side == Sides::WHITE) {
                        !game.occupancies[Sides::WHITE]
                    } else {
                        !game.occupancies[Sides::BLACK]
                    });

                while (_attacks != 0) {
                    target_sq = get_index_of_least_significant_bit(_attacks);

                    if ((if (game.side == Sides::WHITE) {
                        game.occupancies[Sides::BLACK]
                    } else {
                        game.occupancies[Sides::WHITE]
                    } & (1u64 << target_sq))
                        == 0)
                    {
                        // println!(
                        //     "Piece Move: {} -> {}",
                        //     CONVERT_INDEX_COORDINATE[source_sq],
                        //     CONVERT_INDEX_COORDINATE[target_sq]
                        // );
                        current_move.encode_move(source_sq, target_sq, piece, 12, 0, 0);
                        move_list.add(current_move);
                    } else {
                        // println!(
                        //     "Piece Capture: {} -> {}",
                        //     CONVERT_INDEX_COORDINATE[source_sq],
                        //     CONVERT_INDEX_COORDINATE[target_sq]
                        // );
                        current_move.encode_move(source_sq, target_sq, piece, 12, 1, 0);
                        move_list.add(current_move);
                    }

                    if (_attacks & (1u64 << target_sq) != 0) {
                        _attacks ^= (1u64 << target_sq);
                    } else {
                        0;
                    }
                }

                if (_bitboard & (1u64 << source_sq) != 0) {
                    _bitboard ^= (1u64 << source_sq);
                } else {
                    0;
                }
            }
        }
        if (if (game.side == Sides::WHITE) {
            piece == Pieces::QUEEN
        } else {
            piece == Pieces::queen
        }) {
            while (_bitboard != 0) {
                source_sq = get_index_of_least_significant_bit(_bitboard);

                _attacks = get_queen_attacks(source_sq, game.occupancies[Sides::BOTH], game)
                    & (if (game.side == Sides::WHITE) {
                        !game.occupancies[Sides::WHITE]
                    } else {
                        !game.occupancies[Sides::BLACK]
                    });

                while (_attacks != 0) {
                    target_sq = get_index_of_least_significant_bit(_attacks);

                    if ((if (game.side == Sides::WHITE) {
                        game.occupancies[Sides::BLACK]
                    } else {
                        game.occupancies[Sides::WHITE]
                    } & (1u64 << target_sq))
                        == 0)
                    {
                        // println!(
                        //     "Piece Move: {} -> {}",
                        //     CONVERT_INDEX_COORDINATE[source_sq],
                        //     CONVERT_INDEX_COORDINATE[target_sq]
                        // );
                        current_move.encode_move(source_sq, target_sq, piece, 12, 0, 0);
                        move_list.add(current_move);
                    } else {
                        // println!(
                        //     "Piece Capture: {} -> {}",
                        //     CONVERT_INDEX_COORDINATE[source_sq],
                        //     CONVERT_INDEX_COORDINATE[target_sq]
                        // );
                        current_move.encode_move(source_sq, target_sq, piece, 12, 1, 0);
                        move_list.add(current_move);
                    }

                    if (_attacks & (1u64 << target_sq) != 0) {
                        _attacks ^= (1u64 << target_sq);
                    } else {
                        0;
                    }
                }

                if (_bitboard & (1u64 << source_sq) != 0) {
                    _bitboard ^= (1u64 << source_sq);
                } else {
                    0;
                }
            }
        }
        if (if (game.side == Sides::WHITE) {
            piece == Pieces::KING
        } else {
            piece == Pieces::king
        }) {
            while (_bitboard != 0) {
                source_sq = get_index_of_least_significant_bit(_bitboard);

                _attacks = (game.attack_tables.king_attacks[source_sq]
                    & (if (game.side == Sides::WHITE) {
                        !game.occupancies[Sides::WHITE]
                    } else {
                        !game.occupancies[Sides::BLACK]
                    }));

                while (_attacks != 0) {
                    target_sq = get_index_of_least_significant_bit(_attacks);

                    if ((if (game.side == Sides::WHITE) {
                        game.occupancies[Sides::BLACK]
                    } else {
                        game.occupancies[Sides::WHITE]
                    } & (1u64 << target_sq))
                        == 0)
                    {
                        // println!(
                        //     "Piece Move: {} -> {}",
                        //     CONVERT_INDEX_COORDINATE[source_sq],
                        //     CONVERT_INDEX_COORDINATE[target_sq]
                        // );
                        current_move.encode_move(source_sq, target_sq, piece, 12, 0, 0);
                        move_list.add(current_move);
                    } else {
                        // println!(
                        //     "Piece Capture: {} -> {}",
                        //     CONVERT_INDEX_COORDINATE[source_sq],
                        //     CONVERT_INDEX_COORDINATE[target_sq]
                        // );
                        current_move.encode_move(source_sq, target_sq, piece, 12, 1, 0);
                        move_list.add(current_move);
                    }

                    if (_attacks & (1u64 << target_sq) != 0) {
                        _attacks ^= (1u64 << target_sq);
                    } else {
                        0;
                    }
                }

                if (_bitboard & (1u64 << source_sq) != 0) {
                    _bitboard ^= (1u64 << source_sq);
                } else {
                    0;
                }
            }
        }
    }
    move_list
}

fn internal_make_move(_move: LocalMove, mut game: &mut Game) -> usize {
    let move_flag = MoveTypes::AllMoves;
    if (move_flag == MoveTypes::AllMoves) {
        game.make_board_copy();
        let source_sq = _move.get_move_source();
        let target_sq = _move.get_move_target();
        let piece = _move.get_move_piece();
        let promoted = _move.get_move_promoted();
        let capture_flag = _move.get_move_capture_flag();
        let double_pawn_push = _move.get_move_double_push_flag();

        game.bitboards[piece] &= !(1u64 << (source_sq));
        game.bitboards[piece] |= 1u64 << target_sq;

        if (capture_flag != 0) {
            let start_index;
            let end_index;

            if (game.side == Sides::WHITE) {
                start_index = 6;
                end_index = 11;
            } else {
                start_index = 0;
                end_index = 5;
            }
            for i in start_index..end_index {
                if ((game.bitboards[i]) & (1u64 << (target_sq)) != 0) {
                    if (game.bitboards[i] & (1u64 << target_sq) != 0) {
                        game.bitboards[i] ^= (1u64 << target_sq);
                    } else {
                        0;
                    }
                    break;
                }
            }
        }
        if (promoted != 12) {
            let mut i;
            if (game.side == Sides::WHITE) {
                i = Pieces::PAWN
            } else {
                i = Pieces::pawn
            }
            if (game.bitboards[i] & (1u64 << target_sq) != 0) {
                game.bitboards[i] ^= (1u64 << target_sq);
            } else {
                0;
            }
            game.bitboards[promoted] |= 1u64 << target_sq;
        }

        game.occupancies = [0u64; 3];
        game.update_occupancy();

        game.side ^= 1;

        let tmp1 = get_index_of_least_significant_bit(game.bitboards[Pieces::king]);
        let tmp2 = get_index_of_least_significant_bit(game.bitboards[Pieces::KING]);
        if (is_square_under_attack(
            if (game.side == Sides::WHITE) {
                tmp1
            } else {
                tmp2
            },
            game.side,
            game,
        ) != 0)
        {
            game.restore_board_from_copy();
            game.state = GameState::Check;
            return 0;
        } else {
            return 1;
        }
    } else {
        if (_move.get_move_capture_flag() != 0) {
            internal_make_move(_move, game);
            return 0;
        } else {
            return 0;
        }
    }
}

// unsafe fn generate_moves_not_pawn(
// piece: usize,
// mut _bitboard: u64,
// mut move_list: MoveList,
//     mut _attacks: u64,
//     mut current_move: LocalMove,
// ) -> (u64, MoveList) {
//     if (if (SIDE == Sides::WHITE) {
//         piece == Pieces::KNIGHT
//     } else {
//         piece == Pieces::knight
//     }) {
//         while (_bitboard != 0) {
//             source_sq = get_index_of_least_significant_bit(_bitboard) - 1;

//             _attacks = KNIGHT_ATTACKS[source_sq]
//                 & (if (SIDE == Sides::WHITE) {
//                     !game.occupancies[Sides::WHITE]
//                 } else {
//                     !game.occupancies[Sides::BLACK]
//                 });

//             while (_attacks != 0) {
//                 target_sq = get_index_of_least_significant_bit(_attacks) - 1;

//                 if ((if (SIDE == Sides::WHITE) {
//                     game.occupancies[Sides::BLACK]
//                 } else {
//                     game.occupancies[Sides::WHITE]
//                 } & (1u64 << target_sq))
//                     == 0)
//                 {
//                     println!(
//                         "Piece Move: {} -> {}",
//                         CONVERT_INDEX_COORDINATE[source_sq], CONVERT_INDEX_COORDINATE[target_sq]
//                     );
//                     current_move.encode_move(source_sq, target_sq, piece, 12, 0, 0);
//                     move_list.add(current_move);
//                 } else {
//                     println!(
//                         "Piece Capture: {} -> {}",
//                         CONVERT_INDEX_COORDINATE[source_sq], CONVERT_INDEX_COORDINATE[target_sq]
//                     );
//                     current_move.encode_move(source_sq, target_sq, piece, 12, 1, 0);
//                     move_list.add(current_move);
//                 }

//                 if (_attacks & (1u64 << target_sq) != 0) {
//                     _attacks ^= (1u64 << target_sq);
//                 } else {
//                     0;
//                 }
//             }

//             if (_bitboard & (1u64 << source_sq) != 0) {
//                 _bitboard ^= (1u64 << source_sq);
//             } else {
//                 0;
//             }
//         }
//     }
// }

fn generate_bishop_masks() -> [u64; 64] {
    let mut masks: [u64; 64] = [0u64; 64];
    for square_index in 0..64 {
        masks[square_index] = mask_bishop_attack(square_index);
    }
    masks
}
fn generate_rook_masks() -> [u64; 64] {
    let mut masks: [u64; 64] = [0u64; 64];
    for square_index in 0..64 {
        masks[square_index] = mask_rook_attack(square_index);
    }
    masks
}

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
    println!("  A B C D E F G H");
    print!("{}\n", bb);
}
fn print_squares_under_attack(side: usize, game: &Game) {
    for rank in 0..8 {
        print!("{} ", 8 - rank);
        for file in 0..8 {
            let square_index = (rank << 3) + file;
            print!("{} ", is_square_under_attack(square_index, side, &game));
        }
        println!("");
    }
    println!("  A B C D E F G H");
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

/**
this is the most convoluted thing every but it works great, courtesy of Tord Romstad
correction. It work great for generation illegal magic numbers
don't know why, if you know why do enlighten tis humble mortal
*/
fn find_magic_number(
    square_index: usize,
    bit_count_in_mask: usize,
    piece: usize,
    game: &Game,
) -> u64 {
    let mut occupancies: [u64; 4096] = [0; 4096];
    let mut attacks: [u64; 4096] = [0; 4096];
    let mut used_attacks: [u64; 4096] = [0u64; 4096];

    let attack_mask: u64 = if (piece == Pieces::BISHOP) {
        game.attack_tables.bishop_masks[square_index]
    } else {
        game.attack_tables.rook_masks[square_index]
    };

    let occupancy_indicies = 1 << bit_count_in_mask;

    let mut index = 0;
    while (index < occupancy_indicies) {
        occupancies[index] = set_occupancy(index, bit_count_in_mask, attack_mask);

        attacks[index] = if (piece == Pieces::BISHOP) {
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
            let magic_index: i32 = ((occupancies[index] * magic_number)
                >> (64 - bit_count_in_mask))
                .try_into()
                .unwrap();

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
    return (count_bits(bb ^ (bb - 1)) - 1);
}
///Generates an occupancy map depending on the given index/seed, bit_count and attack_mask, courtesy of https://www.chessprogramming.org/Magic_Bitboards
fn set_occupancy(index: usize, bit_count_in_mask: usize, mut attack_mask: u64) -> u64 {
    let mut occupancy_map = 0u64;

    let mut count = 0;
    while count < bit_count_in_mask {
        let square_index = get_index_of_least_significant_bit(attack_mask);

        if (attack_mask & (1u64 << square_index) != 0) {
            attack_mask = attack_mask ^ (1u64 << square_index);
        } else {
            0;
        }
        if (index & (1 << count) != 0) {
            occupancy_map |= (1u64 << square_index);
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

fn generate_bishop_attack_tables(bishop_masks: [u64; 64]) -> [[u64; 64]; 512] {
    let mut attacks: [[u64; 64]; 512] = [[0u64; 64]; 512];
    for square_index in 0..64 {
        let attack_mask = bishop_masks[square_index];
        let bit_count_in_mask = count_bits(attack_mask);
        let occupancy_indicies = (1 << bit_count_in_mask);

        let mut index = 0;

        while (index < occupancy_indicies) {
            let occupancy = set_occupancy(index, bit_count_in_mask, attack_mask);
            let magic_index = (occupancy * BISHOP_MAGIC_NUMBERS[square_index])
                >> (64 - BISHOP_OCCUPANCY_BIT_COUNT[square_index]);
            let tmp: usize = magic_index.try_into().unwrap();

            attacks[tmp][square_index] =
                mask_bishop_attack_with_blocking_pieces(square_index, occupancy);
            index += 1;
        }
    }
    attacks
}

fn get_bishop_attacks(square: usize, mut occupancy: u64, game: &Game) -> u64 {
    occupancy &= game.attack_tables.bishop_masks[square];
    occupancy *= BISHOP_MAGIC_NUMBERS[square];
    occupancy >>= (64 - BISHOP_OCCUPANCY_BIT_COUNT[square]);

    let tmp: usize = occupancy.try_into().unwrap();
    return game.attack_tables.bishop_attacks[tmp][square];
}
fn generate_rook_attack_tables(rook_masks: [u64; 64]) -> [[u64; 64]; 4096] {
    let mut attacks: [[u64; 64]; 4096] = [[0u64; 64]; 4096];
    for square_index in 0..64 {
        let attack_mask = rook_masks[square_index];
        let bit_count_in_mask = count_bits(attack_mask);
        let occupancy_indicies = (1 << bit_count_in_mask);

        let mut index = 0;

        while (index < occupancy_indicies) {
            let occupancy = set_occupancy(index, bit_count_in_mask, attack_mask);
            let magic_index = (occupancy * ROOK_MAGIC_NUMBERS[square_index])
                >> (64 - ROOK_OCCUPANCY_BIT_COUNT[square_index]);
            let tmp: usize = magic_index.try_into().unwrap();

            attacks[tmp][square_index] =
                mask_rook_attack_with_blocking_pieces(square_index, occupancy);
            index += 1;
        }
    }
    attacks
}

fn get_rook_attacks(square: usize, mut occupancy: u64, game: &Game) -> u64 {
    occupancy &= game.attack_tables.rook_masks[square];
    occupancy *= ROOK_MAGIC_NUMBERS[square];
    occupancy >>= (64 - ROOK_OCCUPANCY_BIT_COUNT[square]);

    let tmp: usize = occupancy.try_into().unwrap();
    return game.attack_tables.rook_attacks[tmp][square];
}

fn get_queen_attacks(square: usize, mut occupancy: u64, game: &Game) -> u64 {
    return get_bishop_attacks(square, occupancy, game) | get_rook_attacks(square, occupancy, game);
}

fn is_square_under_attack(square: usize, side: usize, game: &Game) -> usize {
    if ((side == Sides::WHITE)
        && (game.attack_tables.pawn_attacks[Sides::BLACK][square] & game.bitboards[Pieces::PAWN]
            != 0))
    {
        return 1;
    }
    if ((side == Sides::BLACK)
        && (game.attack_tables.pawn_attacks[Sides::WHITE][square] & game.bitboards[Pieces::pawn]
            != 0))
    {
        return 1;
    }

    if ((game.attack_tables.knight_attacks[square]
        & if (side == Sides::WHITE) {
            game.bitboards[Pieces::KNIGHT]
        } else {
            game.bitboards[Pieces::knight]
        })
        != 0)
    {
        return 1;
    }

    if ((game.attack_tables.king_attacks[square]
        & if (side == Sides::WHITE) {
            game.bitboards[Pieces::KING]
        } else {
            game.bitboards[Pieces::king]
        })
        != 0)
    {
        return 1;
    }

    if ((get_bishop_attacks(square, game.occupancies[Sides::BOTH], game)
        & if (side == Sides::WHITE) {
            game.bitboards[Pieces::BISHOP]
        } else {
            game.bitboards[Pieces::bishop]
        })
        != 0)
    {
        return 1;
    }
    if ((get_rook_attacks(square, game.occupancies[Sides::BOTH], game)
        & if (side == Sides::WHITE) {
            game.bitboards[Pieces::ROOK]
        } else {
            game.bitboards[Pieces::rook]
        })
        != 0)
    {
        return 1;
    }
    if ((get_queen_attacks(square, game.occupancies[Sides::BOTH], game)
        & if (side == Sides::WHITE) {
            game.bitboards[Pieces::QUEEN]
        } else {
            game.bitboards[Pieces::queen]
        })
        != 0)
    {
        return 1;
    }

    0
}

struct Sides;
impl Sides {
    const WHITE: usize = 0;
    const BLACK: usize = 1;
    const BOTH: usize = 2;
}

struct Pieces;
impl Pieces {
    const PAWN: usize = 0;
    const BISHOP: usize = 1;
    const KNIGHT: usize = 2;
    const ROOK: usize = 3;
    const QUEEN: usize = 4;
    const KING: usize = 5;
    const pawn: usize = 6;
    const bishop: usize = 7;
    const knight: usize = 8;
    const rook: usize = 9;
    const queen: usize = 10;
    const king: usize = 11;
}

enum AsciiPieces {
    P,
    N,
    B,
    R,
    Q,
    K,
    p,
    n,
    b,
    r,
    q,
    k,
}
enum SquareLabels {
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

#[derive(PartialEq, PartialOrd)]
enum MoveTypes {
    AllMoves,
    OnlyCapture,
}

#[derive(Copy, Clone, Debug)]
pub struct Game {
    state: GameState,
    game_variables: GameVariables,
    bitboards: [u64; 12],
    occupancies: [u64; 3],
    copy_of_bitboards: [u64; 12],
    copy_of_occupancies: [u64; 3],
    side: usize,
    copy_side: usize,
    attack_tables: AttackTables,
    move_list: MoveList,
}
impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::InProgress,
            game_variables: GameVariables::init(),
            bitboards: [0u64; 12],
            side: 0,
            occupancies: [0u64; 3],
            copy_of_bitboards: [0u64; 12],
            copy_of_occupancies: [0u64; 3],
            copy_side: 0,
            attack_tables: AttackTables::init(),
            move_list: MoveList::init(),
        }
    }
    pub fn make_move(&mut self, move_string: &str) {
        let parsed_move = parse_move(move_string, self);
        if (parsed_move._move != 0) {
            internal_make_move(parsed_move, self);
        }
    }
    pub fn get_game_state(&mut self) -> GameState {
        return self.state;
    }
    pub fn get_possible_moves(&mut self) -> Vec<LocalMove> {
        let tmp = generate_moves(self.move_list, &self);
        let tmp2 = tmp.internal_Move(self);
        tmp2
    }
    pub fn init_bitboard(&mut self) {
        self.bitboards[Pieces::PAWN] |= (1u64 << SquareLabels::A2 as usize);
        self.bitboards[Pieces::PAWN] |= (1u64 << SquareLabels::B2 as usize);
        self.bitboards[Pieces::PAWN] |= (1u64 << SquareLabels::C2 as usize);
        self.bitboards[Pieces::PAWN] |= (1u64 << SquareLabels::D2 as usize);
        self.bitboards[Pieces::PAWN] |= (1u64 << SquareLabels::E2 as usize);
        self.bitboards[Pieces::PAWN] |= (1u64 << SquareLabels::F2 as usize);
        self.bitboards[Pieces::PAWN] |= (1u64 << SquareLabels::G2 as usize);
        self.bitboards[Pieces::PAWN] |= (1u64 << SquareLabels::H2 as usize);

        self.bitboards[Pieces::ROOK] |= (1u64 << SquareLabels::A1 as usize);
        self.bitboards[Pieces::ROOK] |= (1u64 << SquareLabels::H1 as usize);

        self.bitboards[Pieces::KNIGHT] |= (1u64 << SquareLabels::G1 as usize);
        self.bitboards[Pieces::KNIGHT] |= (1u64 << SquareLabels::B1 as usize);

        self.bitboards[Pieces::BISHOP] |= (1u64 << SquareLabels::C1 as usize);
        self.bitboards[Pieces::BISHOP] |= (1u64 << SquareLabels::F1 as usize);

        self.bitboards[Pieces::KING] |= (1u64 << SquareLabels::E1 as usize);
        self.bitboards[Pieces::QUEEN] |= (1u64 << SquareLabels::D1 as usize);

        self.bitboards[Pieces::pawn] |= (1u64 << SquareLabels::A7 as usize);
        self.bitboards[Pieces::pawn] |= (1u64 << SquareLabels::B7 as usize);
        self.bitboards[Pieces::pawn] |= (1u64 << SquareLabels::C7 as usize);
        self.bitboards[Pieces::pawn] |= (1u64 << SquareLabels::D7 as usize);
        self.bitboards[Pieces::pawn] |= (1u64 << SquareLabels::E7 as usize);
        self.bitboards[Pieces::pawn] |= (1u64 << SquareLabels::F7 as usize);
        self.bitboards[Pieces::pawn] |= (1u64 << SquareLabels::G7 as usize);
        self.bitboards[Pieces::pawn] |= (1u64 << SquareLabels::H7 as usize);

        self.bitboards[Pieces::rook] |= (1u64 << SquareLabels::A8 as usize);
        self.bitboards[Pieces::rook] |= (1u64 << SquareLabels::H8 as usize);

        self.bitboards[Pieces::knight] |= (1u64 << SquareLabels::G8 as usize);
        self.bitboards[Pieces::knight] |= (1u64 << SquareLabels::B8 as usize);

        self.bitboards[Pieces::bishop] |= (1u64 << SquareLabels::C8 as usize);
        self.bitboards[Pieces::bishop] |= (1u64 << SquareLabels::F8 as usize);

        self.bitboards[Pieces::king] |= (1u64 << SquareLabels::E8 as usize);
        self.bitboards[Pieces::queen] |= (1u64 << SquareLabels::D8 as usize);
    }
    pub fn update_occupancy(&mut self) {
        for x in 0..6 {
            self.occupancies[Sides::WHITE] |= self.bitboards[x];
            self.occupancies[Sides::BLACK] |= self.bitboards[x + 6];
        }
        self.occupancies[Sides::BOTH] |= self.occupancies[Sides::WHITE];
        self.occupancies[Sides::BOTH] |= self.occupancies[Sides::BLACK];
    }
    fn make_board_copy(&mut self) {
        self.copy_of_bitboards = self.bitboards;
        self.copy_of_occupancies = self.occupancies;
        self.copy_side = self.side;
    }
    fn restore_board_from_copy(&mut self) {
        self.bitboards = self.copy_of_bitboards;
        self.occupancies = self.copy_of_occupancies;
        self.side = self.copy_side;
    }
    fn print_board(&self) {
        println!("");
        println!("    A B C D E F G H");
        println!("    ----------------");
        for rank in 0..8 {
            print!("{}  |", 8 - rank);
            for file in 0..8 {
                let square_index = (rank << 3) + file;

                let mut piece = 12;

                for i in 0..12 {
                    if ((self.bitboards[i] & (1u64 << square_index)) != 0) {
                        piece = i;
                    }
                }
                print!(
                    "{} ",
                    if piece == 12 {
                        '.'
                    } else {
                        ASCII_PIECES[piece]
                    }
                );
            }
            print!("|  {}", 8 - rank);
            println!("");
        }
        println!("    ----------------");
        println!("    A B C D E F G H");
        println!("\nCurrent turn: {}", SIDES[self.side]);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}
#[derive(Copy, Clone, Debug)]
struct GameVariables {
    move_list: MoveList,
    nodes: u128,
}

impl GameVariables {
    fn init() -> GameVariables {
        GameVariables {
            move_list: MoveList::init(),
            nodes: 0,
        }
    }
}
#[derive(Copy, Clone, Debug)]
struct AttackTables {
    pawn_attacks: [[u64; 64]; 2],
    knight_attacks: [u64; 64],
    king_attacks: [u64; 64],
    bishop_masks: [u64; 64],
    rook_masks: [u64; 64],
    bishop_attacks: [[u64; 64]; 512],
    rook_attacks: [[u64; 64]; 4096],
}
impl AttackTables {
    fn init() -> AttackTables {
        AttackTables {
            pawn_attacks: generate_pawn_attack_tables(),
            knight_attacks: generate_knight_attack_tables(),
            king_attacks: generate_king_attack_tables(),
            bishop_masks: generate_bishop_masks(),
            rook_masks: generate_rook_masks(),
            bishop_attacks: generate_bishop_attack_tables(generate_bishop_masks()),
            rook_attacks: generate_rook_attack_tables(generate_rook_masks()),
        }
    }
}
#[derive(Copy, Clone, Debug)]
struct MoveList {
    moves: [LocalMove; 256],
    count: usize,
}
impl MoveList {
    fn init() -> MoveList {
        MoveList {
            moves: [LocalMove::init(); 256],
            count: 0,
        }
    }
    fn add(&mut self, _move: LocalMove) {
        self.moves[self.count] = _move;
        self.count += 1;
    }
    fn print(&self) {
        println!("{}", self.count);
        for i in 0..self.count {
            let _move = self.moves[i];
            _move.print();
            println!("");
        }
    }
    fn Move(&self, mut game: &mut Game) {
        for i in 0..self.count {
            let _move = self.moves[i];
            game.make_board_copy();
            if (internal_make_move(_move, &mut game) == 0) {
                continue;
            }
            // make_move(_move, MoveTypes::AllMoves as usize);
            game.print_board();
            // print_bitboard(game.bitboards[Pieces::pawn]);
            // print_bitboard(game.bitboards[Pieces::knight]);
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).expect("Error");
            game.restore_board_from_copy();
        }
    }
    fn internal_Move(&self, mut game: &mut Game) -> Vec<LocalMove> {
        let mut v: Vec<LocalMove> = Vec::new();
        for i in 0..self.count {
            let _move = self.moves[i];
            game.make_board_copy();
            if (internal_make_move(_move, &mut game) == 0) {
                continue;
            }
            v.push(_move);
            game.restore_board_from_copy();
        }
        v
    }
    fn Perft(&self, depth: usize, mut game: &mut Game) {
        if (depth == 0) {
            game.game_variables.nodes += 1;
            return;
        }

        let mut move_list = MoveList::init();
        move_list = generate_moves(move_list, game);
        move_list.print();
        // move_list.Move(&mut game);
        for i in 0..move_list.count {
            game.make_board_copy();

            if (internal_make_move(move_list.moves[i], game) == 0) {
                continue;
            }

            self.Perft(depth - 1, game);
            game.restore_board_from_copy();
        }
    }
    fn PerftTest(&self, depth: usize, mut game: &mut Game) {
        let mut move_list = MoveList::init();
        move_list = generate_moves(move_list, game);
        move_list.print();

        // move_list.Move(&mut game);
        for i in 0..move_list.count {
            game.make_board_copy();
            if (internal_make_move(move_list.moves[i], game) == 0) {
                continue;
            }
            let some_nodes: u128 = game.game_variables.nodes;
            self.Perft(depth - 1, game);
            let old_nodes: u128 = game.game_variables.nodes - some_nodes;
            game.restore_board_from_copy();

            move_list.moves[i].print();
            print!(" |{}\n", old_nodes);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LocalMove {
    _move: usize,
}
impl LocalMove {
    fn init() -> LocalMove {
        LocalMove { _move: 0 }
    }

    ///Thank god, https://www.youtube.com/watch?v=KQcArvyrbIo&list=PLZ1QII7yudbc-Ky058TEaOstZHVbT-2hg&index=24
    fn encode_move(
        &mut self,
        source: usize,
        target: usize,
        piece: usize,
        promoted: usize,
        capture: usize,
        double: usize,
    ) -> usize {
        let tmp = source
            | (target << 6)
            | (piece << 12)
            | (promoted << 16)
            | (capture << 20)
            | (double << 21);
        self._move = tmp;
        tmp
    }

    pub fn get_move_source(&self) -> usize {
        return (self._move & 0x3f);
    }

    pub fn get_move_target(&self) -> usize {
        return (self._move & 0xfc0) >> 6;
    }

    pub fn get_move_piece(&self) -> usize {
        return (self._move & 0xf000) >> 12;
    }

    pub fn get_move_promoted(&self) -> usize {
        return (self._move & 0xf0000) >> 16;
    }
    pub fn get_move_capture_flag(&self) -> usize {
        return if (self._move & 0xf100000) != 0 { 1 } else { 0 };
    }

    pub fn get_move_double_push_flag(&self) -> usize {
        return if (self._move & 0xf200000) != 0 { 1 } else { 0 };
    }

    pub fn print(&self) {
        print!(
            "{} {} {} {} {} {}",
            CONVERT_INDEX_COORDINATE[self.get_move_source()],
            CONVERT_INDEX_COORDINATE[self.get_move_target()],
            ASCII_PIECES[self.get_move_piece()],
            ASCII_PIECES[self.get_move_promoted()],
            self.get_move_capture_flag(),
            self.get_move_double_push_flag()
        )
    }
}
