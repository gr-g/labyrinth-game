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
                (EntryPoint::WestDown, Orientation::West)
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

pub fn list_all_moves() -> Vec<(EntryPoint, Orientation)> {
    vec![
        (EntryPoint::NorthLeft, Orientation::North),
        (EntryPoint::NorthLeft, Orientation::East),
        (EntryPoint::NorthLeft, Orientation::South),
        (EntryPoint::NorthLeft, Orientation::West),
        (EntryPoint::NorthCenter, Orientation::North),
        (EntryPoint::NorthCenter, Orientation::East),
        (EntryPoint::NorthCenter, Orientation::South),
        (EntryPoint::NorthCenter, Orientation::West),
        (EntryPoint::NorthRight, Orientation::North),
        (EntryPoint::NorthRight, Orientation::East),
        (EntryPoint::NorthRight, Orientation::South),
        (EntryPoint::NorthRight, Orientation::West),
        (EntryPoint::EastUp, Orientation::North),
        (EntryPoint::EastUp, Orientation::East),
        (EntryPoint::EastUp, Orientation::South),
        (EntryPoint::EastUp, Orientation::West),
        (EntryPoint::EastCenter, Orientation::North),
        (EntryPoint::EastCenter, Orientation::East),
        (EntryPoint::EastCenter, Orientation::South),
        (EntryPoint::EastCenter, Orientation::West),
        (EntryPoint::EastDown, Orientation::North),
        (EntryPoint::EastDown, Orientation::East),
        (EntryPoint::EastDown, Orientation::South),
        (EntryPoint::EastDown, Orientation::West),
        (EntryPoint::SouthLeft, Orientation::North),
        (EntryPoint::SouthLeft, Orientation::East),
        (EntryPoint::SouthLeft, Orientation::South),
        (EntryPoint::SouthLeft, Orientation::West),
        (EntryPoint::SouthCenter, Orientation::North),
        (EntryPoint::SouthCenter, Orientation::East),
        (EntryPoint::SouthCenter, Orientation::South),
        (EntryPoint::SouthCenter, Orientation::West),
        (EntryPoint::SouthRight, Orientation::North),
        (EntryPoint::SouthRight, Orientation::East),
        (EntryPoint::SouthRight, Orientation::South),
        (EntryPoint::SouthRight, Orientation::West),
        (EntryPoint::WestUp, Orientation::North),
        (EntryPoint::WestUp, Orientation::East),
        (EntryPoint::WestUp, Orientation::South),
        (EntryPoint::WestUp, Orientation::West),
        (EntryPoint::WestCenter, Orientation::North),
        (EntryPoint::WestCenter, Orientation::East),
        (EntryPoint::WestCenter, Orientation::South),
        (EntryPoint::WestCenter, Orientation::West),
        (EntryPoint::WestDown, Orientation::North),
        (EntryPoint::WestDown, Orientation::East),
        (EntryPoint::WestDown, Orientation::South),
        (EntryPoint::WestDown, Orientation::West)
    ]
}

// For each own move, analyzes a set of scenarios (opponent moves) and returns
// the number of times that the target object will be reachable in that scenario.
pub fn object_reachable_scenarios(board: &Board, row: usize, col: usize, obj: TileContent) -> HashMap<((EntryPoint, Orientation), (usize, usize)), usize> {
    // List all possible first moves.
    let moves = list_all_moves();

    // Here we will store, for each move, the number of scenarios in
    // which the target is reachable on the next move.
    let mut reachable_target_count = HashMap::new();

    // Apply each move to a copy of the board.
    for m in &moves {
        if Some(m.0) == board.extra_tile_position() {
            continue;
        }

        let mut new_board = board.clone();
        let mut pos = (row, col);
        new_board.push_tile(m.0, m.1, &mut [&mut pos]);

        // List all positions reachable from pos and collect them in a vector.
        let reach = new_board.reachable_from(pos.0, pos.1);
        let reach: Vec<_> = reach.into_iter().collect();

        for i in 1..reach.len() {
            reachable_target_count.insert((*m, reach[i]), 0);
        }

        // Apply the scenarios to the board and add 1 to the count if in that
        // scenario the target object is reachable in 1 move from the (possibly
        // shifted) reachable position.
        for m2 in &moves {
            let mut new_board2 = new_board.clone();
            let mut reach2 = reach.clone();
            let mut reach2_itermut: Vec<_> = reach2.iter_mut().collect();
            new_board2.push_tile(m2.0, m2.1, &mut reach2_itermut);

            for i in 1..reach2.len() {
                if objects_reachable_in_1_move_from(&new_board2, reach2[i].0, reach2[i].1).contains_key(&obj) {
                    *reachable_target_count.get_mut(&(*m, reach[i])).unwrap() += 1;
                }
            }
        }
    }
    reachable_target_count
}
