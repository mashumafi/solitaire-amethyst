use amethyst::{
    assets::AssetStorage,
    core::{
        math::{Point3, Point4, Vector2, Vector3},
        transform::Transform,
    },
    ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions,
    winit::MouseButton,
};

use log::info;
use std::iter::Map;

use crate::{
    components::{CardComponent, CardState, FoundationComponent, StackComponent, TableauComponent},
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

fn get_under_cursor<'a>(
    cursor: Point3<f32>,
    sheet_storage: Read<'a, AssetStorage<SpriteSheet>>,
    sprites: ReadStorage<'a, SpriteRender>,
    transformations: &WriteStorage<'a, Transform>,
    entities: Entities<'a>,
) -> Vec<(Entity, Vector3<f32>)> {
    (&sprites, transformations, &entities)
        .join()
        .filter(|(sprite, sprite_transform, entity)| {
            if let Some(sprite_sheet) = sheet_storage.get(&sprite.sprite_sheet) {
                let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                let texture_rect =
                    Rectangle2::new(-sprite.width, -sprite.height, sprite.width, sprite.height)
                        / 2.;
                let global_matrix = *sprite_transform.global_matrix();
                let transform_rect = texture_rect * global_matrix;
                transform_rect.contains(cursor.x, cursor.y)
            } else {
                false
            }
        })
        .map(|(sprite, sprite_transform, entity)| (entity, *sprite_transform.translation()))
        .collect()
}

fn filter_by_card_state<'a>(
    entities: Vec<(Entity, Vector3<f32>)>,
    cards: ReadStorage<'a, CardComponent>,
) -> Vec<(Entity, Vector3<f32>)> {
    entities
        .into_iter()
        .filter(|(entity, translation)| match cards.get(*entity) {
            Some(card) => match card.state {
                CardState::TableauCovered => false,
                _ => true,
            },
            None => false,
        })
        .collect()
}

fn get_min(cards: Vec<(Entity, Vector3<f32>)>) -> Option<(Entity, Vector3<f32>)> {
    cards
        .into_iter()
        .min_by(|(entity1, transform1), (entity2, transform2)| {
            transform1.z.partial_cmp(&transform2.z).unwrap()
        })
}

impl<'a> System<'a> for StackSystem {
    type SystemData = (
        Read<'a, AssetStorage<SpriteSheet>>,
        Read<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, ScreenDimensions>,
        ReadStorage<'a, CardComponent>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, SpriteRender>,
        WriteStorage<'a, FoundationComponent>,
        WriteStorage<'a, StackComponent>,
        WriteStorage<'a, TableauComponent>,
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
            mut foundations,
            mut stacks,
            mut tableaus,
            mut transformations,
            entities,
        ): Self::SystemData,
    ) {
        if let Some(cursor) = screen_to_world(&cameras, &input, &dimension, &transformations) {
            self.point = match self.point {
                Some(prev_point) => {
                    if input.mouse_button_is_down(MouseButton::Left) {
                        let offset = Vector2::new(cursor.x - prev_point.x, cursor.y - prev_point.y);
                        for (sprite_transform, _stack) in (&mut transformations, &stacks).join() {
                            sprite_transform
                                .prepend_translation(Vector3::new(offset.x, offset.y, 0.));
                        }
                        Some(Vector2::new(cursor.x, cursor.y))
                    } else {
                        // Check if stack was created
                        if let Some((stack, stack_entity)) = (&stacks, &entities).join().next() {
                            // Check if the card can be moved
                            let canmove = if let Some((drop_entity, drop_translation)) = get_min(
                                get_under_cursor(
                                    cursor,
                                    sheet_storage,
                                    sprites,
                                    &transformations,
                                    entities,
                                )
                                .into_iter()
                                .filter(|(entity, translation)| *entity != stack_entity)
                                .collect(),
                            ) {
                                if let Some(tableau) = tableaus.get_mut(drop_entity) {
                                    info!("found a tableau");
                                    false
                                } else if let Some(foundation) = foundations.get_mut(drop_entity) {
                                    info!("found foundation");
                                    false
                                } else {
                                    info!("invalid target {:?}", drop_entity);
                                    false
                                }
                            } else {
                                false
                            };
                            let sprite_transform = transformations
                                .get_mut(stack_entity)
                                .expect("There should be a transform");
                            if canmove {
                            } else {
                                // Move card back to original position
                                sprite_transform.set_translation_xyz(
                                    stack.original.x,
                                    stack.original.y,
                                    stack.original.z,
                                );
                            }
                            // Remove stack component from the card
                            stacks.remove(stack_entity);
                        }
                        None
                    }
                }
                None => {
                    if input.mouse_button_is_down(MouseButton::Left) {
                        let selected = get_min(filter_by_card_state(
                            get_under_cursor(
                                cursor,
                                sheet_storage,
                                sprites,
                                &transformations,
                                entities,
                            ),
                            cards,
                        ));
                        if let Some((entity, point)) = selected {
                            stacks.insert(entity, StackComponent::new(point));
                        }
                        Some(Vector2::new(cursor.x, cursor.y))
                    } else {
                        None
                    }
                }
            }
        }
    }
}
