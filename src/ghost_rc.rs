use std::{
    ops::Deref,
    rc::{Rc, Weak},
};

use ghost_cell::{GhostCell, GhostToken};

struct Ref<'t, T>(Rc<GhostCell<'t, T>>);

impl<'t, T> Ref<'t, T> {
    fn new(v: T) -> Self {
        Self(Rc::new(GhostCell::new(v)))
    }

    fn weak(&self) -> Weak<GhostCell<'t, T>> {
        Rc::downgrade(&self.0)
    }
}

impl<'t, T> Clone for Ref<'t, T> {
    fn clone(&self) -> Self {
        Ref(self.0.clone())
    }
}

impl<'t, T> Deref for Ref<'t, T> {
    type Target = GhostCell<'t, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type WRef<'t, T> = Weak<GhostCell<'t, T>>;

struct Player<'t> {
    game: GameWRef<'t>,
    name: String,
    health: i32,
    friends: Vec<PlayerWRef<'t>>,
}

type PlayerRef<'t> = Ref<'t, Player<'t>>;
type PlayerWRef<'t> = WRef<'t, Player<'t>>;

fn make_friends<'t>(token: &mut GhostToken<'t>, player1: &PlayerRef<'t>, player2: &PlayerRef<'t>) {
    player1.borrow_mut(token).friends.push(player2.weak());
    player2.borrow_mut(token).friends.push(player1.weak());
}

#[derive(Default)]
struct Game<'t> {
    players: Vec<PlayerRef<'t>>,
}

type GameRef<'t> = Ref<'t, Game<'t>>;
type GameWRef<'t> = WRef<'t, Game<'t>>;

fn create_player<'t>(
    token: &mut GhostToken<'t>,
    game: &GameRef<'t>,
    name: &str,
    health: i32,
) -> PlayerRef<'t> {
    let p = Ref::new(Player {
        game: game.weak(),
        name: name.into(),
        health,
        friends: Default::default(),
    });

    game.borrow_mut(token).players.push(p.clone());
    p
}

pub fn run_game() {
    GhostToken::new(|mut token| {
        let game = Ref::new(Default::default());
        let t = &mut token;

        let p1 = create_player(t, &game, "Eric", 10);
        let p2 = create_player(t, &game, "Tom", 15);
        let p3 = create_player(t, &game, "Carl", 17);

        make_friends(t, &p1, &p2);
        make_friends(t, &p1, &p3);

        p2.borrow_mut(t).health = 20;

        for x in p1.borrow(t).friends.iter() {
            println!(
                "{}: {}",
                x.upgrade().unwrap().borrow(t).name,
                x.upgrade().unwrap().borrow(t).health
            )
        }
    });
}
