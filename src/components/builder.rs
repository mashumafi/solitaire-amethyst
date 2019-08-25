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
        BoomerangComponent, CardComponent, CardState, DeckComponent, DragComponent, WasteComponent,
    },
    resources::CardResource,
};

pub fn build_camera(mut builder: EntityBuilder, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    builder
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

pub fn build_tableau(
    world: &mut World,
    dimensions: &ScreenDimensions,
    cards: &[Card],
    sprites: &mut CardResource,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        70. + (cards.iter().count() as f32) * 105.,
        dimensions.height() - 176.,
        -20.,
    );
    let tableau = world
        .create_entity()
        .with(sprites.empty())
        .with(transform)
        .build();
    let mut parent = tableau;
    for card in cards.iter() {
        parent = build_card(
            world.create_entity(),
            *card,
            CardState::Tableau,
            Vector3::new(0., -32., 1.),
            sprites,
            Some(parent),
        );
    }
    tableau
}

pub fn build_card(
    mut builder: EntityBuilder,
    card: Card,
    state: CardState,
    offset: Vector3<f32>,
    sprites: &mut CardResource,
    parent: Option<Entity>,
) -> Entity {
    let mut transform = Transform::default();
    transform.append_translation(offset);
    builder = builder
        .with(sprites.face(card))
        .with(transform)
        .with(DragComponent::default())
        .with(CardComponent::new(card, state))
        .with(BoomerangComponent::default());
    if let Some(parent) = parent {
        builder = builder.with(Parent { entity: parent });
    }
    builder.build()
}

pub fn build_deck(
    builder: EntityBuilder,
    sprites: &mut CardResource,
    dimensions: &ScreenDimensions,
    deck: Deck,
) {
    let mut transform = Transform::default();
    transform.append_translation(Vector3::new(70., dimensions.height() - 80., -20.));
    builder
        .with(sprites.back())
        .with(transform)
        .with(DeckComponent::new(deck))
        .build();
}

pub fn build_waste(
    builder: EntityBuilder,
    sprites: &mut CardResource,
    dimensions: &ScreenDimensions,
) {
    let mut transform = Transform::default();
    transform.append_translation(Vector3::new(175., dimensions.height() - 80., -20.));
    builder
        .with(sprites.empty())
        .with(transform)
        .with(WasteComponent::default())
        .build();
}
