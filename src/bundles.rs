use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, error::Error};

use crate::systems::{DeckSystem, FlipSystem, StackSystem};

pub struct SolitaireBundle;
impl SolitaireBundle {}

impl Default for SolitaireBundle {
    fn default() -> Self {
        SolitaireBundle {}
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for SolitaireBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(FlipSystem::default(), "flip_system", &[]);
        builder.add(StackSystem::default(), "stack_system", &[]);
        builder.add(DeckSystem::default(), "deck_system", &[]);
        Ok(())
    }
}
