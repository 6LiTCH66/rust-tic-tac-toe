use std::io::stdin;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Player{
    X,
    O,
    Position(i32),
    None,
}
const COLUMN: usize = 3;
const ROW: usize = 3;


#[derive(Debug)]
struct TicTacToe{
    board: [[Player; 3]; 3],
    player: Player,
    cell: i32,
    taken_cells: Vec<i32>
}

impl TicTacToe{

    fn new() -> Self{
        Self{
            board: [
                [Player::Position(1), Player::Position(2), Player::Position(3)],
                [Player::Position(4), Player::Position(5), Player::Position(6)],
                [Player::Position(7), Player::Position(8), Player::Position(9)]],
            player: Player::None,
            cell: 0,
            taken_cells: Vec::new(),
        }
    }

    fn print_game_board(&self){
        println!("|---||---||---|");
        for i in 0..ROW {
            for j in 0..COLUMN {
                match self.board[i][j] {
                    Player::Position(num) => {
                        print!("| {} |", num)
                    },
                    _ => {
                        print!("| {:?} |", self.board[i][j])
                    }
                }

            }
            println!();
            println!("|---||---||---|")
        }
    }

    fn set_player(&mut self, player: i32){
        match player {
            1 => {
                self.player = Player::X
            },
            2 => {
                self.player = Player::O
            },
            _ => {
                println!("No such number!\n Try again!");
            }
        }
    }
    fn change_player(&mut self){
        if self.player == Player::X {
            self.player = Player::O
        }else{
            self.player = Player::X
        }
    }

    fn update_game_board(&mut self){
        match self.cell {
            1 => {
                self.board[0][0] = self.player;
            },
            2 => {
                self.board[0][1] = self.player;
            },
            3 => {
                self.board[0][2] = self.player;
            },
            4 => {
                self.board[1][0] = self.player;
            },
            5 => {
                self.board[1][1] = self.player;
            },
            6 => {
                self.board[1][2] = self.player;
            },
            7 => {
                self.board[2][0] = self.player
            },
            8 => {
                self.board[2][1] = self.player
            },
            9 => {
                self.board[2][2] = self.player
            },
            _ => {}
        }
        self.change_player()

    }

    fn is_win(&self) -> bool{

        //Horizontal Win
        if self.board[0][0] == Player::X && self.board[0][1] == Player::X && self.board[0][2] == Player::X {
            println!("X won the game");
            return true;
        }

        if self.board[1][0] == Player::X && self.board[1][1] == Player::X && self.board[1][2] == Player::X {
            println!("X won the game");
            return true;
        }

        if self.board[2][0] == Player::X && self.board[2][1] == Player::X && self.board[2][2] == Player::X  {
            println!("X won the game");
            return true;
        }

        //Vertical Win

        if self.board[0][0] == Player::X && self.board[1][0] == Player::X && self.board[2][0] == Player::X {
            println!("X won the game");
            return true;
        }
        if self.board[0][1] == Player::X && self.board[1][1] == Player::X && self.board[2][1] == Player::X{
            println!("X won the game");
            return true;
        }

        if self.board[0][2] == Player::X && self.board[1][2] == Player::X && self.board[2][2] == Player::X  {
            println!("X won the game");
            return true;
        }

        //Diagonal Wins

        if self.board[0][0] == Player::X && self.board[1][1] == Player::X && self.board[2][2] == Player::X  {
            println!("X won the game");
            return true;
        }

        if self.board[0][2] == Player::X && self.board[1][1] == Player::X && self.board[2][0] == Player::X  {
            println!("X won the game");
            return true;
        }


        //Horizontal Win
        if self.board[0][0] == Player::O && self.board[0][1] == Player::O && self.board[0][2] == Player::O {
            println!("O won the game");
            return true;
        }

        if self.board[1][0] == Player::O && self.board[1][1] == Player::O && self.board[1][2] == Player::O {
            println!("O won the game");
            return true;
        }

        if self.board[2][0] == Player::O && self.board[2][1] == Player::O && self.board[2][2] == Player::O  {
            println!("O won the game");
            return true;
        }

        //Vertical Win

        if self.board[0][0] == Player::O && self.board[1][0] == Player::O && self.board[2][0] == Player::O {
            println!("O won the game");
            return true;
        }
        if self.board[0][1] == Player::O && self.board[1][1] == Player::O && self.board[2][1] == Player::O{
            println!("O won the game");
            return true;
        }

        if self.board[0][2] == Player::O && self.board[1][2] == Player::O && self.board[2][2] == Player::O {
            println!("O won the game");
            return true;
        }

        //Diagonal Wins

        if self.board[0][0] == Player::O && self.board[1][1] == Player::O && self.board[2][2] == Player::O  {
            println!("O won the game");
            return true;
        }

        if self.board[0][2] == Player::O && self.board[1][1] == Player::O && self.board[2][0] == Player::O  {
            println!("O won the game");
            return true;
        }


        return false
    }

    fn fill_game_board(&mut self){
        self.print_game_board();
        let mut cell = String::new();
        loop {
            println!("Please enter a cell number for {:?}:", self.player);
            stdin().read_line(&mut cell).expect("Cannot read line!");
            match cell.trim().parse::<i32>() {
                Ok(cell_num) => {
                    match cell_num {
                        1..=9 => {
                            if !self.taken_cells.contains(&cell_num) {
                                self.taken_cells.push(cell_num);
                                self.cell = cell_num;
                                break;
                            }else {
                                cell.clear();
                            }
                        }
                        _ => {
                            cell.clear();
                        }
                    }

                },
                Err(err) => {
                    println!("Error: {}", err);
                    cell.clear();
                }
            }
        }


        self.update_game_board();

    }
}

fn main() {
    let mut player = String::new();
    let mut tic_tac_toe = TicTacToe::new();
    println!("1. X\n2. O");
    println!("Please choose your char:");
    stdin().read_line(&mut player).expect("Cannot read line! \nTry again!");
    match player.trim().parse::<i32>() {
        Ok(num) => {
            tic_tac_toe.set_player(num);
            while !tic_tac_toe.is_win() {

                tic_tac_toe.fill_game_board();
            }

        }
        Err(err) => {
            println!("Error: {}", err)
        }
    }

}
