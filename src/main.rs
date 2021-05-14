use bevy::prelude::*;
use std::f32::consts::PI;


enum PieceColor {
    White,
    Black
}

enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

#[derive(Copy, Clone)]
struct BoardPosition {
    x: u8,
    y: u8
}

struct Meshes {
    king: Handle<Mesh>,
    queen: Handle<Mesh>,
    rook: Handle<Mesh>,
    bishop: Handle<Mesh>,
    knight: Handle<Mesh>,
    pawn: Handle<Mesh>
}

struct Materials {
    white_material: Handle<StandardMaterial>,
    black_material: Handle<StandardMaterial>
}

fn board_to_global(position: BoardPosition) -> Vec3 {
    return Vec3::new(position.x as f32 - 3.5,
                     0.0,
                     -(position.y as f32 - 3.5));
}

fn piece_spawner(commands: &mut Commands, materials: Res<Materials>, meshes: Res<Meshes>) {
    for i in 0..8 {
        spawn_piece(commands, &materials, &meshes,
                    PieceType::Pawn, PieceColor::White,
                    BoardPosition { x: i, y: 1 });
        spawn_piece(commands, &materials, &meshes,
                    PieceType::Pawn, PieceColor::Black,
                    BoardPosition { x: i, y: 6 });
    }

    spawn_piece(commands, &materials, &meshes,
                PieceType::Rook, PieceColor::White,
                BoardPosition {x: 0, y: 0});
    spawn_piece(commands, &materials, &meshes,
                PieceType::Rook, PieceColor::White,
                BoardPosition {x: 7, y: 0});
    spawn_piece(commands, &materials, &meshes,
                PieceType::Rook, PieceColor::Black,
                BoardPosition {x: 0, y: 7});
    spawn_piece(commands, &materials, &meshes,
                PieceType::Rook, PieceColor::Black,
                BoardPosition {x: 7, y: 7});

    spawn_piece(commands, &materials, &meshes,
                PieceType::Knight, PieceColor::White,
                BoardPosition {x: 1, y: 0});
    spawn_piece(commands, &materials, &meshes,
                PieceType::Knight, PieceColor::White,
                BoardPosition {x: 6, y: 0});
    spawn_piece(commands, &materials, &meshes,
                PieceType::Knight, PieceColor::Black,
                BoardPosition {x: 1, y: 7});
    spawn_piece(commands, &materials, &meshes,
                PieceType::Knight, PieceColor::Black,
                BoardPosition {x: 6, y: 7});

    spawn_piece(commands, &materials, &meshes,
                PieceType::Bishop, PieceColor::White,
                BoardPosition {x: 2, y: 0});
    spawn_piece(commands, &materials, &meshes,
                PieceType::Bishop, PieceColor::White,
                BoardPosition {x: 5, y: 0});
    spawn_piece(commands, &materials, &meshes,
                PieceType::Bishop, PieceColor::Black,
                BoardPosition {x: 2, y: 7});
    spawn_piece(commands, &materials, &meshes,
                PieceType::Bishop, PieceColor::Black,
                BoardPosition {x: 5, y: 7});

    spawn_piece(commands, &materials, &meshes,
                PieceType::Queen, PieceColor::White,
                BoardPosition {x: 3, y: 0});
    spawn_piece(commands, &materials, &meshes,
                PieceType::Queen, PieceColor::Black,
                BoardPosition {x: 3, y: 7});

    spawn_piece(commands, &materials, &meshes,
                PieceType::King, PieceColor::White,
                BoardPosition {x: 4, y: 0});
    spawn_piece(commands, &materials, &meshes,
                PieceType::King, PieceColor::Black,
                BoardPosition {x: 4, y: 7});
}

fn spawn_piece(commands: &mut Commands, materials: &Res<Materials>, meshes: &Res<Meshes>,
               piece_type: PieceType, color: PieceColor, position: BoardPosition) {

    let mesh = match piece_type {
        PieceType::King => meshes.king.clone(),
        PieceType::Queen => meshes.queen.clone(),
        PieceType::Rook => meshes.rook.clone(),
        PieceType::Bishop => meshes.bishop.clone(),
        PieceType::Knight => meshes.knight.clone(),
        PieceType::Pawn => meshes.pawn.clone()
    };

    let material = match color {
        PieceColor::White => materials.white_material.clone(),
        PieceColor::Black => materials.black_material.clone()
    };

    let rotation_rad: f32 = match color {
        PieceColor::White => 0.0,
        PieceColor::Black => PI
    };

    commands.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform {
            translation: board_to_global(position),
            rotation: Quat::from_rotation_y(rotation_rad),
            ..Default::default()
        },
        ..Default::default()})
        .with(piece_type)
        .with(color)
        .with(position);
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>) {

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0 })),
        material: materials.add(
            StandardMaterial {
                albedo_texture: Some(asset_server.load("textures/board.png")),
                ..Default::default()
            }
        ),
        ..Default::default()
    })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 10.0)),
            light: Light {
                depth: (0.0..1000.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .insert_resource(Meshes {
            king: asset_server.load("models/pawns.glb#Mesh4/Primitive0"),
            queen: asset_server.load("models/pawns.glb#Mesh5/Primitive0"),
            rook: asset_server.load("models/pawns.glb#Mesh2/Primitive0"),
            bishop: asset_server.load("models/pawns.glb#Mesh1/Primitive0"),
            knight: asset_server.load("models/pawns.glb#Mesh3/Primitive0"),
            pawn: asset_server.load("models/pawns.glb#Mesh0/Primitive0")
        })
        .insert_resource(Materials {
            white_material: materials.add(StandardMaterial {
                albedo: Color::rgb(1.0, 1.0, 1.0),
                // Bright metal for White
                albedo_texture: Some(asset_server.load("textures/cc0textures.com/Metal024_1K_Color.png")),
                shaded: true,
                ..Default::default()
            }),
            black_material: materials.add(StandardMaterial {
                albedo: Color::rgb(1.0, 1.0, 1.0),
                // Dark rusty texture for Black
                albedo_texture: Some(asset_server.load("textures/cc0textures.com/Rust004_1K_Color.png")),
                shaded: true,
                ..Default::default()
            })
        });
}

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_pieces", SystemStage::single(piece_spawner.system()))
        .run();
}