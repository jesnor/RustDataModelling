use std::{
    ops::Deref,
    rc::{Rc, Weak},
};

use ghost_cell::{GhostCell, GhostToken};

struct Ref<'brand, T>(Rc<GhostCell<'brand, T>>);

impl<'brand, T> Ref<'brand, T> {
    fn new(v: T) -> Self {
        Self(Rc::new(GhostCell::new(v)))
    }

    fn weak(&self) -> Weak<GhostCell<'brand, T>> {
        Rc::downgrade(&self.0)
    }
}

impl<'brand, T> Clone for Ref<'brand, T> {
    fn clone(&self) -> Self {
        Ref(self.0.clone())
    }
}

impl<'brand, T> Deref for Ref<'brand, T> {
    type Target = GhostCell<'brand, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type WRef<'brand, T> = Weak<GhostCell<'brand, T>>;

struct Player<'brand> {
    game: GameWRef<'brand>,
    name: String,
    health: i32,
    friends: Vec<PlayerWRef<'brand>>,
}

type PlayerRef<'brand> = Ref<'brand, Player<'brand>>;
type PlayerWRef<'brand> = WRef<'brand, Player<'brand>>;

fn make_friends<'brand>(
    token: &mut GhostToken<'brand>,
    player1: &PlayerRef<'brand>,
    player2: &PlayerRef<'brand>,
) {
    player1.borrow_mut(token).friends.push(player2.weak());
    player2.borrow_mut(token).friends.push(player1.weak());
}

#[derive(Default)]
struct Game<'brand> {
    players: Vec<PlayerRef<'brand>>,
}

type GameRef<'brand> = Ref<'brand, Game<'brand>>;
type GameWRef<'brand> = WRef<'brand, Game<'brand>>;

fn create_player<'brand>(
    token: &mut GhostToken<'brand>,
    game: &GameRef<'brand>,
    name: &str,
    health: i32,
) -> PlayerRef<'brand> {
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
