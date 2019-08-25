use amethyst::{
    assets::AssetStorage,
    core::{
        math::{Point4, Vector2, Vector3},
        transform::Transform,
    },
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions,
    winit::MouseButton,
};

use log::info;

use crate::{
    components::{CardComponent, CardState, DragComponent, StackComponent},
    math::{screen_to_world, Rectangle2},
    resources::CardResource,
};

pub struct FlipSystem {}

impl Default for FlipSystem {
    fn default() -> Self {
        FlipSystem {}
    }
}

impl<'a> System<'a> for FlipSystem {
    type SystemData = (
        Read<'a, AssetStorage<SpriteSheet>>,
        Read<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, ScreenDimensions>,
        WriteExpect<'a, CardResource>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, CardComponent>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            sheet_storage,
            input,
            dimension,
            mut card_resource,
            cameras,
            mut cards,
            mut sprites,
            mut transformations,
            entities,
        ): Self::SystemData,
    ) {
        for (card, card_entity) in (&cards, &entities).join() {
            sprites.insert(
                card_entity,
                match card.state {
                    CardState::TableauCovered => card_resource.back(),
                    _ => card_resource.face(card.card),
                },
            );
        }
    }
}
