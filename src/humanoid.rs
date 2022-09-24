use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxesMask};

use crate::ground::STATIC_GROUP;

pub const HUMANOID_TRAINING_GROUP: u32 = 0b001;

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
impl BodySize {
    pub fn body() -> Self {
        Self {
            hw: 0.2,
            hh: 0.3,
            hl: 0.1,
        }
    }
    pub fn femur() -> Self {
        Self {
            hw: 0.05,
            hh: 0.2,
            hl: 0.05,
        }
    }
    // pub fn tibia() -> Self {
    //     Self {
    //         hw: 0.04,
    //         hh: 0.2,
    //         hl: 0.04,
    //     }
    // }
}

trait DetaHumanoidBodyPartPbrBundleils {
    fn from_halfsize(
        hs: &BodySize,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self;
}
impl DetaHumanoidBodyPartPbrBundleils for PbrBundle {
    fn from_halfsize(
        hs: &BodySize,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: hs.hw,
                min_x: -hs.hw,
                max_y: hs.hh,
                min_y: -hs.hh,
                max_z: hs.hl,
                min_z: -hs.hl,
            })),
            material: materials.add(Color::rgba(0.3, 0.2, 0.2, 0.5).into()),
            ..default()
        }
    }
}

pub fn spawn_humanoid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
) -> Entity {
    let body_size = BodySize::body();
    let femur_size = BodySize::femur();
    // let tibia_size = BodySize::tibia();

    let mut femur_entities: Vec<Entity> = vec![];
    femur_entities.push(
        commands
            .spawn()
            .insert(Name::new("femur"))
            .insert(Sleeping::disabled())
            .insert_bundle(PbrBundle::from_halfsize(&femur_size, meshes, materials))
            .insert(RigidBody::Dynamic)
            .insert(Ccd::enabled())
            .insert(Velocity::zero())
            .insert(ExternalForce::default())
            .insert_bundle(TransformBundle::from(transform))
            .insert(ReadMassProperties::default())
            .with_children(|children| {
                let femur_border_radius = 0.02;
                children
                    .spawn()
                    .insert(Name::new("femur_collider"))
                    .insert_bundle(TransformBundle::from(Transform::identity()))
                    .insert(Collider::round_cuboid(
                        femur_size.hw - femur_border_radius,
                        femur_size.hh - femur_border_radius,
                        femur_size.hl - femur_border_radius,
                        femur_border_radius,
                    ))
                    .insert(ColliderScale::Absolute(Vec3::ONE))
                    .insert(Friction::coefficient(0.5))
                    .insert(Restitution::coefficient(0.))
                    .insert(CollisionGroups::new(HUMANOID_TRAINING_GROUP, STATIC_GROUP))
                    .insert(CollidingEntities::default())
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(ContactForceEventThreshold(0.1))
                    .insert(ColliderMassProperties::MassProperties(MassProperties {
                        mass: 5.0,
                        principal_inertia: Vec3::new(0.5, 0.2, 0.5),
                        ..default()
                    }));
            })
            .id(),
    );

    let humanoid_id = commands
        .spawn()
        .insert(Name::new("humanoid"))
        .insert(Sleeping::disabled())
        .insert_bundle(PbrBundle::from_halfsize(&body_size, meshes, materials))
        .insert(RigidBody::Dynamic)
        .insert(Ccd::enabled())
        .insert(Damping {
            linear_damping: 0.05,
            angular_damping: 0.05,
        })
        .insert(Velocity::zero())
        .insert(ExternalForce::default())
        .insert_bundle(TransformBundle::from(transform))
        .insert(ReadMassProperties::default())
        .with_children(|children| {
            let body_border_radius = 0.05;
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
                .insert(CollisionGroups::new(HUMANOID_TRAINING_GROUP, STATIC_GROUP))
                .insert(CollidingEntities::default())
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ContactForceEventThreshold(0.1))
                .insert(ColliderMassProperties::MassProperties(MassProperties {
                    mass: 20.0,
                    principal_inertia: Vec3::new(5., 2., 5.),
                    ..default()
                }));
        })
        .id();

    let body_femur_joint_mask = JointAxesMask::LOCKED_FIXED_AXES;
    // JointAxesMask::X
    // | JointAxesMask::Y // vertical suspension
    // | JointAxesMask::Z // tire suspension along car
    // | JointAxesMask::ANG_X // wheel main axis
    // JointAxesMask::ANG_Y
    // | JointAxesMask::ANG_Z;

    for femur_id in femur_entities.iter() {
        commands.entity(*femur_id).insert(ImpulseJoint::new(
            humanoid_id,
            GenericJointBuilder::new(body_femur_joint_mask)
                .local_axis1(Vec3::Y)
                .local_axis2(Vec3::Y)
                .local_anchor1(Vec3::new(0., -body_size.hh, 0.))
                .local_anchor2(Vec3::new(0., femur_size.hh, 0.))
                .build(),
        ));
    }

    return humanoid_id;
}
