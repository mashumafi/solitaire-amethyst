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
    components::{CardComponent, CardState, PileComponent, StackComponent},
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
) -> Vec<(Entity, Vector3<f32>, f32)> {
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
        .map(|(sprite, sprite_transform, entity)| {
            (
                entity,
                *sprite_transform.translation(),
                (*sprite_transform.global_matrix() * Point4::new(0., 0., 0., 1.)).z,
            )
        })
        .collect()
}

fn filter_by_card_state<'a>(
    entities: Vec<(Entity, Vector3<f32>, f32)>,
    cards: ReadStorage<'a, CardComponent>,
) -> Vec<(Entity, Vector3<f32>, f32)> {
    entities
        .into_iter()
        .filter(|(entity, translation, z)| match cards.get(*entity) {
            Some(card) => match card.state {
                CardState::TableauCovered => false,
                _ => true,
            },
            None => false,
        })
        .collect()
}

fn get_max(cards: Vec<(Entity, Vector3<f32>, f32)>) -> Option<(Entity, Vector3<f32>, f32)> {
    cards
        .into_iter()
        .max_by(|(entity1, transform1, z1), (entity2, transform2, z2)| z1.partial_cmp(&z2).unwrap())
}

impl<'a> System<'a> for StackSystem {
    type SystemData = (
        Read<'a, AssetStorage<SpriteSheet>>,
        Read<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, ScreenDimensions>,
        ReadStorage<'a, CardComponent>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, SpriteRender>,
        WriteStorage<'a, PileComponent>,
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
            mut piles,
            mut stacks,
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
                            let canmove = if let Some((drop_entity, drop_translation, z)) = get_max(
                                get_under_cursor(
                                    cursor,
                                    sheet_storage,
                                    sprites,
                                    &transformations,
                                    entities,
                                )
                                .into_iter()
                                .filter(|(entity, translation, z)| *entity != stack_entity)
                                .collect(),
                            ) {
                                if let Some(drop_pile) = piles.get_mut(drop_entity) {
                                    match drop_pile {
                                        PileComponent::Foundation(drop_foundation) => {
                                            info!("found foundation");
                                            // Check if it is just a single card
                                            if let Some(stack_pile) = piles.get_mut(drop_entity) {
                                                match stack_pile {
                                                    PileComponent::Waste(stack_waste) => {
                                                        // Is it a card
                                                        //   Is this card valid?
                                                        //     Push the old card into the stack
                                                        //     Change the old card to this card
                                                        // No card
                                                        //   Is it valid? (an ACE?)
                                                        //     Move this card (change parent)
                                                        //     Move the old foundation to this entity
                                                    }
                                                    PileComponent::Tableau => {}
                                                    PileComponent::Foundation(stack_foundation) => {
                                                    }
                                                }
                                            }
                                            false
                                        }
                                        PileComponent::Tableau => {
                                            info!("found a tableau");
                                            // Is it valid?
                                            //   Remove tableau/waste/foundation (from the drop entity)
                                            //   Move the card (change parent)
                                            false
                                        }
                                        _ => false,
                                    }
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
                                // Is stack a tableau
                                //   Add tableau (to the stack entity parent)
                                // Is stack a waste
                                //   If waste has cards
                                //     Generate new entity for the card
                                // Is stack a foundation
                                //   If foundation has cards
                                //     Generate new entity for card
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
                        let selected = get_max(filter_by_card_state(
                            get_under_cursor(
                                cursor,
                                sheet_storage,
                                sprites,
                                &transformations,
                                entities,
                            ),
                            cards,
                        ));
                        if let Some((entity, point, z)) = selected {
                            transformations
                                .get_mut(entity)
                                .expect("We just looked at this")
                                .translation_mut()
                                .z = 13.;
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
