use crate::engine::Engine;
// mod piece;
pub struct Interface {
    engine: Engine,
}

impl Interface {
    pub fn run(engine: Engine) {
        let interface = Self { engine };
        drop(interface);
        todo!("Run the game");
    }
}
