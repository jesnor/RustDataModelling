use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Player {
    game: GameWRef,
    name: String,
    health: i32,
    friends: Vec<PlayerWRef>,
}

type PlayerRef = Rc<RefCell<Player>>;
type PlayerWRef = Weak<RefCell<Player>>;

fn make_friends(player1: &PlayerRef, player2: &PlayerRef) {
    player1.borrow_mut().friends.push(Rc::downgrade(player2));
    player2.borrow_mut().friends.push(Rc::downgrade(player1));
}

#[derive(Default)]
struct Game {
    players: Vec<PlayerRef>,
}

type GameRef = Rc<RefCell<Game>>;
type GameWRef = Weak<RefCell<Game>>;

fn create_player(game: &GameRef, name: &str, health: i32) -> PlayerRef {
    let p: Rc<RefCell<Player>> = Rc::new(RefCell::new(Player {
        game: Rc::downgrade(game),
        name: name.into(),
        health,
        friends: Default::default(),
    }));

    game.borrow_mut().players.push(p.clone());
    p
}

pub fn run_game() {
    let game: GameRef = Default::default();

    let p1 = create_player(&game, "Eric", 10);
    let p2 = create_player(&game, "Tom", 15);
    let p3 = create_player(&game, "Carl", 17);

    make_friends(&p1, &p2);
    make_friends(&p1, &p3);

    p2.borrow_mut().health = 20;

    for x in p1.borrow().friends.iter() {
        println!(
            "{}: {}",
            x.upgrade().unwrap().borrow().name,
            x.upgrade().unwrap().borrow().health
        )
    }
}
