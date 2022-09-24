use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const STATIC_GROUP: u32 = 0b010;

pub fn ground_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let multiplier: usize = 10;
    let scale = 28. / multiplier as f32;
    let num_cols: usize = 2 * multiplier;
    let num_rows: usize = 3 * multiplier;
    let hx = num_cols as f32 * scale;
    let hy = 0.5;
    let hz = num_rows as f32 * scale;
    let ground_size: Vec3 = 2. * Vec3::new(hx, hy, hz);
    let heights: Vec<Real> = vec![hy; num_rows * num_cols];
    commands
        .spawn()
        .insert(Name::new("ground-heightfield"))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: hx,
                min_x: -hx,
                max_y: hy,
                min_y: -hy,
                max_z: hz,
                min_z: -hz,
            })),
            material: materials.add(Color::rgb(0.2, 0.5, 0.2).into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert_bundle(TransformBundle::default())
        .insert(Collider::heightfield(
            heights,
            num_rows,
            num_cols,
            ground_size.into(),
        ))
        .insert(ColliderScale::Absolute(Vec3::ONE))
        .insert(CollisionGroups::new(STATIC_GROUP, u32::MAX))
        .insert(Friction::coefficient(1.))
        .insert(Restitution::coefficient(0.));
}
