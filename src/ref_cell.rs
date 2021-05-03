use std::cell::{Ref, RefCell, RefMut};

struct Player<'t> {
    game: GameRef<'t>,
    name: String,
    health: i32,
    friends: Vec<PlayerRef<'t>>,
}

#[derive(Copy, Clone)]
struct PlayerRef<'t> {
    game: GameRef<'t>,
    index: usize,
}

impl<'t> PlayerRef<'t> {
    fn borrow(&self) -> Ref<'_, Player<'t>> {
        Ref::map(self.game.borrow(), |r| &r.players[self.index])
    }

    fn borrow_mut(&self) -> RefMut<'_, Player<'t>> {
        RefMut::map(self.game.borrow_mut(), |r| &mut r.players[self.index])
    }

    fn make_friends(self, player2: PlayerRef<'t>) {
        self.borrow_mut().friends.push(player2);
        player2.borrow_mut().friends.push(self);
    }
}

struct Game<'t> {
    players: Vec<Player<'t>>,
}

impl Default for Game<'_> {
    fn default() -> Self {
        Self {
            players: Default::default(),
        }
    }
}

type GameRef<'t> = &'t RefCell<Game<'t>>;

fn create_player<'t>(game: GameRef<'t>, name: &str, health: i32) -> PlayerRef<'t> {
    let p = Player {
        game,
        name: name.into(),
        health,
        friends: Default::default(),
    };

    let mut g = game.borrow_mut();
    g.players.push(p);

    PlayerRef {
        game: game,
        index: g.players.len() - 1,
    }
}

pub fn run_game() {
    let game: RefCell<Game<'_>> = Default::default();

    let p1 = create_player(&game, "Eric", 10);
    let p2 = create_player(&game, "Tom", 15);
    let p3 = create_player(&game, "Carl", 17);

    p1.make_friends(p2);
    p1.make_friends(p3);

    p2.borrow_mut().health = 20;

    for x in p1.borrow().friends.iter() {
        println!("{}: {}", x.borrow().name, x.borrow().health)
    }
}
