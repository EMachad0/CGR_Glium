use arara::prelude::*;
use arara_particle_system::{self, Value, ParticleSystem, ParticleSystemPlugin};
use cgmath::Deg;

fn main() {
    logger::init();
    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticleSystemPlugin)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(LogDiagnosticPlugin {
            wait_duration: Duration::from_secs(3),
        })
        .add_startup_system(add_shapes.system())
        .insert_resource(BPLight {
            position: vec3(10.0, 10.0, 0.0),
        })
        .insert_resource(FlyCamera::from_camera(
            Camera::new((0.0, 5.0, 5.0), Deg(-90.0), Deg(-30.0)),
            20.0,
            0.5,
        ))
        .build()
        .run()
}

fn add_shapes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // ------------- Particle ------------------

    commands
        .spawn_bundle(SimpleMeshBundle {
            mesh: meshes.add(Mesh::from(Icosphere::new(6, 0.1))),
            transform: Transform::from_xyz(5.0, 5.0, 1.0),
            color: Color::PURPLE,
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 5.0,
            buffer_quantity: 1000,
            spawn_quantity: 50,
            radius: 5.0,
            particle_color: Color::BLUE,
            particle_velocity: Value::Range(2.0, 4.0),
            particle_mesh: meshes.add(Mesh::from(Square::new(0.2, 0.5))),
            timer: Timer::from_seconds( 0.5, true),
            ..Default::default()
        });
}
