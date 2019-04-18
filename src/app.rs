use crate::game::{Continuation, GameState};
use specs::{Dispatcher, World};

pub struct Application<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    world: World,
    game: GameState,
}

impl<'a, 'b> Application<'a, 'b> {
    pub fn new(dispatcher: Dispatcher<'a, 'b>, game: GameState) -> Result<Application<'a, 'b>, u8> {
        let world = World::new();

        Ok(Application {
            dispatcher,
            world,
            game,
        })
    }

    pub fn run(&mut self) -> Result<(), u8> {
        loop {
            match self.game.update(&mut self.world) {
                Continuation::Continue => {
                    self.dispatcher.dispatch(&self.world.res);
                    self.world.maintain();
                }
                Continuation::Exit => {
                    self.game.stop(&mut self.world);
                    break;
                }
            }
        }

        Ok(())
    }
}
