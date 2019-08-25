use amethyst::{
    assets::{AssetStorage, Loader},
    core::{math::Vector3, transform::Transform, Hidden, Parent},
    ecs::{Entity, EntityBuilder},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use crate::{
    card::{Card, Deck},
    components::{
        builder::{build_camera, build_card, build_deck, build_tableau, build_waste},
        CardComponent, CardState, DeckComponent, DragComponent, StackComponent, WasteComponent,
    },
    resources::CardResource,
};

use log::info;

pub struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = world.read_resource::<ScreenDimensions>().clone();

        build_camera(world.create_entity(), &dimensions);

        let mut sprites = CardResource::new(world);
        init_sprites(world, &mut sprites, &dimensions);
        world.add_resource(sprites);
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }
        }

        Trans::None
    }
}

fn init_sprites(world: &mut World, sprites: &mut CardResource, dimensions: &ScreenDimensions) {
    let mut deck = Deck::default();
    // waste
    build_waste(world.create_entity(), sprites, dimensions);

    // foundation
    for i in 0..4 {
        build_card(
            world.create_entity(),
            deck.draw().unwrap(),
            CardState::Foundation,
            Vector3::new(385. + (i as f32) * 105., dimensions.height() - 80., 0.),
            None,
        );
    }
    // tableau
    for i in 0..7 {
        let cards: Vec<Card> = (0..=i).into_iter().map(|_| deck.draw().unwrap()).collect();
        build_tableau(world, &dimensions, &cards[..], sprites);
    }
    // deck
    build_deck(world.create_entity(), sprites, &dimensions, deck);
}
