fn main() {
    println!("Hello, world!");
    let test = Game::init();
    let attack = AttackTables::init();
}
#[derive(Copy, Clone, Debug)]
struct Game {
    state: GameState,
    game_variables: GameVariables,
    bitboards: [u64; 12],
}
impl Game {
    fn init() -> Game {
        Game {
            state: GameState::InProgress,
            game_variables: GameVariables::init(),
            bitboards: [0u64; 12],
        }
    }
}
#[derive(Copy, Clone, Debug)]
enum GameState {
    InProgress,
    Check,
    GameOver,
}
#[derive(Copy, Clone, Debug)]
struct GameVariables {
    move_list: MoveList,
    side: u8,
    occupancies: [u64; 3],
}

impl GameVariables {
    fn init() -> GameVariables {
        GameVariables {
            move_list: MoveList::init(),
            side: 0,
            occupancies: [10000u64; 3],
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
            pawn_attacks: [[0u64; 64]; 2],
            knight_attacks: [0u64; 64],
            king_attacks: [0u64; 64],
            bishop_masks: [0u64; 64],
            rook_masks: [0u64; 64],
            bishop_attacks: [[0u64; 64]; 512],
            rook_attacks: [[0u64; 64]; 4096],
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
        for i in 0..self.count {
            let _move = self.moves[i];
            _move.print();
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct LocalMove {
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

    fn get_move_source(&self) -> usize {
        return (self._move & 0x3f);
    }

    fn get_move_target(&self) -> usize {
        return (self._move & 0xfc0) >> 6;
    }

    fn get_move_piece(&self) -> usize {
        return (self._move & 0xf000) >> 12;
    }

    fn get_move_promoted(&self) -> usize {
        return (self._move & 0xf0000) >> 16;
    }
    fn get_move_capture_flag(&self) -> usize {
        return if (self._move & 0xf100000) != 0 { 1 } else { 0 };
    }

    fn get_move_double_push_flag(&self) -> usize {
        return if (self._move & 0xf200000) != 0 { 1 } else { 0 };
    }

    fn print(&self) {
        println!(
            "{} {} {} {} {} {}",
            self.get_move_source(),
            self.get_move_target(),
            self.get_move_piece(),
            self.get_move_promoted(),
            self.get_move_capture_flag(),
            self.get_move_double_push_flag()
        )
    }
}
