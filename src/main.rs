use bevy::prelude::*;
use std::f32::consts::PI;
use bevy_mod_picking::*;
use rand::random;
use crate::GameState::WaitingForSelect;
use bevy::input::gamepad::GamepadButtonType::Select;

struct ChessPiece;

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

struct Textures {
    texture_white: Handle<Texture>,
    texture_black: Handle<Texture>
}

struct SelectedPiece;
struct MovingPiece;

struct SharedData {
    game_state: GameState
}

enum GameState {
    WaitingForSelect,
    PieceSelected,
    PieceMoving
}

fn board_to_global(position: BoardPosition) -> Vec3 {
    return Vec3::new(position.x as f32 - 3.5,
                     0.0,
                     -(position.y as f32 - 3.5));
}

fn piece_spawner(commands: &mut Commands, textures: Res<Textures>,
                 mut materials: ResMut<Assets<StandardMaterial>>, meshes: Res<Meshes>) {
    for i in 0..8 {
        spawn_piece(commands, &textures, &mut materials, &meshes,
                    PieceType::Pawn, PieceColor::White,
                    BoardPosition { x: i, y: 1 });
        spawn_piece(commands, &textures, &mut materials, &meshes,
                    PieceType::Pawn, PieceColor::Black,
                    BoardPosition { x: i, y: 6 });
    }

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::White,
                BoardPosition {x: 0, y: 0});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::White,
                BoardPosition {x: 7, y: 0});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::Black,
                BoardPosition {x: 0, y: 7});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::Black,
                BoardPosition {x: 7, y: 7});

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::White,
                BoardPosition {x: 1, y: 0});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::White,
                BoardPosition {x: 6, y: 0});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::Black,
                BoardPosition {x: 1, y: 7});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::Black,
                BoardPosition {x: 6, y: 7});

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::White,
                BoardPosition {x: 2, y: 0});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::White,
                BoardPosition {x: 5, y: 0});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::Black,
                BoardPosition {x: 2, y: 7});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::Black,
                BoardPosition {x: 5, y: 7});

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Queen, PieceColor::White,
                BoardPosition {x: 3, y: 0});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Queen, PieceColor::Black,
                BoardPosition {x: 3, y: 7});

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::King, PieceColor::White,
                BoardPosition {x: 4, y: 0});
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::King, PieceColor::Black,
                BoardPosition {x: 4, y: 7});
}

fn spawn_piece(commands: &mut Commands, textures: &Res<Textures>,
               materials: &mut ResMut<Assets<StandardMaterial>>, meshes: &Res<Meshes>,
               piece_type: PieceType, color: PieceColor, position: BoardPosition) {

    let mesh = match piece_type {
        PieceType::King => meshes.king.clone(),
        PieceType::Queen => meshes.queen.clone(),
        PieceType::Rook => meshes.rook.clone(),
        PieceType::Bishop => meshes.bishop.clone(),
        PieceType::Knight => meshes.knight.clone(),
        PieceType::Pawn => meshes.pawn.clone()
    };

    let texture = match color {
        PieceColor::White => textures.texture_white.clone(),
        PieceColor::Black => textures.texture_black.clone()
    };

    let rotation_rad: f32 = match color {
        PieceColor::White => 0.0,
        PieceColor::Black => PI
    };

    commands.spawn(PbrBundle {
        mesh,
        material: materials.add(StandardMaterial {
            albedo_texture: Some(texture.clone()),
            ..Default::default()
        }),
        transform: Transform {
            translation: board_to_global(position),
            rotation: Quat::from_rotation_y(rotation_rad),
            ..Default::default()
        },
        ..Default::default()})
        .with(PickableMesh::default())
        .with(InteractableMesh::default())
        // .with(SelectablePickMesh::default())
        .with(piece_type)
        .with(color)
        .with(position)
        .with(ChessPiece);
}

fn piece_raycast_system(
    commands: &mut Commands,
    mut query: Query<(&InteractableMesh, Entity, &Handle<StandardMaterial>), (With<ChessPiece>)>,
    mut query2: Query<(Entity, &Handle<StandardMaterial>, &PieceColor), (With<SelectedPiece>)>,
    textures: Res<Textures>,
    mut materials: ResMut<Assets<StandardMaterial>>, mut shared_data: ResMut<SharedData>) {

    for (interactable, entity, mut material_handle) in &mut query.iter_mut() {
        let mouse_down_event = interactable
            .mouse_down_event(&Group::default(), MouseButton::Left)
            .unwrap();

        if mouse_down_event.is_none() {
            continue;
        }

        if let MouseDownEvents::MouseJustReleased = mouse_down_event {
            if let GameState::PieceSelected = shared_data.game_state {
                for (entity, mut material_handle, piece_color) in query2.iter() {
                    commands.remove_one::<SelectedPiece>(entity);

                    let texture = match piece_color {
                        PieceColor::White => textures.texture_white.clone(),
                        PieceColor::Black => textures.texture_black.clone()
                    };

                    let material = materials.get_mut(material_handle).unwrap();
                    material.albedo = Color::WHITE;
                    material.albedo_texture = Some(texture);
                }

                shared_data.game_state = GameState::WaitingForSelect;
            }

            match shared_data.game_state {
                GameState::PieceSelected | GameState::WaitingForSelect => {
                    let material = materials.get_mut(material_handle).unwrap();

                    material.albedo_texture = None;
                    material.albedo = Color::rgb(0.0, 0.0, 1.0);

                    shared_data.game_state = GameState::PieceSelected;
                    commands.insert(entity, (SelectedPiece, ));
                },
                _ => ()
            }
        }
    }
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
        .with(PickSource::default())
        .insert_resource(Meshes {
            king: asset_server.load("models/pawns.glb#Mesh4/Primitive0"),
            queen: asset_server.load("models/pawns.glb#Mesh5/Primitive0"),
            rook: asset_server.load("models/pawns.glb#Mesh2/Primitive0"),
            bishop: asset_server.load("models/pawns.glb#Mesh1/Primitive0"),
            knight: asset_server.load("models/pawns.glb#Mesh3/Primitive0"),
            pawn: asset_server.load("models/pawns.glb#Mesh0/Primitive0")
        })
        .insert_resource(Textures {
            texture_white: asset_server.load("textures/cc0textures.com/Metal024_1K_Color.png"),
            texture_black: asset_server.load("textures/cc0textures.com/Rust004_1K_Color.png")
        })
        .insert_resource(SharedData {
            game_state: WaitingForSelect
        });
}

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_pieces", SystemStage::single(piece_spawner.system()))
        .add_system(piece_raycast_system.system())
        .run();
}