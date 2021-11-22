use arara::prelude::*;
use cgmath::Deg;

fn main() {
    logger::init();

    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(CoordinateSystemPlugin)
        // .add_plugin(EntityCountDiagnosticPlugin)
        // .add_plugin(LogDiagnosticPlugin { wait_duration: Duration::from_secs(1) })
        .add_startup_system(add_shapes.system())
        .insert_resource(BPLight {
            position: vec3(-2.0, 5.0, 3.0),
        })
        .insert_resource(FlyCamera::from_camera(
            Camera::new((0.0, 2.0, 10.0), Deg(-90.0), Deg(0.0)),
            1.0,
            0.5,
        ))
        .build()
        .run()
}

struct Robot;
struct LowerLeftLeg;
struct LowerRightLeg;
struct UpperLeftLeg;
struct UpperRightLeg;
struct Body;
struct LeftArm;
struct RightArm;

fn add_shapes(mut commands: Commands) {
    // ------------- Floor ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 0.1, 4., 4.)),
        transform: Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        color: Color::BLACK,
        ..Default::default()
    });

    // ------------- Foot ------------------

    commands
        .spawn()
        .insert(Robot)
        .insert_bundle(TransformBundle {
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        })
        .with_children(|robot| {
            robot
                .spawn()
                .insert(LowerLeftLeg)
                .insert_bundle(TransformBundle::default())
                .with_children(|parent| {
                    parent.spawn_bundle(SimpleMeshBundle {
                        mesh: Box::new(Cuboid::new(0.5, 0.3, 1.)),
                        transform: Transform::from_xyz(-1., 0.2, 0.),
                        color: Color::SILVER,
                        ..Default::default()
                    });
                    parent.spawn_bundle(SimpleMeshBundle {
                        mesh: Box::new(Cuboid::new(0.5, 2., 0.5)),
                        transform: Transform::from_xyz(-1., 1., -0.25),
                        color: Color::SILVER,
                        ..Default::default()
                    });
                });
            robot.spawn()
                .insert()
        })
        .with_children(|robot| {
            parent.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Cuboid::new(0.5, 0.3, 1.)),
                transform: Transform::from_xyz(1., 0.2, 0.),
                color: Color::SILVER,
                ..Default::default()
            });
            parent.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Cuboid::new(0.5, 2., 0.5)),
                transform: Transform::from_xyz(1., 1., -0.25),
                color: Color::SILVER,
                ..Default::default()
            });
        });

    // ------------- Legs ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16, 0.45)),
        transform: Transform::from_xyz(1., 2.1, -0.25),
        color: Color::DARK_GRAY,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16, 0.45)),
        transform: Transform::from_xyz(-1., 2.1, -0.25),
        color: Color::DARK_GRAY,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5, 1.8, 0.5)),
        transform: Transform::from_xyz(-1., 3., -0.25),
        color: Color::SILVER,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5, 1.8, 0.5)),
        transform: Transform::from_xyz(1., 3., -0.25),
        color: Color::SILVER,
        ..Default::default()
    });

    // ------------- Body ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(2.55, 3., 1.)),
        transform: Transform::from_xyz(0., 5.4, -0.25),
        color: Color::SILVER,
        ..Default::default()
    });

    // // ------------- Arms ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5, 2., 0.5)),
        transform: Transform::from_xyz(1.6, 5.6, -0.25),
        color: Color::SILVER,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16, 0.45)),
        transform: Transform::from_xyz(1.6, 6.6, -0.25),
        color: Color::DARK_GRAY,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5, 2., 0.5)),
        transform: Transform::from_xyz(-1.6, 5.6, -0.25),
        color: Color::SILVER,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16, 0.45)),
        transform: Transform::from_xyz(-1.6, 6.6, -0.25),
        color: Color::DARK_GRAY,
        ..Default::default()
    });
}
