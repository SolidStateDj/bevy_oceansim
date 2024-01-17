mod camera;
mod oceanmath;

use bevy::{prelude::*, render::render_resource::{ShaderRef, AsBindGroup}};
use camera::PanOrbitCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(AmbientLight{color: Color::rgb(1.0, 1.0, 1.0), brightness: 1.0})
        .add_systems(Update, (update_time_for_materials))
        .run();
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    time: f32,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ocean.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<CustomMaterial>>,) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane{size: 5.0, subdivisions: 10})),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(CustomMaterial {
            time: 0.,
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });
}

fn update_time_for_materials(mut materials: ResMut<Assets<CustomMaterial>>, time: Res<Time>) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds() as f32;
    }
}
 


