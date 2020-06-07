use crate::board::{TileKind, TileContent, Board, EntryPoint, Orientation};
use std::collections::HashMap;

pub fn objects_reachable_in_1_move_from(board: &Board, row: usize, col: usize) -> HashMap<TileContent, Vec<EntryPoint>> {
    // List the moves that we have to analyse.
    let moves = match board.extra_tile().kind {
        TileKind::Straight => {
            vec![
                (EntryPoint::NorthLeft, Orientation::North),
                (EntryPoint::NorthLeft, Orientation::East),
                (EntryPoint::NorthCenter, Orientation::North),
                (EntryPoint::NorthCenter, Orientation::East),
                (EntryPoint::NorthRight, Orientation::North),
                (EntryPoint::NorthRight, Orientation::East),
                (EntryPoint::EastUp, Orientation::North),
                (EntryPoint::EastUp, Orientation::East),
                (EntryPoint::EastCenter, Orientation::North),
                (EntryPoint::EastCenter, Orientation::East),
                (EntryPoint::EastDown, Orientation::North),
                (EntryPoint::EastDown, Orientation::East),
                (EntryPoint::SouthLeft, Orientation::North),
                (EntryPoint::SouthLeft, Orientation::East),
                (EntryPoint::SouthCenter, Orientation::North),
                (EntryPoint::SouthCenter, Orientation::East),
                (EntryPoint::SouthRight, Orientation::North),
                (EntryPoint::SouthRight, Orientation::East),
                (EntryPoint::WestUp, Orientation::North),
                (EntryPoint::WestUp, Orientation::East),
                (EntryPoint::WestCenter, Orientation::North),
                (EntryPoint::WestCenter, Orientation::East),
                (EntryPoint::WestDown, Orientation::North),
                (EntryPoint::WestDown, Orientation::East)
            ]
        },
        TileKind::Corner => {
            vec![
                (EntryPoint::NorthLeft, Orientation::East),
                (EntryPoint::NorthLeft, Orientation::South),
                (EntryPoint::NorthCenter, Orientation::East),
                (EntryPoint::NorthCenter, Orientation::South),
                (EntryPoint::NorthRight, Orientation::East),
                (EntryPoint::NorthRight, Orientation::South),
                (EntryPoint::EastUp, Orientation::South),
                (EntryPoint::EastUp, Orientation::West),
                (EntryPoint::EastCenter, Orientation::South),
                (EntryPoint::EastCenter, Orientation::West),
                (EntryPoint::EastDown, Orientation::South),
                (EntryPoint::EastDown, Orientation::West),
                (EntryPoint::SouthLeft, Orientation::West),
                (EntryPoint::SouthLeft, Orientation::North),
                (EntryPoint::SouthCenter, Orientation::West),
                (EntryPoint::SouthCenter, Orientation::North),
                (EntryPoint::SouthRight, Orientation::West),
                (EntryPoint::SouthRight, Orientation::North),
                (EntryPoint::WestUp, Orientation::North),
                (EntryPoint::WestUp, Orientation::East),
                (EntryPoint::WestCenter, Orientation::North),
                (EntryPoint::WestCenter, Orientation::East),
                (EntryPoint::WestDown, Orientation::North),
                (EntryPoint::WestDown, Orientation::East)
            ]
        },
        TileKind::Junction => {
            vec![
                (EntryPoint::NorthLeft, Orientation::North),
                (EntryPoint::NorthCenter, Orientation::North),
                (EntryPoint::NorthRight, Orientation::North),
                (EntryPoint::EastUp, Orientation::East),
                (EntryPoint::EastCenter, Orientation::East),
                (EntryPoint::EastDown, Orientation::East),
                (EntryPoint::SouthLeft, Orientation::South),
                (EntryPoint::SouthCenter, Orientation::South),
                (EntryPoint::SouthRight, Orientation::South),
                (EntryPoint::WestUp, Orientation::West),
                (EntryPoint::WestCenter, Orientation::West),
                (EntryPoint::WestDown, Orientation::West),
            ]
        }
    };

    // Apply each move to a copy of the board.
    let mut reachable_obj = HashMap::new();

    for m in moves {
        if Some(m.0) == board.extra_tile_position() {
            continue;
        }

        let mut new_board = board.clone();
        let mut pos = (row, col);
        new_board.push_tile(m.0, m.1, &mut [&mut pos]);
        for o in new_board.objects_reachable_from(pos.0, pos.1) {
            let pushes = reachable_obj.entry(o).or_insert(Vec::new());
            if !pushes.contains(&m.0) {
                pushes.push(m.0);
            }
        }
    }
    reachable_obj
}
