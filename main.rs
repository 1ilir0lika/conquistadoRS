pub mod oggetti;
pub mod functions;
mod traits;
use crate::oggetti::mappa::mappa::Mappa;
use crate::oggetti::campo::campo::Campo;  
use crate::oggetti::punto::punto::Punto;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
const COLORE_MAPPA: Color = Color::rgb(0.7, 0.7, 0.7);
fn main() {
    //per forza numero pari di punti
    let mut user1:Campo=Campo(vec![Punto{x:1,y:1},Punto{x:4,y:1},Punto{x:2,y:4},Punto{x:4,y:5},Punto{x:2,y:2},Punto{x:1,y:2},Punto{x:1,y:4},Punto{x:1,y:5}]);
    println!("{:?}",user1.area());
    //App::new()
     //   .add_plugins(DefaultPlugins)
     //   .add_startup_system(setup)
     //   .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(COLORE_MAPPA)),
        ..default()
    });
}