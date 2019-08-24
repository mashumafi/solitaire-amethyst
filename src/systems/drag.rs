use amethyst::{
    assets::AssetStorage,
    core::{
        math::{Point4, Vector2, Vector3},
        transform::Transform,
    },
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions,
    winit::MouseButton,
};

use log::info;

use crate::{
    components::{BoomerangComponent, CardComponent, CardState, DragComponent},
    math::{screen_to_world, Rectangle2},
};

pub struct DragSystem {
    point: Option<Vector2<f32>>,
}

impl Default for DragSystem {
    fn default() -> Self {
        DragSystem { point: None }
    }
}

impl<'a> System<'a> for DragSystem {
    type SystemData = (
        Read<'a, AssetStorage<SpriteSheet>>,
        Read<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, ScreenDimensions>,
        ReadStorage<'a, CardComponent>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, SpriteRender>,
        WriteStorage<'a, BoomerangComponent>,
        WriteStorage<'a, DragComponent>,
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
            mut boomerangs,
            mut drags,
            mut transformations,
            entities,
        ): Self::SystemData,
    ) {
        let was_pressed = self.point.is_some();
        let is_pressed = input.mouse_button_is_down(MouseButton::Left);

        if !is_pressed {
            let mut to_remove = Vec::new();
            self.point = None;
            for (sprite_transform, drag, boomerang, entity) in
                (&mut transformations, &mut drags, &mut boomerangs, &entities).join()
            {
                if drag.selected {
                    to_remove.push(entity);
                    drag.selected = false;
                    if let Some(original) = boomerang.original {
                        sprite_transform.set_translation_xyz(original.x, original.y, original.z);
                        boomerang.original = None;
                    }
                }
            }
            for entity in to_remove {
                sprites.get(entity);
                /*match entities.delete(entity) {
                    Ok(()) => info!("deleted"),
                    Err(e) => info!("failed {}", e),
                }*/
            }
            return;
        }

        if let Some(cursor) = screen_to_world(&cameras, &input, &dimension, &transformations) {
            if was_pressed {
                let last_cursor = self.point.unwrap();
                let offset = Vector2::new(cursor.x - last_cursor.x, cursor.y - last_cursor.y);
                for (sprite_transform, drag) in (&mut transformations, &drags).join() {
                    if drag.selected {
                        sprite_transform.prepend_translation(Vector3::new(offset.x, offset.y, 0.));
                    }
                }
            } else {
                let mut selected: Option<(&mut DragComponent, f32)> = None;
                for (sprite, sprite_transform, drag, boomerang) in
                    (&sprites, &transformations, &mut drags, &mut boomerangs).join()
                {
                    if let Some(sprite_sheet) = sheet_storage.get(&sprite.sprite_sheet) {
                        let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                        boomerang.original = Some(*sprite_transform.translation());
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
                            if let Some((_previous, z)) = &mut selected {
                                if *z < middle.z {
                                    selected = Some((drag, middle.z));
                                }
                            } else {
                                selected = Some((drag, middle.z));
                            }
                        }
                    }
                }
                if let Some((drag, _z)) = &mut selected {
                    drag.selected = true;
                }
            }
            self.point = Some(Vector2::new(cursor.x, cursor.y));
        }
    }
}
