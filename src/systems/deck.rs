use amethyst::{
    assets::AssetStorage,
    core::{
        math::{Point4, Translation3, UnitQuaternion, Vector2, Vector3},
        transform::Transform,
        Parent,
    },
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions,
    winit::MouseButton,
};

use log::info;

use crate::{
    components::{
        builder::{build_camera, build_deck, build_waste},
        CardComponent, CardState, DeckComponent, PileComponent, StackComponent,
    },
    math::{screen_to_world, Rectangle2},
    resources::CardResource,
};

pub struct DeckSystem {
    is_down: bool,
    is_pressed: bool,
}

impl Default for DeckSystem {
    fn default() -> Self {
        DeckSystem {
            is_down: false,
            is_pressed: false,
        }
    }
}

impl<'a> System<'a> for DeckSystem {
    type SystemData = (
        Read<'a, AssetStorage<SpriteSheet>>,
        Read<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, ScreenDimensions>,
        WriteExpect<'a, CardResource>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, StackComponent>,
        WriteStorage<'a, CardComponent>,
        WriteStorage<'a, DeckComponent>,
        WriteStorage<'a, Parent>,
        WriteStorage<'a, PileComponent>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            sheet_storage,
            input,
            dimensions,
            mut card_resource,
            cameras,
            mut boomerangs,
            mut cards,
            mut decks,
            mut parents,
            mut piles,
            mut sprites,
            mut transforms,
            entities,
        ): Self::SystemData,
    ) {
        let is_down = input.mouse_button_is_down(MouseButton::Left);
        if is_down == self.is_down {
            return;
        }
        self.is_down = is_down;

        if let Some(point) = screen_to_world(&cameras, &input, &dimensions, &transforms) {
            if let Some((deck, deck_sprite, deck_transform)) =
                (&mut decks, &sprites, &transforms).join().next()
            {
                if let Some(sprite_sheet) = sheet_storage.get(&deck_sprite.sprite_sheet) {
                    let sprite = &sprite_sheet.sprites[deck_sprite.sprite_number];
                    let texture_rect =
                        Rectangle2::new(-sprite.width, -sprite.height, sprite.width, sprite.height)
                            / 2.;
                    if (texture_rect * *deck_transform.global_matrix()).contains(point.x, point.y) {
                        if self.is_pressed && !is_down {
                            self.is_pressed = false;
                            match deck.cards_mut().draw() {
                                Some(deck_card) => {
                                    info!("drew a card");
                                    for (pile, waste_entity) in (&mut piles, &entities).join() {
                                        match pile {
                                            PileComponent::Waste(waste) => {
                                                if let Some((card, parent, card_entity)) = (
                                                    &mut cards, &parents, &entities,
                                                )
                                                    .join()
                                                    .filter(
                                                        |(card_component, parent, card_entity)| {
                                                            parent.entity == waste_entity
                                                        },
                                                    )
                                                    .next()
                                                {
                                                    info!("updating card");
                                                    // update waste
                                                    waste.insert(card.card);
                                                    sprites.insert(
                                                        waste_entity,
                                                        card_resource.face(card.card),
                                                    );
                                                    // update card on top of waste
                                                    card.card = deck_card;
                                                } else {
                                                    info!("creating card!");
                                                    // no card found, create it
                                                    let card_entity = entities.create();
                                                    parents.insert(
                                                        card_entity,
                                                        Parent {
                                                            entity: waste_entity,
                                                        },
                                                    );
                                                    cards.insert(
                                                        card_entity,
                                                        CardComponent::new(
                                                            deck_card,
                                                            CardState::Waste,
                                                        ),
                                                    );
                                                    let mut transform = Transform::default();
                                                    transform.append_translation(Vector3::new(
                                                        0., 0., 1.,
                                                    ));
                                                    transforms.insert(card_entity, transform);
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                None => {
                                    info!("deck empty");
                                    for pile in (&mut piles).join() {
                                        match pile {
                                            PileComponent::Waste(waste) => {
                                                waste.swap(deck.cards_mut())
                                            }
                                            _ => {}
                                        };
                                    }
                                }
                            }
                        } else {
                            self.is_pressed = true;
                        }
                    } else {
                        self.is_pressed = false;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Deck;
    use amethyst::{
        assets::{AssetStorage, Handle, Loader, Processor},
        core::{shrev::EventChannel, transform::TransformBundle},
        error::Error,
        input::{Button, InputBundle, InputEvent, StringBindings, VirtualKeyCode},
        renderer::{
            plugins::RenderFlat2D, types::DefaultBackend, RenderingBundle, SpriteSheet, Texture,
        },
        winit::{
            dpi::LogicalPosition, DeviceEvent, DeviceId, ElementState, Event, KeyboardInput,
            ModifiersState, MouseButton, MouseScrollDelta, ScanCode, WindowEvent, WindowId,
        },
        LogLevelFilter, LoggerConfig,
    };
    use amethyst_test::{prelude::*, GameUpdate};
    const HIDPI: f64 = 1.0;

    #[derive(Debug)]
    struct WaitForLoad;
    impl<T, E> State<T, E> for WaitForLoad
    where
        T: GameUpdate,
        E: Send + Sync + 'static,
    {
        fn update(&mut self, data: StateData<'_, T>) -> Trans<T, E> {
            data.data.update(&data.world);

            let assets = &data.world.read_resource::<CardResource>();
            assets.print();
            if assets.is_complete() {
                Trans::Pop
            } else {
                Trans::None
            }
        }
    }

    #[test]
    fn test_draw_system_down() -> Result<(), Error> {
        let mut config = LoggerConfig::default();
        config.level_filter = LogLevelFilter::Warn;
        amethyst::start_logger(config);

        AmethystApplication::ui_base::<amethyst::input::StringBindings>()
            .with_setup(|world| {
                world.register::<DeckComponent>();
                world.register::<PileComponent>();
                let dimensions = world.read_resource::<ScreenDimensions>().clone();
                let mut sprites = CardResource::new(world);
                {
                    let mut input = world.write_resource::<InputHandler<StringBindings>>();
                    let mut events = EventChannel::<InputEvent<StringBindings>>::new();
                    input.send_event(
                        &mouse_move(LogicalPosition { x: 70., y: 80. }),
                        &mut events,
                        HIDPI as f32,
                    );
                    assert!(input.mouse_position().is_some());
                    assert!(!input.mouse_button_is_down(MouseButton::Left));
                }
                let deck = Deck::default();

                build_camera(world.create_entity(), &dimensions);
                build_waste(world.create_entity(), &mut sprites, &dimensions);
                build_deck(world.create_entity(), &mut sprites, &dimensions, deck);
                world.add_resource(sprites);
            })
            .with_bundle_fn(|| {
                RenderingBundle::<DefaultBackend>::new().with_plugin(RenderFlat2D::default())
            })
            .with_state(|| WaitForLoad)
            .with_system_single(
                DeckSystem {
                    is_down: true,
                    is_pressed: true,
                },
                "deck_system",
                &[],
            )
            .with_assertion(|world| {
                (&world.read_storage::<CardComponent>())
                    .join()
                    .next()
                    .expect("There should be a card in the waste");
            })
            .run()
    }

    fn key_press(scancode: ScanCode, virtual_keycode: VirtualKeyCode) -> Event {
        key_event(scancode, virtual_keycode, ElementState::Pressed)
    }

    fn key_release(scancode: ScanCode, virtual_keycode: VirtualKeyCode) -> Event {
        key_event(scancode, virtual_keycode, ElementState::Released)
    }

    fn key_event(
        scancode: ScanCode,
        virtual_keycode: VirtualKeyCode,
        state: ElementState,
    ) -> Event {
        Event::WindowEvent {
            window_id: unsafe { WindowId::dummy() },
            event: WindowEvent::KeyboardInput {
                device_id: unsafe { DeviceId::dummy() },
                input: KeyboardInput {
                    scancode,
                    state,
                    virtual_keycode: Some(virtual_keycode),
                    modifiers: ModifiersState {
                        shift: false,
                        ctrl: false,
                        alt: false,
                        logo: false,
                    },
                },
            },
        }
    }

    fn mouse_press(button: MouseButton) -> Event {
        mouse_event(button, ElementState::Pressed)
    }

    fn mouse_release(button: MouseButton) -> Event {
        mouse_event(button, ElementState::Released)
    }

    fn mouse_move(position: LogicalPosition) -> Event {
        Event::WindowEvent {
            window_id: unsafe { WindowId::dummy() },
            event: WindowEvent::CursorMoved {
                device_id: unsafe { DeviceId::dummy() },
                position,
                modifiers: ModifiersState {
                    shift: false,
                    ctrl: false,
                    alt: false,
                    logo: false,
                },
            },
        }
    }

    fn mouse_event(button: MouseButton, state: ElementState) -> Event {
        Event::WindowEvent {
            window_id: unsafe { WindowId::dummy() },
            event: WindowEvent::MouseInput {
                device_id: unsafe { DeviceId::dummy() },
                state,
                button,
                modifiers: ModifiersState {
                    shift: false,
                    ctrl: false,
                    alt: false,
                    logo: false,
                },
            },
        }
    }

    fn mouse_wheel(x: f32, y: f32) -> Event {
        Event::DeviceEvent {
            device_id: unsafe { DeviceId::dummy() },
            event: DeviceEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(x, y),
            },
        }
    }
}
