mod camera;
mod db;
mod db_client;
mod humanoid;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_atmosphere::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier3d::prelude::*;
use camera::*;
use db_client::DbClientResource;
use humanoid::humanoid_start_system;

fn rapier_config_start_system(mut c: ResMut<RapierContext>) {
    c.integration_parameters.max_velocity_iterations = 512;
    c.integration_parameters.max_stabilization_iterations = 512;
    dbg!(c.integration_parameters);
}

const FPS: f32 = 60.;
fn main() {
    App::new()
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: 1. / FPS,
                substeps: 10,
            },
            ..default()
        })
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(FPS as f64),
            ..default()
        })
        .insert_resource(DbClientResource::default())
        // .insert_resource(DqnResource::default())
        .insert_resource(WindowDescriptor {
            title: "humanoid deep learning".to_string(),
            width: 1024.,
            height: 768.,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(CameraConfig::default())
        .insert_resource(AtmosphereSettings { resolution: 1024 })
        .add_plugins(DefaultPlugins)
        .add_plugin(AtmospherePlugin)
        .add_plugin(FramepacePlugin)
        .add_startup_system(camera_start_system)
        .add_startup_system(humanoid_start_system)
        .add_system(camera_controller_system)
        .add_system(camera_switch_system)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            style: DebugRenderStyle {
                rigid_body_axes_length: 0.5,
                subdivisions: 50,
                ..default()
            },
            // | DebugRenderMode::COLLIDER_AABBS
            mode: DebugRenderMode::COLLIDER_SHAPES
                | DebugRenderMode::RIGID_BODY_AXES
                | DebugRenderMode::JOINTS
                | DebugRenderMode::CONTACTS
                | DebugRenderMode::SOLVER_CONTACTS,
            ..default()
        })
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        // .add_startup_system(dqn_exclusive_start_system.exclusive_system())
        .add_startup_system(rapier_config_start_system)
        .run();
}
