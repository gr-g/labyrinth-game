use labyrinth_game::Game;

fn main() {
    let mut g = Game::new();

    while !g.is_over() {
        g.play_turn();
    }
}
