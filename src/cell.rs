use crate::{cell_pool::CellPool, clear::Clear};

use super::ref_set::RefSet;
use std::cell::{Cell, RefCell};

#[derive(Default, Clone)]
struct Player<'t> {
    game: Cell<Option<GameRef<'t>>>,
    name: RefCell<String>,
    health: Cell<i32>,
    friends: RefSet<'t, Player<'t>>,
}

impl<'t> Player<'t> {
    fn init(&self, game: GameRef<'t>, name: &str, health: i32) {
        self.game.set(Some(game));
        *self.name.borrow_mut() = name.to_owned();
        self.health.set(health);
    }

    fn make_friends(&'t self, player2: PlayerRef<'t>) {
        self.friends.add(player2);
        player2.friends.add(self);
    }
}

impl<'t> Clear for Player<'t> {
    fn clear(&self) {
        self.name.borrow_mut().clear();
    }
}

type PlayerRef<'t> = &'t Player<'t>;

struct Game<'t> {
    players: CellPool<Player<'t>>,
}

impl<'t> Game<'t> {
    fn new(max_player_count: usize) -> Self {
        Self {
            players: CellPool::new(max_player_count),
        }
    }

    fn create_player(&'t self, name: &str, health: i32) -> Result<PlayerRef<'t>, &'static str> {
        let p = self.players.alloc()?;
        p.init(self, name, health);
        Ok(p)
    }
}

type GameRef<'t> = &'t Game<'t>;

pub fn run_game() -> Result<(), &'static str> {
    let game = Game::new(100);

    let p1 = game.create_player("Eric", 10)?;
    let p2 = game.create_player("Tom", 15)?;
    let p3 = game.create_player("Carl", 17)?;

    p1.make_friends(p2);
    p1.make_friends(p3);

    p2.health.set(20);

    for x in p1.friends.iter() {
        println!("{}: {}", x.name.borrow(), x.health.get())
    }

    Ok(())
}
