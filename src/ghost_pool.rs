use ghost_cell::{GhostCell, GhostToken};

use crate::cell_pool::CellPool;

struct GCell<'brand, T>(GhostCell<'brand, T>);

impl<'brand, T> GCell<'brand, T> {
    fn new(value: T) -> Self {
        Self(GhostCell::new(value))
    }

    fn borrow<'a>(&'a self, token: &'a GhostToken<'brand>) -> &'a T {
        self.0.borrow(token)
    }

    fn borrow_mut<'a>(&'a self, token: &'a mut GhostToken<'brand>) -> &'a mut T {
        self.0.borrow_mut(token)
    }
}

impl<'brand, T: Default> Default for GCell<'brand, T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

type Ref<'t, 'brand, T> = &'t GCell<'brand, T>;

#[derive(Default)]
struct Player<'t, 'brand> {
    game: Option<GameRef<'t, 'brand>>,
    name: String,
    health: i32,
    friends: Vec<PlayerRef<'t, 'brand>>,
}

impl<'t, 'brand> Player<'t, 'brand> {
    fn init(&mut self, game: GameRef<'t, 'brand>, name: &str, health: i32) {
        self.game = Some(game);
        self.name = name.to_owned();
        self.health = health;
    }
}

type PlayerRef<'t, 'brand> = Ref<'t, 'brand, Player<'t, 'brand>>;

fn make_friends<'t, 'brand>(
    token: &mut GhostToken<'brand>,
    player1: PlayerRef<'t, 'brand>,
    player2: PlayerRef<'t, 'brand>,
) {
    player1.borrow_mut(token).friends.push(player2);
    player2.borrow_mut(token).friends.push(player1);
}

struct Game<'t, 'brand> {
    players: &'t CellPool<GCell<'brand, Player<'t, 'brand>>>,
}

type GameRef<'t, 'brand> = Ref<'t, 'brand, Game<'t, 'brand>>;

fn create_player<'t, 'brand>(
    token: &mut GhostToken<'brand>,
    game: GameRef<'t, 'brand>,
    name: &str,
    health: i32,
) -> Result<PlayerRef<'t, 'brand>, &'static str> {
    let p = game.borrow_mut(token).players.alloc()?;
    p.borrow_mut(token).init(game, name, health);
    Ok(p)
}

pub fn run_game() -> Result<(), &'static str> {
    GhostToken::new(|mut token| {
        let players = CellPool::new(100);
        let game = GCell::new(Game { players: &players });
        let t = &mut token;

        let p1 = create_player(t, &game, "Eric", 10)?;
        let p2 = create_player(t, &game, "Tom", 15)?;
        let p3 = create_player(t, &game, "Carl", 17)?;

        make_friends(t, &p1, &p2);
        make_friends(t, &p1, &p3);

        p2.borrow_mut(t).health = 20;

        for x in p1.borrow(t).friends.iter() {
            println!("{}: {}", x.borrow(t).name, x.borrow(t).health)
        }

        Ok(())
    })
}
