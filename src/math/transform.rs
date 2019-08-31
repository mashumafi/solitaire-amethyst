use amethyst::{
    core::{
        math::{Point2, Point3, Vector2},
        transform::Transform,
    },
    ecs::{
        storage::{GenericReadStorage, MaskedStorage},
        Component, Join, Read, ReadExpect, ReadStorage, Storage, SystemData, Tracked, WriteStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::Camera,
    window::ScreenDimensions,
};

use log::info;

use std::ops::Deref;

pub fn screen_to_world<'a, S>(
    cameras: &ReadStorage<'a, Camera>,
    input: &Read<'a, InputHandler<StringBindings>>,
    dimension: &ReadExpect<'a, ScreenDimensions>,
    transformations: &Storage<Transform, S>,
) -> Option<Point3<f32>>
where
    S: Deref<Target = MaskedStorage<Transform>>,
{
    if let Some((x, y)) = input.mouse_position() {
        if let Some((camera, camera_transform)) = (cameras, transformations).join().next() {
            Some(camera.projection().screen_to_world(
                Point2::new(x, y),
                Vector2::new(dimension.width(), dimension.height()),
                &camera_transform,
            ))
        } else {
            None
        }
    } else {
        None
    }
}
