#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    Playing,
    XWon,
    OWon,
    Tie,
}

#[derive(Copy, Clone, PartialEq)]
pub enum BoardSpace {
    Occupied(Player),
    Empty,
}

#[derive(Clone)]
pub struct Game {
    board: [[BoardSpace; 3]; 3],
    turn: Player,
    state: GameState,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [[BoardSpace::Empty; 3]; 3],
            turn: Player::X,
            state: GameState::Playing,
        }
    }

    pub fn play(&mut self, x: usize, y: usize) {
        if let BoardSpace::Empty = self.board[x][y] {
            self.board[x][y] = BoardSpace::Occupied(self.turn);
            self.turn = match self.turn {
                Player::X => Player::O,
                Player::O => Player::X,
            }
        }
        self.state = self.check_win();
    }

    fn empty_spaces(&self) -> EmptySpacesIter {
        EmptySpacesIter {
            game: self,
            x: 0,
            y: 0,
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn get_board(&self) -> [[BoardSpace; 3]; 3] {
        self.board
    }

    fn check_win(&self) -> GameState {
        let mut x_won = false;
        let mut o_won = false;
        let mut tie = true;
        for row in self.board {
            if row.iter().all(|&x| x == BoardSpace::Occupied(Player::X)) {
                x_won = true;
            }
            if row.iter().all(|&x| x == BoardSpace::Occupied(Player::O)) {
                o_won = true;
            }
            if row.iter().any(|&x| x == BoardSpace::Empty) {
                tie = false;
            }
        }

        for i in 0..self.board[0].len() {
            if self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i]
                && self.board[0][i] != BoardSpace::Empty
            {
                match self.board[0][i] {
                    BoardSpace::Occupied(Player::X) => x_won = true,
                    BoardSpace::Occupied(Player::O) => o_won = true,
                    _ => (),
                }
            }
        }

        // hardcode, sorry about that
        if self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
            && self.board[0][0] != BoardSpace::Empty
        {
            match self.board[0][0] {
                BoardSpace::Occupied(Player::X) => x_won = true,
                BoardSpace::Occupied(Player::O) => o_won = true,
                _ => (),
            }
        }

        if self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
            && self.board[0][2] != BoardSpace::Empty
        {
            match self.board[0][2] {
                BoardSpace::Occupied(Player::X) => x_won = true,
                BoardSpace::Occupied(Player::O) => o_won = true,
                _ => (),
            }
        }

        if x_won {
            GameState::XWon
        } else if o_won {
            GameState::OWon
        } else if tie {
            GameState::Tie
        } else {
            GameState::Playing
        }
    }
}

struct EmptySpacesIter<'a> {
    game: &'a Game,
    x: usize,
    y: usize,
}

impl<'a> Iterator for EmptySpacesIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.x < self.game.board.len() {
            while self.y < self.game.board[self.x].len() {
                let space = &self.game.board[self.x][self.y];
                self.y += 1;
                if *space == BoardSpace::Empty {
                    return Some((self.x, self.y - 1));
                }
            }
            self.x += 1;
            self.y = 0;
        }
        None
    }
}

pub fn ai_move(game: &Game) -> Option<(usize, usize)> {
    if game.state != GameState::Playing {
        None
    } else {
        let ((x, y), _) = minimax(game, -10, 10);
        Some((x, y))
    }
}

fn minimax(game: &Game, mut alpha: i32, mut beta: i32) -> ((usize, usize), i32) {
    if game.state != GameState::Playing {
        return ((0, 0), evaluate_position(game));
    }

    let mut best_move = (0, 0);
    let mut best_score;
    match game.turn {
        Player::X => {
            best_score = -10;
            for (x, y) in game.empty_spaces() {
                let mut clonned_game = game.clone();
                clonned_game.play(x, y);
                let (_, score) = minimax(&clonned_game, alpha, beta);
                if score > best_score {
                    best_score = score;
                    best_move = (x, y);
                }
                alpha = alpha.max(score);
                if beta <= alpha {
                    break;
                }
            }
        }
        Player::O => {
            best_score = 10;
            for (x, y) in game.empty_spaces() {
                let mut clonned_game = game.clone();
                clonned_game.play(x, y);
                let (_, score) = minimax(&clonned_game, alpha, beta);
                if score < best_score {
                    best_score = score;
                    best_move = (x, y);
                }
                beta = beta.min(score);
                if beta <= alpha {
                    break;
                }
            }
        }
    }

    (best_move, best_score)
}

pub fn evaluate_position(game: &Game) -> i32 {
    match game.state {
        GameState::XWon => 1,
        GameState::OWon => -1,
        GameState::Tie => 0,
        GameState::Playing => panic!("shouldn't be called for board that still in game"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BoardSpace::*;
    use Player::*;

    #[test]
    fn test_minimax_last_step() {
        // tie
        let board = [
            [Occupied(X), Occupied(O), Occupied(O)],
            [Occupied(O), Occupied(X), Occupied(X)],
            [Occupied(X), Empty, Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game, -10, 10);
        assert_eq!(score, 0);

        // x win
        let board = [
            [Occupied(X), Occupied(O), Occupied(O)],
            [Empty, Occupied(X), Occupied(X)],
            [Occupied(X), Occupied(O), Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game, -10, 10);
        assert_eq!(score, 1);
    }

    #[test]
    fn test_minimax_depth2() {
        // x win
        let board = [
            [Occupied(X), Occupied(O), Occupied(O)],
            [Empty, Occupied(X), Occupied(X)],
            [Occupied(X), Occupied(O), Empty],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: O,
        };
        let (_, score) = minimax(&game, -10, 10);
        assert_eq!(score, 1);

        // tie
        let board = [
            [Occupied(X), Occupied(O), Occupied(O)],
            [Empty, Occupied(X), Occupied(X)],
            [Occupied(X), Empty, Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: O,
        };
        let (_, score) = minimax(&game, -10, 10);
        assert_eq!(score, 0);

        // o win
        let board = [
            [Empty, Occupied(O), Occupied(O)],
            [Empty, Occupied(X), Occupied(X)],
            [Occupied(X), Occupied(X), Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: O,
        };
        let (_, score) = minimax(&game, -10, 10);
        assert_eq!(score, -1);
    }

    #[test]
    fn test_minimax_depth3() {
        // x win
        let board = [
            [Empty, Occupied(O), Occupied(O)],
            [Empty, Occupied(X), Occupied(X)],
            [Occupied(X), Occupied(O), Empty],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game, -10, 10);
        assert_eq!(score, 1);

        // tie
        let board = [
            [Empty, Occupied(O), Occupied(O)],
            [Empty, Empty, Occupied(X)],
            [Occupied(X), Occupied(X), Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game, -10, 10);
        assert_eq!(score, 0);

        // o win
        let board = [
            [Occupied(O), Empty, Occupied(O)],
            [Empty, Empty, Occupied(X)],
            [Occupied(O), Occupied(X), Occupied(X)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game, -10, 10);
        assert_eq!(score, -1);
    }

    #[test]
    // #[ignore]
    fn test_minimax_the_game_is_a_tie() {
        let board = [
            [Empty, Empty, Empty],
            [Empty, Empty, Empty],
            [Empty, Empty, Empty],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game, -10, 10);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_iterator() {
        let board = [
            [Occupied(X), Occupied(O), Empty],
            [Occupied(O), Occupied(X), Occupied(X)],
            [Empty, Empty, Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let empty = game.empty_spaces().count();
        assert_eq!(empty, 3);
    }
}
