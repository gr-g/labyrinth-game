use std::collections::HashSet;
use std::collections::VecDeque;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug)]
pub enum TileKind {
    Straight,
    Corner,
    Junction
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileContent {
    None,
    YellowMarker,
    RedMarker,
    GreenMarker,
    BlueMarker,
    Bat,
    Beetle,
    Book,
    Candelabrum,
    Crown,
    Dragon,
    Fairy,
    Gem,
    Gold,
    Genie,
    Ghost,
    Helm,
    Keys,
    Map,
    Moth,
    Owl,
    Rat,
    Ring,
    Salamander,
    Spider,
    Treasure,
    Troll,
    Skull,
    Sword
}

impl TileContent {
    pub fn to_chars(self) -> [char; 2] {
        match self {
            TileContent::None         => [' ', ' '],
            TileContent::YellowMarker => ['Y', ' '],
            TileContent::RedMarker    => ['R', ' '],
            TileContent::GreenMarker  => [' ', 'G'],
            TileContent::BlueMarker   => [' ', 'B'],
            TileContent::Bat          => ['B', 'a'],
            TileContent::Beetle       => ['B', 'e'],
            TileContent::Book         => ['B', 'o'],
            TileContent::Candelabrum  => ['C', 'a'],
            TileContent::Crown        => ['C', 'r'],
            TileContent::Dragon       => ['D', 'r'],
            TileContent::Fairy        => ['F', 'a'],
            TileContent::Gem          => ['G', 'e'],
            TileContent::Genie        => ['G', 'n'],
            TileContent::Ghost        => ['G', 'h'],
            TileContent::Gold         => ['G', 'o'],
            TileContent::Helm         => ['H', 'e'],
            TileContent::Keys         => ['K', 'e'],
            TileContent::Map          => ['M', 'a'],
            TileContent::Moth         => ['M', 'o'],
            TileContent::Owl          => ['O', 'w'],
            TileContent::Rat          => ['R', 'a'],
            TileContent::Ring         => ['R', 'i'],
            TileContent::Spider       => ['S', 'p'],
            TileContent::Salamander   => ['S', 'a'],
            TileContent::Treasure     => ['T', 'r'],
            TileContent::Troll        => ['T', 'l'],
            TileContent::Skull        => ['S', 'k'],
            TileContent::Sword        => ['S', 'w']
        }
    }

    pub fn try_from_str(s: &str) -> Option<TileContent> {
        match s.trim().to_ascii_lowercase().as_str() {
            "y"  => Some(TileContent::YellowMarker),
            "r"  => Some(TileContent::RedMarker),
            "g"  => Some(TileContent::GreenMarker),
            "b"  => Some(TileContent::BlueMarker),
            "ba" => Some(TileContent::Bat),
            "be" => Some(TileContent::Beetle),
            "bo" => Some(TileContent::Book),
            "ca" => Some(TileContent::Candelabrum),
            "cr" => Some(TileContent::Crown),
            "dr" => Some(TileContent::Dragon),
            "fa" => Some(TileContent::Fairy),
            "ge" => Some(TileContent::Gem),
            "gn" => Some(TileContent::Genie),
            "gh" => Some(TileContent::Ghost),
            "go" => Some(TileContent::Gold),
            "he" => Some(TileContent::Helm),
            "ke" => Some(TileContent::Keys),
            "ma" => Some(TileContent::Map),
            "mo" => Some(TileContent::Moth),
            "ow" => Some(TileContent::Owl),
            "ra" => Some(TileContent::Rat),
            "ri" => Some(TileContent::Ring),
            "sa" => Some(TileContent::Salamander),
            "sp" => Some(TileContent::Spider),
            "tr" => Some(TileContent::Treasure),
            "tl" => Some(TileContent::Troll),
            "sk" => Some(TileContent::Skull),
            "sw" => Some(TileContent::Sword),
            _    => None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    kind: TileKind,
    content: TileContent
}

impl Tile {
    fn standard_set() -> Vec<Tile> {
        vec![
            Tile { kind: TileKind::Corner, content: TileContent::RedMarker },
            Tile { kind: TileKind::Corner, content: TileContent::BlueMarker },
            Tile { kind: TileKind::Corner, content: TileContent::YellowMarker },
            Tile { kind: TileKind::Corner, content: TileContent::GreenMarker },
            Tile { kind: TileKind::Junction, content: TileContent::Bat },
            Tile { kind: TileKind::Corner, content: TileContent::Beetle },
            Tile { kind: TileKind::Junction, content: TileContent::Book },
            Tile { kind: TileKind::Junction, content: TileContent::Candelabrum },
            Tile { kind: TileKind::Junction, content: TileContent::Crown },
            Tile { kind: TileKind::Junction, content: TileContent::Dragon },
            Tile { kind: TileKind::Junction, content: TileContent::Fairy },
            Tile { kind: TileKind::Junction, content: TileContent::Gem },
            Tile { kind: TileKind::Junction, content: TileContent::Gold },
            Tile { kind: TileKind::Junction, content: TileContent::Genie },
            Tile { kind: TileKind::Junction, content: TileContent::Ghost },
            Tile { kind: TileKind::Junction, content: TileContent::Helm },
            Tile { kind: TileKind::Junction, content: TileContent::Keys },
            Tile { kind: TileKind::Junction, content: TileContent::Map },
            Tile { kind: TileKind::Corner, content: TileContent::Moth },
            Tile { kind: TileKind::Corner, content: TileContent::Owl },
            Tile { kind: TileKind::Corner, content: TileContent::Rat },
            Tile { kind: TileKind::Junction, content: TileContent::Ring },
            Tile { kind: TileKind::Corner, content: TileContent::Salamander },
            Tile { kind: TileKind::Corner, content: TileContent::Spider },
            Tile { kind: TileKind::Junction, content: TileContent::Treasure },
            Tile { kind: TileKind::Junction, content: TileContent::Troll },
            Tile { kind: TileKind::Junction, content: TileContent::Skull },
            Tile { kind: TileKind::Junction, content: TileContent::Sword },
            Tile { kind: TileKind::Corner, content: TileContent::None },
            Tile { kind: TileKind::Corner, content: TileContent::None },
            Tile { kind: TileKind::Corner, content: TileContent::None },
            Tile { kind: TileKind::Corner, content: TileContent::None },
            Tile { kind: TileKind::Corner, content: TileContent::None },
            Tile { kind: TileKind::Corner, content: TileContent::None },
            Tile { kind: TileKind::Corner, content: TileContent::None },
            Tile { kind: TileKind::Corner, content: TileContent::None },
            Tile { kind: TileKind::Corner, content: TileContent::None },
            Tile { kind: TileKind::Corner, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
            Tile { kind: TileKind::Straight, content: TileContent::None },
        ]
    }

    // Outputs whether the tile, in a certain orientation, connects
    // towards North, East, South, West respectively.
    fn connections(self, orientation: Orientation) -> (bool, bool, bool, bool) {
        let (mut n, mut e, mut s, mut w) = (false, false, false, false);

        match (self.kind, orientation) {
            (TileKind::Straight, Orientation::North) | (TileKind::Straight, Orientation::South) => { n = true; s = true; },
            (TileKind::Straight, Orientation::East) | (TileKind::Straight, Orientation::West) => { e = true; w = true; },
            (TileKind::Corner, Orientation::North) => { n = true; e = true; },
            (TileKind::Corner, Orientation::East) => { e = true; s = true; },
            (TileKind::Corner, Orientation::South) => { s = true; w = true; },
            (TileKind::Corner, Orientation::West) => { w = true; n = true; },
            (TileKind::Junction, Orientation::North) => { e = true; s = true; w = true; },
            (TileKind::Junction, Orientation::East) => { s = true; w = true; n = true; },
            (TileKind::Junction, Orientation::South) => { w = true; n = true; e = true; },
            (TileKind::Junction, Orientation::West) => { n = true; e = true; s = true; }
        };
        (n, e, s, w)
    }

    fn to_ascii_3x6(self, orientation: Orientation) -> [[char; 6]; 3] {
        let mut res = [['\u{2588}'; 6]; 3];
        res[0][0] = '\u{2587}'; res[0][1] = '\u{2587}'; res[0][2] = '\u{2587}'; res[0][3] = '\u{2587}'; res[0][4] = '\u{2587}';
        res[0][5] = '\u{2589}';
        res[1][5] = '\u{2589}';
        res[2][5] = '\u{2589}';

        let (n, e, s, w) = self.connections(orientation);
        if n { res[0][2] = ' '; res[0][3] = ' '; }
        if e { res[1][4] = ' '; res[1][5] = ' '; }
        if s { res[2][2] = ' '; res[2][3] = ' '; }
        if w { res[1][0] = ' '; res[1][1] = ' '; }

        res[1][2..4].copy_from_slice(&self.content.to_chars());

        res
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    North,
    East,
    South,
    West
}

impl Orientation {
    fn random() -> Orientation {
        let mut rng = thread_rng();
        match rng.gen_range(0, 4) {
            0 => Orientation::North,
            1 => Orientation::East,
            2 => Orientation::South,
            3 => Orientation::West,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntryPoint {
    NorthLeft,
    NorthCenter,
    NorthRight,
    EastUp,
    EastCenter,
    EastDown,
    SouthLeft,
    SouthCenter,
    SouthRight,
    WestUp,
    WestCenter,
    WestDown
}

#[derive(Debug)]
pub struct Board {
    tiles: Vec<(Tile, Orientation)>,
    extra_tile: Tile,
    extra_pos: Option<EntryPoint>
}

impl Board {
    pub fn new() -> Board {
        // Generate tiles in a random order.
        let mut t = Tile::standard_set();
        t.shuffle(&mut thread_rng());

        // Generate random orientations.
        let mut o = vec![];
        o.resize_with(49, Orientation::random);

        // Setup fixed elements.
        Board::place(&mut t, &mut o, TileContent::RedMarker, 0, Orientation::East);
        Board::place(&mut t, &mut o, TileContent::Skull, 2, Orientation::North);
        Board::place(&mut t, &mut o, TileContent::Sword, 4, Orientation::North);
        Board::place(&mut t, &mut o, TileContent::BlueMarker, 6, Orientation::South);
        Board::place(&mut t, &mut o, TileContent::Gold, 14, Orientation::West);
        Board::place(&mut t, &mut o, TileContent::Keys, 16, Orientation::West);
        Board::place(&mut t, &mut o, TileContent::Gem, 18, Orientation::North);
        Board::place(&mut t, &mut o, TileContent::Helm, 20, Orientation::East);
        Board::place(&mut t, &mut o, TileContent::Book, 28, Orientation::West);
        Board::place(&mut t, &mut o, TileContent::Crown, 30, Orientation::South);
        Board::place(&mut t, &mut o, TileContent::Treasure, 32, Orientation::East);
        Board::place(&mut t, &mut o, TileContent::Candelabrum, 34, Orientation::East);
        Board::place(&mut t, &mut o, TileContent::YellowMarker, 42, Orientation::North);
        Board::place(&mut t, &mut o, TileContent::Map, 44, Orientation::South);
        Board::place(&mut t, &mut o, TileContent::Ring, 46, Orientation::South);
        Board::place(&mut t, &mut o, TileContent::GreenMarker, 48, Orientation::West);

        // Fill in the board.
        let mut tile_it = t.into_iter();
        let mut orient_it = o.into_iter();

        let mut tiles = vec![];
        tiles.resize_with(49, || (tile_it.next().unwrap(), orient_it.next().unwrap()));
        let extra_tile = tile_it.next().unwrap();

        Board { tiles, extra_tile, extra_pos: None }
    }

    // Make a move by pushing the extra tile into the board, at a given
    // entry point and with a given orientation.
    pub fn push_tile(&mut self, entry: EntryPoint, orientation: Orientation) {
        let (out_tile, out_pos) = match entry {
            EntryPoint::NorthLeft   => (self.push_north(1, orientation), EntryPoint::SouthLeft),
            EntryPoint::NorthCenter => (self.push_north(3, orientation), EntryPoint::SouthCenter),
            EntryPoint::NorthRight  => (self.push_north(5, orientation), EntryPoint::SouthRight),
            EntryPoint::EastUp      => (self.push_east(1, orientation), EntryPoint::WestUp),
            EntryPoint::EastCenter  => (self.push_east(3, orientation), EntryPoint::WestCenter),
            EntryPoint::EastDown    => (self.push_east(5, orientation), EntryPoint::WestDown),
            EntryPoint::SouthLeft   => (self.push_south(1, orientation), EntryPoint::NorthLeft),
            EntryPoint::SouthCenter => (self.push_south(3, orientation), EntryPoint::NorthCenter),
            EntryPoint::SouthRight  => (self.push_south(5, orientation), EntryPoint::NorthRight),
            EntryPoint::WestUp      => (self.push_west(1, orientation), EntryPoint::EastUp),
            EntryPoint::WestCenter  => (self.push_west(3, orientation), EntryPoint::EastCenter),
            EntryPoint::WestDown    => (self.push_west(5, orientation), EntryPoint::EastDown)
        };
        self.extra_tile = out_tile;
        self.extra_pos = Some(out_pos);
    }

    pub fn content_at(&self, row: usize, col: usize) -> TileContent {
        assert!(row < 7);
        assert!(col < 7);
        self.tiles[row*7+col].0.content
    }

    pub fn content_position(&self, content: TileContent) -> Option<(usize, usize)> {
        if let Some(i) = self.tiles.iter().position(|x| x.0.content == content) {
            Some((i / 7, i % 7))
        } else {
            None
        }
    }

    pub fn extra_tile_position(&self) -> Option<EntryPoint> {
        self.extra_pos
    }

    pub fn remove_content(&mut self, content: TileContent) {
        if let Some(i) = self.tiles.iter().position(|x| x.0.content == content) {
            self.tiles[i].0.content = TileContent::None;
        }
    }

    fn push_north(&mut self, col: usize, orientation: Orientation) -> Tile {
        let mut temp = (self.extra_tile, orientation);
        for row in 0..7 {
            std::mem::swap(self.tiles.get_mut(row*7+col).unwrap(), &mut temp);
        }
        temp.0
    }

    fn push_east(&mut self, row: usize, orientation: Orientation) -> Tile  {
        let mut temp = (self.extra_tile, orientation);
        for col in (0..7).rev() {
            std::mem::swap(self.tiles.get_mut(row*7+col).unwrap(), &mut temp);
        }
        temp.0
    }

    fn push_south(&mut self, col: usize, orientation: Orientation) -> Tile  {
        let mut temp = (self.extra_tile, orientation);
        for row in (0..7).rev() {
            std::mem::swap(self.tiles.get_mut(row*7+col).unwrap(), &mut temp);
        }
        temp.0
    }

    fn push_west(&mut self, row: usize, orientation: Orientation) -> Tile  {
        let mut temp = (self.extra_tile, orientation);
        for col in 0..7 {
            std::mem::swap(self.tiles.get_mut(row*7+col).unwrap(), &mut temp);
        }
        temp.0
    }

    fn place(t: &mut [Tile], o: &mut [Orientation], object: TileContent, position: usize, orientation: Orientation) {
        t.swap(t.iter().position(|x| x.content == object).unwrap(), position);
        o[position] = orientation;
    }

    pub fn to_ascii_29x56(&self) -> [[char; 56]; 29] {
        let mut res = [[' '; 56]; 29];

        // Draw markers on the perimeter.
        res[2][16] = '1'; res[2][28] = '2'; res[2][40] = '3';
        res[3][15] = '\\'; res[3][27] = '\\'; res[3][39] = '\\';
        res[3][16] = '/'; res[3][28] = '/'; res[3][40] = '/';
        res[8][49] = '<'; res[14][49] = '<'; res[20][49] = '<';
        res[8][50] = '4'; res[14][50] = '5'; res[20][50] = '6';
        res[25][15] = '/'; res[25][27] = '/'; res[25][39] = '/';
        res[25][16] = '\\'; res[25][28] = '\\'; res[25][40] = '\\';
        res[26][16] = '9'; res[26][28] = '8'; res[26][40] = '7';
        res[8][6] = '>'; res[14][6] = '>'; res[20][6] = '>';
        res[8][4] = '1'; res[14][4] = '1'; res[20][4] = '1';
        res[8][5] = '2'; res[14][5] = '1'; res[20][5] = '0';

        // Draw the tiles on the board.
        for r in 0..7 {
            for c in 0..7 {
                Board::put_ascii_tile(&mut res, 3*(1+r)+1, 6*(1+c)+1, self.tiles[r*7+c].0, self.tiles[r*7+c].1);
            }
        }

        // Draw the extra tile.
        let (offset_row, offset_col) = match self.extra_pos {
            None => (0, 0),
            Some(EntryPoint::NorthLeft) => (0, 13),
            Some(EntryPoint::NorthCenter) => (0, 25),
            Some(EntryPoint::NorthRight) => (0, 37),
            Some(EntryPoint::EastUp) => (7, 50),
            Some(EntryPoint::EastCenter) => (13, 50),
            Some(EntryPoint::EastDown) => (19, 50),
            Some(EntryPoint::SouthLeft) => (25, 13),
            Some(EntryPoint::SouthCenter) => (25, 25),
            Some(EntryPoint::SouthRight) => (25, 37),
            Some(EntryPoint::WestUp) => (7, 0),
            Some(EntryPoint::WestCenter) => (13, 0),
            Some(EntryPoint::WestDown) => (19, 0)
        };
        Board::put_ascii_tile(&mut res, offset_row, offset_col, self.extra_tile, Orientation::North);
        res
    }

    fn put_ascii_tile(grid: &mut [[char; 56]; 29], grid_row: usize, grid_col: usize, tile: Tile, orientation: Orientation) {
        let tile_ascii = tile.to_ascii_3x6(orientation);
        for r in 0..3 {
            grid[grid_row+r][grid_col..grid_col+6].copy_from_slice(&tile_ascii[r]);
        }
    }

    // Find locations reachable from a given position.
    pub fn reachable_from(&self, row: usize, col: usize) -> HashSet<(usize, usize)> {
        let mut reachable = HashSet::new();
        let mut frontier = VecDeque::new();

        frontier.push_back((row, col));
        reachable.insert((row, col));

        while let Some((r, c)) = frontier.pop_front() {
            let (ti, or) = self.tiles[r*7+c];
            let (n, e, s, w) = ti.connections(or);
            if n && r > 0 {
                let (ti, or) = self.tiles[(r-1)*7+c];
                let (_, _, s, _) = ti.connections(or);
                if s && !reachable.contains(&(r-1, c)) {
                    frontier.push_back((r-1, c));
                    reachable.insert((r-1, c));
                }
            }
            if e && c < 6 {
                let (ti, or) = self.tiles[r*7+(c+1)];
                let (_, _, _, w) = ti.connections(or);
                if w && !reachable.contains(&(r, c+1)) {
                    frontier.push_back((r, c+1));
                    reachable.insert((r, c+1));
                }
            }
            if s && r < 6 {
                let (ti, or) = self.tiles[(r+1)*7+c];
                let (n, _, _, _) = ti.connections(or);
                if n && !reachable.contains(&(r+1, c)) {
                    frontier.push_back((r+1, c));
                    reachable.insert((r+1, c));
                }
            }
            if w && c > 0 {
                let (ti, or) = self.tiles[r*7+(c-1)];
                let (_, e, _, _) = ti.connections(or);
                if e && !reachable.contains(&(r, c-1)) {
                    frontier.push_back((r, c-1));
                    reachable.insert((r, c-1));
                }
            }
        }
        reachable
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
