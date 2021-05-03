use crate::fixed_vec::{Clear, FixedVec};

use super::ref_set::RefSet;
use std::cell::{Cell, RefCell};

#[derive(Default, Clone)]
struct Player {
    game: Cell<Option<GameRef>>,
    name: RefCell<String>,
    health: Cell<i32>,
    friends: RefSet<'static, Player>,
}

impl Player {
    fn init(&self, game: GameRef, name: &str, health: i32) {
        self.game.set(Some(game));
        *self.name.borrow_mut() = name.to_owned();
        self.health.set(health);
    }

    fn make_friends(&'static self, player2: PlayerRef) {
        self.friends.add(player2);
        player2.friends.add(self);
    }
}

impl Clear for Player {
    fn clear(&self) {
        self.name.borrow_mut().clear();
    }
}

type PlayerRef = &'static Player;

struct Game {
    // This is a simplification, the actual implementation has to handle player removal as well
    players: FixedVec<'static, Player>,
}

impl Game {
    fn new(players: &'static [Player]) -> Self {
        Self {
            players: FixedVec::new(players),
        }
    }

    fn create_player(&'static self, name: &str, health: i32) -> PlayerRef {
        let p = &self.players.alloc();
        p.init(self, name, health);
        p
    }
}

type GameRef = &'static Game;

pub fn run_game() {
    let players = Box::leak(vec![Player::default(); 100].into_boxed_slice());
    let game = Box::leak(Box::new(Game::new(players)));

    let p1 = game.create_player("Eric", 10);
    let p2 = game.create_player("Tom", 15);
    let p3 = game.create_player("Carl", 17);

    p1.make_friends(p2);
    p1.make_friends(p3);

    p2.health.set(20);

    for x in p1.friends.iter() {
        println!("{}: {}", x.name.borrow(), x.health.get())
    }
}
