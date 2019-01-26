use amethyst::SimpleState;
use amethyst::StateData;
use amethyst::GameData;

pub struct SeekingWarmth;

impl SimpleState for SeekingWarmth {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;


    }
}

