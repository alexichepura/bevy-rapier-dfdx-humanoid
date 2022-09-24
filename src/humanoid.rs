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
    pub fn tibia() -> Self {
        Self {
            hw: 0.04,
            hh: 0.2,
            hl: 0.04,
        }
    }
    pub fn foot() -> Self {
        Self {
            hw: 0.04,
            hh: 0.02,
            hl: 0.1,
        }
    }
}

trait BodyPartPbrBundle {
    fn from_halfsize(
        hs: &BodySize,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        color: &Color,
    ) -> Self;
}
impl BodyPartPbrBundle for PbrBundle {
    fn from_halfsize(
        hs: &BodySize,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        color: &Color,
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
            material: materials.add((*color).into()),
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
    let tibia_size = BodySize::tibia();
    let foot_size = BodySize::foot();

    let body_id = commands
        .spawn()
        .insert(Name::new("body"))
        .insert(Sleeping::disabled())
        .insert_bundle(PbrBundle::from_halfsize(
            &body_size,
            meshes,
            materials,
            &Color::rgba(0.3, 0.2, 0.2, 0.5),
        ))
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
                .insert(Name::new("body_collider"))
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

    let mut femur_entities: Vec<Entity> = vec![];
    for i in 0..2 {
        let femur_id = commands
            .spawn()
            .insert(Name::new("femur"))
            .insert(Sleeping::disabled())
            .insert_bundle(PbrBundle::from_halfsize(
                &femur_size,
                meshes,
                materials,
                &Color::rgba(0.2, 0.3, 0.2, 0.5),
            ))
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
            .insert(ImpulseJoint::new(
                body_id,
                GenericJointBuilder::new(body_femur_joint_mask)
                    .local_axis1(Vec3::Y)
                    .local_axis2(Vec3::Y)
                    .local_anchor1(Vec3::new(
                        match i {
                            0 => 0.1,
                            _ => -0.1,
                        },
                        -body_size.hh,
                        0.,
                    ))
                    .local_anchor2(Vec3::new(0., femur_size.hh, 0.))
                    .build(),
            ))
            .id();
        femur_entities.push(femur_id);
    }

    let femur_tibia_joint_mask = JointAxesMask::LOCKED_FIXED_AXES;

    let mut tibia_entities: Vec<Entity> = vec![];
    for i in 0..2 {
        let tibia_id = commands
            .spawn()
            .insert(Name::new("tibia"))
            .insert(Sleeping::disabled())
            .insert_bundle(PbrBundle::from_halfsize(
                &tibia_size,
                meshes,
                materials,
                &Color::rgba(0.2, 0.2, 0.3, 0.5),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Ccd::enabled())
            .insert(Velocity::zero())
            .insert(ExternalForce::default())
            .insert_bundle(TransformBundle::from(transform))
            .insert(ReadMassProperties::default())
            .with_children(|children| {
                let tibia_border_radius = 0.02;
                children
                    .spawn()
                    .insert(Name::new("tibia_collider"))
                    .insert_bundle(TransformBundle::from(Transform::identity()))
                    .insert(Collider::round_cuboid(
                        tibia_size.hw - tibia_border_radius,
                        tibia_size.hh - tibia_border_radius,
                        tibia_size.hl - tibia_border_radius,
                        tibia_border_radius,
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
            .insert(ImpulseJoint::new(
                femur_entities[i],
                GenericJointBuilder::new(femur_tibia_joint_mask)
                    .local_axis1(Vec3::Y)
                    .local_axis2(Vec3::Y)
                    .local_anchor1(Vec3::new(0., -femur_size.hh, 0.))
                    .local_anchor2(Vec3::new(0., tibia_size.hh, 0.))
                    .build(),
            ))
            .id();
        tibia_entities.push(tibia_id);
    }

    let tibia_foot_joint_mask = JointAxesMask::LOCKED_FIXED_AXES;
    for i in 0..2 {
        commands
            .spawn()
            .insert(Name::new("foot"))
            .insert(Sleeping::disabled())
            .insert_bundle(PbrBundle::from_halfsize(
                &foot_size,
                meshes,
                materials,
                &Color::rgba(0.2, 0.2, 0.2, 0.5),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Ccd::enabled())
            .insert(Velocity::zero())
            .insert(ExternalForce::default())
            .insert_bundle(TransformBundle::from(transform))
            .insert(ReadMassProperties::default())
            .with_children(|children| {
                let foot_border_radius = 0.01;
                children
                    .spawn()
                    .insert(Name::new("foot_collider"))
                    .insert_bundle(TransformBundle::from(Transform::identity()))
                    .insert(Collider::round_cuboid(
                        foot_size.hw - foot_border_radius,
                        foot_size.hh - foot_border_radius,
                        foot_size.hl - foot_border_radius,
                        foot_border_radius,
                    ))
                    .insert(ColliderScale::Absolute(Vec3::ONE))
                    .insert(Friction::coefficient(0.5))
                    .insert(Restitution::coefficient(0.))
                    .insert(CollisionGroups::new(HUMANOID_TRAINING_GROUP, STATIC_GROUP))
                    .insert(CollidingEntities::default())
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(ContactForceEventThreshold(0.1))
                    .insert(ColliderMassProperties::MassProperties(MassProperties {
                        mass: 1.0,
                        principal_inertia: Vec3::new(0.1, 0.1, 0.1),
                        ..default()
                    }));
            })
            .insert(ImpulseJoint::new(
                tibia_entities[i],
                GenericJointBuilder::new(tibia_foot_joint_mask)
                    .local_axis1(Vec3::Y)
                    .local_axis2(Vec3::Y)
                    .local_anchor1(Vec3::new(0., -tibia_size.hh, 0.))
                    .local_anchor2(Vec3::new(0., foot_size.hh, foot_size.hl - tibia_size.hl))
                    .build(),
            ));
    }

    return body_id;
}
