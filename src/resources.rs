use crate::card::Card;
use amethyst::{
    assets::{AssetStorage, Completion, Handle, Loader, ProgressCounter},
    core::{math::Vector3, transform::Transform, Hidden, Parent},
    ecs::{Entity, EntityBuilder},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use log::info;

pub struct CardResource {
    faces: Handle<SpriteSheet>,
    backs: Handle<SpriteSheet>,
    empty: Handle<SpriteSheet>,
    progress: ProgressCounter,
}

impl CardResource {
    pub fn new(world: &mut World) -> Self {
        let mut progress = ProgressCounter::new();

        let cards_texture = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "sprites/cards.png",
                ImageFormat::default(),
                &mut progress,
                &texture_storage,
            )
        };

        let faces_sheet = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                "sprites/faces.ron",
                SpriteSheetFormat(cards_texture.clone()),
                &mut progress,
                &sheet_storage,
            )
        };

        let backs_sheet = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                "sprites/backs.ron",
                SpriteSheetFormat(cards_texture),
                &mut progress,
                &sheet_storage,
            )
        };

        let empty_texture = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "sprites/empty.png",
                ImageFormat::default(),
                &mut progress,
                &texture_storage,
            )
        };

        let empty_sheet = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                "sprites/empty.ron",
                SpriteSheetFormat(empty_texture),
                &mut progress,
                &sheet_storage,
            )
        };

        CardResource {
            faces: faces_sheet,
            backs: backs_sheet,
            empty: empty_sheet,
            progress,
        }
    }

    pub fn face(&mut self, card: Card) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.faces.clone(),
            sprite_number: card.index(),
        }
    }

    pub fn back(&mut self) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.backs.clone(),
            sprite_number: 1,
        }
    }

    pub fn empty(&mut self) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.empty.clone(),
            sprite_number: 0,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.progress.is_complete()
    }

    pub fn wait(&self) {
        while !self.is_complete() {
            self.print();
        }
    }

    pub fn print(&self) {
        match self.progress.complete() {
            Completion::Complete => info!("Complete :)"),
            Completion::Failed => info!("Failed :("),
            Completion::Loading => info!(
                "Loading ({}/{}/{})",
                self.progress.num_finished(),
                self.progress.num_failed(),
                self.progress.num_assets()
            ),
        }
    }
}
