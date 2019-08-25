use amethyst::{
    assets::AssetStorage,
    core::{
        math::{Point4, Vector2, Vector3},
        transform::Transform,
    },
    ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions,
    winit::MouseButton,
};

use log::info;

use crate::{
    components::{CardComponent, CardState, StackComponent},
    math::{screen_to_world, Rectangle2},
};

pub struct StackSystem {
    point: Option<Vector2<f32>>,
}

impl Default for StackSystem {
    fn default() -> Self {
        StackSystem { point: None }
    }
}

impl<'a> System<'a> for StackSystem {
    type SystemData = (
        Read<'a, AssetStorage<SpriteSheet>>,
        Read<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, ScreenDimensions>,
        ReadStorage<'a, CardComponent>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, SpriteRender>,
        WriteStorage<'a, StackComponent>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            sheet_storage,
            input,
            dimension,
            cards,
            cameras,
            sprites,
            mut stacks,
            mut transformations,
            entities,
        ): Self::SystemData,
    ) {
        let was_pressed = self.point.is_some();
        let is_pressed = input.mouse_button_is_down(MouseButton::Left);

        if !is_pressed {
            self.point = None;
            if let Some((sprite_transform, stack, entity)) =
                (&mut transformations, &mut stacks, &entities).join().next()
            {
                sprite_transform.set_translation_xyz(
                    stack.original.x,
                    stack.original.y,
                    stack.original.z,
                );
                stacks.remove(entity);
            }
            return;
        }

        if let Some(cursor) = screen_to_world(&cameras, &input, &dimension, &transformations) {
            if was_pressed {
                let last_cursor = self.point.unwrap();
                let offset = Vector2::new(cursor.x - last_cursor.x, cursor.y - last_cursor.y);
                for (sprite_transform, stack) in (&mut transformations, &stacks).join() {
                    sprite_transform.prepend_translation(Vector3::new(offset.x, offset.y, 0.));
                }
            } else {
                let mut selected: Option<(Entity, Vector3<f32>)> = None;
                for (sprite, sprite_transform, card, entity) in
                    (&sprites, &transformations, &cards, &entities).join()
                {
                    if let Some(sprite_sheet) = sheet_storage.get(&sprite.sprite_sheet) {
                        let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                        let texture_rect = Rectangle2::new(
                            -sprite.width,
                            -sprite.height,
                            sprite.width,
                            sprite.height,
                        ) / 2.;
                        let global_matrix = *sprite_transform.global_matrix();
                        let middle = global_matrix * Point4::new(0., 0., 0., 1.);
                        let transform_rect = texture_rect * global_matrix;
                        if transform_rect.contains(cursor.x, cursor.y) {
                            if let Some((_previous, point)) = &mut selected {
                                if point.z < middle.z {
                                    selected = Some((entity, *sprite_transform.translation()));
                                }
                            } else {
                                selected = Some((entity, *sprite_transform.translation()));
                            }
                        }
                    }
                }
                if let Some((entity, point)) = &mut selected {
                    stacks.insert(*entity, StackComponent::new(*point));
                }
            }
            self.point = Some(Vector2::new(cursor.x, cursor.y));
        }
    }
}
