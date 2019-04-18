use specs::World;

pub enum Continuation {
    Continue,
    Exit,
}

pub struct GameState {}

impl GameState {
    // Create entities, load stuff, etc
    pub fn new() -> GameState {
        GameState {}
    }

    // Called every loop
    pub fn update(&mut self, world: &mut World) -> Continuation {
        Continuation::Continue
    }

    // Called when the loop breaks
    pub fn stop(&mut self, world: &mut World) {}
}
