use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn humanoid_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_humanoid(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(0., 1., 0.),
    );
}

#[derive(Debug, Clone)]
pub struct BodySize {
    pub hw: f32,
    pub hh: f32,
    pub hl: f32,
}

pub fn spawn_humanoid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
) -> Entity {
    let body_size = BodySize {
        hw: 0.2,
        hh: 0.3,
        hl: 0.1,
    };

    let humanoid_id = commands
        .spawn()
        .insert(Name::new("humanoid"))
        .insert(Sleeping::disabled())
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: body_size.hw,
                min_x: -body_size.hw,
                max_y: body_size.hh,
                min_y: -body_size.hh,
                max_z: body_size.hl,
                min_z: -body_size.hl,
            })),
            material: materials.add(Color::rgba(0.3, 0.2, 0.2, 0.5).into()),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Ccd::enabled())
        // .insert(Damping {
        //     linear_damping: 0.05,
        //     angular_damping: 0.05,
        // })
        .insert(Velocity::zero())
        .insert(ExternalForce::default())
        .insert_bundle(TransformBundle::from(transform))
        .insert(ReadMassProperties::default())
        .with_children(|children| {
            let body_border_radius = 0.1;
            children
                .spawn()
                .insert(Name::new("car_collider"))
                .insert_bundle(TransformBundle::from(Transform::identity()))
                .insert(Collider::round_cuboid(
                    body_size.hw - body_border_radius,
                    body_size.hh - body_border_radius,
                    body_size.hl - body_border_radius,
                    body_border_radius,
                ))
                .insert(ColliderScale::Absolute(Vec3::ONE))
                .insert(Friction::coefficient(0.5))
                .insert(Restitution::coefficient(0.))
                // .insert(CollisionGroups::new(HUMANOID_TRAINING_GROUP, STATIC_GROUP))
                .insert(CollidingEntities::default())
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ContactForceEventThreshold(0.1))
                .insert(ColliderMassProperties::MassProperties(MassProperties {
                    mass: 20.0,
                    principal_inertia: Vec3::new(50., 20., 50.),
                    ..default()
                }));
            // .insert(ColliderMassProperties::Density(1.0));
        })
        .id();
    return humanoid_id;
}
