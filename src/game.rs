use crate::board::{TileContent, Board, EntryPoint, Orientation};
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Player {
    tag: char,
    pos: (usize, usize),
    objectives: Vec<TileContent>
}

impl Player {
    fn to_colored_str(&self) -> String {
        match self.tag {
            'r' => "\x1B[1;31mRed\x1B[0m".to_string(),
            'b' => "\x1B[1;34mBlue\x1B[0m".to_string(),
            'y' => "\x1B[1;33mYellow\x1B[0m".to_string(),
            'g' => "\x1B[1;32mGreen\x1B[0m".to_string(),
            _ => panic!()
        }
    }
}

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    current_player: usize,
    board: Board,
    end: bool
}

impl Game {
    pub fn new() -> Game {
        let mut g = Game {
            players: Game::get_players(),
            current_player: 0,
            board: Board::new(),
            end: false
        };

        g.deal_objectives();
        g
    }

    pub fn play_turn(&mut self) {
        println!("{}", self.draw_to_string());
        self.get_push();

        println!("{}", self.draw_to_string());
        self.get_move();

        // Check the objectives.
        let p = &mut self.players[self.current_player];
        let colored_str = p.to_colored_str();
        if p.objectives.last() == Some(&self.board.content_at(p.pos.0, p.pos.1)) {
            let done = p.objectives.pop().unwrap();
            let left = p.objectives.len();
            if left > 2 {
                println!("You reached the \x1B[1m{:?}\x1B[0m! You have now {} objectives to go!", done, p.objectives.len()-1);
                self.board.remove_content(done);
            }
            else if left == 2 {
                println!("You reached the \x1B[1m{:?}\x1B[0m! You have now only {} objective to go!", done, p.objectives.len()-1);
                self.board.remove_content(done);
            } else if left == 1 {
                println!("You reached the \x1B[1m{:?}\x1B[0m! You just have to go back to the start tile (\"{}\") to win!", done, p.tag.to_ascii_uppercase());
                self.board.remove_content(done);
            } else {
                // Victory!
                println!("{}", self.draw_to_string());
                println!("Player {} wins the game! Congratulations!", colored_str);
                self.end = true;
            }
        }

        // Go to the next player.
        self.current_player = (self.current_player + 1) % self.players.len();
    }

    fn get_players() -> Vec<Player> {
        loop {
            println!("Who is playing?");
            println!("(for example: enter 'r,b' if the red and blue players are playing)");
            let mut players_str = String::new();

            std::io::stdin().read_line(&mut players_str)
                .expect("Failed to read line");

            // Read the list of players given in input.
            let (mut r, mut b, mut y, mut g, mut err) = (0, 0, 0, 0, 0);
            let players: Vec<_> = players_str.trim().split(',').filter_map(|x| {
                match x.trim().to_ascii_lowercase().as_str() {
                    "r" => { r += 1; Some(Player{ tag: 'r', pos: (0, 0), objectives: vec![] }) },
                    "b" => { b += 1; Some(Player{ tag: 'b', pos: (0, 6), objectives: vec![] }) },
                    "y" => { y += 1; Some(Player{ tag: 'y', pos: (6, 0), objectives: vec![] }) },
                    "g" => { g += 1; Some(Player{ tag: 'g', pos: (6, 6), objectives: vec![] }) },
                    _ => {
                        err += 1;
                        println!("{} is not a valid player!", x);
                        None
                    }
                }
            }).collect();

            if err > 0 {
                continue;
            }

            if players.len() < 2 || players.len() > 4 {
                println!("The number of players should be between 2 and 4!");
                continue;
            }

            if r > 1 || b > 1 || y > 1 || g > 1 {
                println!("The same player cannot appear multiple times!");
                continue;
            }

            println!();
            return players;
        };
    }

    fn deal_objectives(&mut self) {
        // Shuffle the 24 objectives.
        let mut obj = ["ba","be","bo","ca","cr","dr","fa","ge","gn","gh","go","he",
                       "ke","ma","mo","ow","ra","ri","sa","sp","tr","tl","sk","sw"];
        obj.shuffle(&mut thread_rng());

        let mut stack: Vec<_> = obj.iter().map(|o| TileContent::try_from_str(o).unwrap()).collect();

        // Add the final objective: going back to the home tile.
        for p in &mut self.players {
            p.objectives.push(TileContent::try_from_str(&p.tag.to_string()).unwrap());
        }

        // Distribute the 24 objectives.
        let mut idx = 0;
        while let Some(obj) = stack.pop() {
            self.players[idx].objectives.push(obj);
            idx = (idx + 1) % self.players.len();
        }
    }

    fn get_push(&mut self) {
        let p = &self.players[self.current_player];
        let obj = p.objectives.last().unwrap();
        let obj_str = format!("{:?}", obj);
        let [obj_c1, obj_c2] = obj.to_chars();

        loop {
            println!("Player {}, it's your turn. Shift the labyrinth!", p.to_colored_str());
            println!("Your objective is: \x1B[1m{} (\"{}{}\")\x1B[0m.", obj_str, obj_c1, obj_c2);
            println!();
            println!("Where do you put the tile?");
            println!("(for example: enter '1e' to push the tile from the '1' position, oriented towards east)");
            let mut s = String::new();

            std::io::stdin().read_line(&mut s)
                .expect("Failed to read line");

            // Check if the push given in input is valid.
            let entry: String = s.chars().filter(|c| c.is_digit(10)).collect();
            let entry = match entry.as_str() {
                "1" => EntryPoint::NorthLeft,
                "2" => EntryPoint::NorthCenter,
                "3" => EntryPoint::NorthRight,
                "4" => EntryPoint::EastUp,
                "5" => EntryPoint::EastCenter,
                "6" => EntryPoint::EastDown,
                "7" => EntryPoint::SouthRight,
                "8" => EntryPoint::SouthCenter,
                "9" => EntryPoint::SouthLeft,
                "10" => EntryPoint::WestDown,
                "11" => EntryPoint::WestCenter,
                "12" => EntryPoint::WestUp,
                _ => {
                    println!("Invalid move!");
                    continue;
                }
            };

            if Some(entry) == self.board.extra_tile_position() {
                println!("You cannot undo the previous move!");
                continue;
            }

            let orientation: String = s.trim().chars().filter(|c| !c.is_digit(10)).collect();
            let orientation = match orientation.as_str() {
                "n" => Orientation::North,
                "e" => Orientation::East,
                "s" => Orientation::South,
                "w" => Orientation::West,
                _ => {
                    println!("Invalid move!");
                    continue;
                }
            };

            // The push is valid, apply it.
            self.board.push_tile(entry, orientation);

            // Update player positions.
            for p in self.players.iter_mut() {
                match (p.pos.0, entry) {
                    (1, EntryPoint::EastUp) => { p.pos.1 = (p.pos.1 + 6) % 7 },
                    (1, EntryPoint::WestUp) => { p.pos.1 = (p.pos.1 + 1) % 7 },
                    (3, EntryPoint::EastCenter) => { p.pos.1 = (p.pos.1 + 6) % 7 },
                    (3, EntryPoint::WestCenter) => { p.pos.1 = (p.pos.1 + 1) % 7 },
                    (5, EntryPoint::EastDown) => { p.pos.1 = (p.pos.1 + 6) % 7 },
                    (5, EntryPoint::WestDown) => { p.pos.1 = (p.pos.1 + 1) % 7 },
                    _ => {}
                }
                match (p.pos.1, entry) {
                    (1, EntryPoint::NorthLeft) => { p.pos.0 = (p.pos.0 + 1) % 7 },
                    (1, EntryPoint::SouthLeft) => { p.pos.0 = (p.pos.0 + 6) % 7 },
                    (3, EntryPoint::NorthCenter) => { p.pos.0 = (p.pos.0 + 1) % 7 },
                    (3, EntryPoint::SouthCenter) => { p.pos.0 = (p.pos.0 + 6) % 7 },
                    (5, EntryPoint::NorthRight) => { p.pos.0 = (p.pos.0 + 1) % 7 },
                    (5, EntryPoint::SouthRight) => { p.pos.0 = (p.pos.0 + 6) % 7 },
                    _ => {}
                }
            }

            return;
        }
    }

    fn get_move(&mut self) {
        let (row, col) = self.players[self.current_player].pos;
        let reachable = self.board.reachable_from(row, col);

        loop {
            println!("Player {}, you are in row {}, column {}. Where do you move?", self.players[self.current_player].to_colored_str(), row+1, col+1);
            println!("(for example: enter '35' to go to row 3, column 5. Or enter 'Sk' to go the Skull)");
            let mut s = String::new();

            std::io::stdin().read_line(&mut s)
                .expect("Failed to read line");

            // Check if the move given in input is valid.
            let mut next_pos = if let Some(obj) = TileContent::try_from_str(&s) {
                self.board.content_position(obj)
            } else {
                None
            };

            if next_pos == None {
                let d: Vec<_> = s.chars().filter(|c| c.is_digit(10)).collect();
                if d.len() < 2 {
                    println!("Invalid coordinates.");
                    continue;
                }
                next_pos = Some(((d[0].to_digit(10).unwrap()-1) as usize, (d[1].to_digit(10).unwrap()-1) as usize));
            }

            let next_pos = next_pos.unwrap();
            if next_pos.0 >= 7 || next_pos.1 >= 7 {
                println!("Invalid coordinates.");
                continue;
            }

            if !reachable.contains(&next_pos) {
                println!("You cannot reach row {}, column {}.", next_pos.0+1, next_pos.1+1);
                continue;
            }

            // The move is valid, update the player position.
            self.players[self.current_player].pos = next_pos;
            return;
        }

    }

    pub fn is_over(&self) -> bool {
        self.end
    }

    pub fn draw_to_string(&self) -> String {
        let mut b = self.board.to_ascii_29x56();

        // Draw players.
        for p in &self.players {
            b[3*(1+p.pos.0)+1][6*(1+p.pos.1)+5] = p.tag;
        }

        // Make sure the current player is not covered by others.
        let p = &self.players[self.current_player];
        b[3*(1+p.pos.0)+1][6*(1+p.pos.1)+5] = p.tag;

        let mut s = String::new();
        for r in 0..29 {
            for c in 0..56 {
                s.push(b[r][c]);
            }
            s.push('\n');
        }

        // Colorize players.
        s = s.replace("r\u{2589}", "\x1B[7;1;31mr\x1B[0m\u{2589}");
        s = s.replace("b\u{2589}", "\x1B[7;1;34mb\x1B[0m\u{2589}");
        s = s.replace("y\u{2589}", "\x1B[7;1;33my\x1B[0m\u{2589}");
        s = s.replace("g\u{2589}", "\x1B[7;1;32mg\x1B[0m\u{2589}");

        s
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
