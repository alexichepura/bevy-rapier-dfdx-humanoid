use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const STATIC_GROUP: u32 = 0b010;

pub fn ground_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let num_cols: usize = 20;
    let num_rows: usize = 20;
    let hx = 10.;
    let hy = 0.;
    let hz = 10.;
    let heights: Vec<Real> = vec![hy; num_rows * num_cols];
    commands
        .spawn()
        .insert(Name::new("ground"))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: hx,
                min_x: -hx,
                max_y: hy,
                min_y: -hy,
                max_z: hz,
                min_z: -hz,
            })),
            material: materials.add(Color::rgb(0.2, 0.4, 0.15).into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert_bundle(TransformBundle::from_transform(Transform::identity()))
        .insert(Collider::heightfield(
            heights,
            num_rows,
            num_cols,
            (2. * Vec3::new(hx, hy, hz)).into(),
        ))
        .insert(ColliderScale::Absolute(Vec3::ONE))
        .insert(CollisionGroups::new(STATIC_GROUP, u32::MAX))
        .insert(Friction::coefficient(1.))
        .insert(Restitution::coefficient(0.));
}
