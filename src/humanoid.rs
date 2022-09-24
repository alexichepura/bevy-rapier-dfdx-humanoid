use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn humanoid_start_system(mut commands: Commands) {
    spawn_humanoid(&mut commands, Transform::from_xyz(0., 1., 0.));
}

#[derive(Debug, Clone)]
pub struct BodySize {
    pub hw: f32,
    pub hh: f32,
    pub hl: f32,
}

pub fn spawn_humanoid(commands: &mut Commands, transform: Transform) -> Entity {
    let body_size = BodySize {
        hw: 0.2,
        hh: 0.3,
        hl: 0.1,
    };

    let humanoid_id = commands
        .spawn()
        .insert(Name::new("humanoid"))
        .insert(Sleeping::disabled())
        // .insert_bundle(PbrBundle {
        //     mesh: meshes.add(bevy_mesh(Cylinder::new(wheel_hw, wheel_r).to_trimesh(50))),
        //     material: materials.add(Color::rgba(0.1, 0.1, 0.1, 0.7).into()),
        //     ..default()
        // })
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
                .insert(ColliderMassProperties::Density(1.0));
        })
        .id();
    return humanoid_id;
}
