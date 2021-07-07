use bevy::prelude::*;
use std::f32::consts::PI;
use bevy_mod_picking::*;
use rand::random;
use crate::GameState::WaitingForSelect;
use bevy::input::gamepad::GamepadButtonType::Select;
use bevy::window::WindowId;
use std::cmp;
use crate::PieceType::Knight;

struct ChessPiece;
struct ChessBoard;

#[derive(Copy, Clone, PartialEq)]
enum PieceColor {
    White,
    Black
}

#[derive(Copy, Clone, PartialEq)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone)]
struct LogicChessPiece {
    piece_color: PieceColor,
    piece_type: PieceType,
}

struct SharedData {
    game_state: GameState,
    cursor_board_pos: BoardPosition,
    current_move: PieceColor,
    board: Vec<Vec<Option<LogicChessPiece>>>
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
                 mut materials: ResMut<Assets<StandardMaterial>>, meshes: Res<Meshes>,
                 mut shared_data: ResMut<SharedData>) {
    for i in 0..8 {
        spawn_piece(commands, &textures, &mut materials, &meshes,
                    PieceType::Pawn, PieceColor::White,
                    BoardPosition { x: i, y: 1 },
                    &mut shared_data.board);
        spawn_piece(commands, &textures, &mut materials, &meshes,
                    PieceType::Pawn, PieceColor::Black,
                    BoardPosition { x: i, y: 6 },
                    &mut shared_data.board);
    }

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::White,
                BoardPosition {x: 0, y: 0},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::White,
                BoardPosition {x: 7, y: 0},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::Black,
                BoardPosition {x: 0, y: 7},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::Black,
                BoardPosition {x: 7, y: 7},
                &mut shared_data.board);

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::White,
                BoardPosition {x: 1, y: 0},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::White,
                BoardPosition {x: 6, y: 0},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::Black,
                BoardPosition {x: 1, y: 7},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::Black,
                BoardPosition {x: 6, y: 7},
                &mut shared_data.board);

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::White,
                BoardPosition {x: 2, y: 0},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::White,
                BoardPosition {x: 5, y: 0},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::Black,
                BoardPosition {x: 2, y: 7},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::Black,
                BoardPosition {x: 5, y: 7},
                &mut shared_data.board);

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Queen, PieceColor::White,
                BoardPosition {x: 3, y: 0},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Queen, PieceColor::Black,
                BoardPosition {x: 3, y: 7},
                &mut shared_data.board);

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::King, PieceColor::White,
                BoardPosition {x: 4, y: 0},
                &mut shared_data.board);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::King, PieceColor::Black,
                BoardPosition {x: 4, y: 7},
                &mut shared_data.board);
}

fn spawn_piece(commands: &mut Commands, textures: &Res<Textures>,
               materials: &mut ResMut<Assets<StandardMaterial>>, meshes: &Res<Meshes>,
               piece_type: PieceType, color: PieceColor, position: BoardPosition,
               board: &mut Vec<Vec<Option<LogicChessPiece>>>) {

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

    board[usize::from(position.y)][usize::from(position.x)] = Some(LogicChessPiece {
        piece_color: color,
        piece_type: piece_type.clone()
    });

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
        .with(piece_type)
        .with(color)
        .with(position)
        .with(ChessPiece);
}

fn piece_raycast_system(
    commands: &mut Commands,
    mut query: Query<(&InteractableMesh, Entity, &Handle<StandardMaterial>, &PieceColor), With<ChessPiece>>,
    mut query2: Query<(Entity, &Handle<StandardMaterial>, &PieceColor), With<SelectedPiece>>,
    textures: Res<Textures>,
    mut materials: ResMut<Assets<StandardMaterial>>, mut shared_data: ResMut<SharedData>) {

    for (interactable, entity, mut material_handle, piece_color) in &mut query.iter_mut() {
        let mouse_down_event = interactable
            .mouse_down_event(&Group::default(), MouseButton::Left)
            .unwrap();

        if mouse_down_event.is_none() || piece_color != &shared_data.current_move {
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

fn board_raycast_system(
    commands: &mut Commands,
    mut query: Query<(&InteractableMesh, Entity, &Handle<StandardMaterial>), With<ChessBoard>>,
    mut query2: Query<(Entity, &mut Transform, &mut BoardPosition, &Handle<StandardMaterial>, &PieceColor, &PieceType), With<SelectedPiece>>,
    // mut query3: Query<(&InteractableMesh, Entity, &Handle<StandardMaterial>, &PieceColor), With<ChessPiece>>,
    mut query3: Query<(&InteractableMesh, &BoardPosition, &PieceColor, Entity), Without<SelectedPiece>>,
    textures: Res<Textures>,
    mut materials: ResMut<Assets<StandardMaterial>>, mut shared_data: ResMut<SharedData>) {

    let mut captured_color = PieceColor::White;
    let mut captured_position = BoardPosition { x: 0, y: 0 };
    let mut should_capture = false;

    if let GameState::PieceSelected = shared_data.game_state {
        let mut flag = false;

        for (interactable, entity, mut material_handle) in &mut query.iter_mut() {
            let mouse_down_event = interactable
                .mouse_down_event(&Group::default(), MouseButton::Left)
                .unwrap();

            if mouse_down_event.is_none() {
                continue;
            }

            if let MouseDownEvents::MouseJustReleased = mouse_down_event {
                shared_data.game_state = GameState::WaitingForSelect;
                commands.insert(entity, (SelectedPiece, ));

                flag = true;
            }
        }

        if flag {
            for (entity, mut transform, mut board_position,
                mut material_handle, piece_color, piece_type) in query2.iter_mut() {
                commands.remove_one::<SelectedPiece>(entity);

                let texture = match piece_color {
                    PieceColor::White => textures.texture_white.clone(),
                    PieceColor::Black => textures.texture_black.clone()
                };

                let material = materials.get_mut(material_handle).unwrap();
                material.albedo = Color::WHITE;
                material.albedo_texture = Some(texture);

                let dest_field = shared_data.board[usize::from(shared_data.cursor_board_pos.y)]
                                                                        [usize::from(shared_data.cursor_board_pos.x)];
                let move_vec = Vec2 { x: shared_data.cursor_board_pos.x as f32 - board_position.x as f32,
                    y: shared_data.cursor_board_pos.y as f32 - board_position.y as f32 };

                let possible = match dest_field {
                    None => {
                        if *piece_type != PieceType::Knight {
                            check_move_pattern(*piece_type, *piece_color, move_vec)
                            && !check_move_blocked(&shared_data.board, (*board_position, shared_data.cursor_board_pos))
                        } else {
                            check_move_pattern(*piece_type, *piece_color, move_vec)
                        }
                    },
                    Some(dest_piece) => {
                        if dest_piece.piece_color == *piece_color {
                            false
                        } else {
                            should_capture = true;

                            check_capture_pattern(*piece_type, *piece_color,
                                      (*board_position, shared_data.cursor_board_pos))
                            && (*piece_type == PieceType::Knight
                            || !check_move_blocked(&shared_data.board, (*board_position, shared_data.cursor_board_pos)))
                        }
                    }
                };

                if !possible {
                    continue;
                }

                let mut board_copy = shared_data.board.clone();
                board_copy[usize::from(board_position.y)][usize::from(board_position.x)] = None;
                board_copy[usize::from(shared_data.cursor_board_pos.y)][usize::from(shared_data.cursor_board_pos.x)] = Some(LogicChessPiece {
                    piece_color: piece_color.clone(),
                    piece_type: piece_type.clone()
                });

                let possible = !check_mate(&board_copy, *piece_color);

                if !possible {
                    continue;
                }

                shared_data.board[usize::from(board_position.y)][usize::from(board_position.x)] = None;

                board_position.x = shared_data.cursor_board_pos.x;
                board_position.y = shared_data.cursor_board_pos.y;

                shared_data.board[usize::from(board_position.y)][usize::from(board_position.x)] = Some(LogicChessPiece {
                    piece_color: piece_color.clone(),
                    piece_type: piece_type.clone()
                });

                transform.translation = board_to_global(shared_data.cursor_board_pos);

                shared_data.current_move = match shared_data.current_move {
                    PieceColor::White => PieceColor::Black,
                    PieceColor::Black => PieceColor::White
                };

                if should_capture {
                    captured_color = piece_color.clone();
                    captured_position = board_position.clone();
                }

                break;
            }

            if should_capture {
                for (mesh, board_position, piece_color, entity) in query3.iter() {
                    if board_position.x == captured_position.x
                        && board_position.y == captured_position.y
                        && *piece_color != captured_color {
                        commands.despawn(entity);
                    }
                }
            }
        }
    }
}

fn get_board_pos(
    pick_state: Res<PickState>,
    mut shared_data: ResMut<SharedData>
) {
    let pick = pick_state.top(Group::default());

    match pick {
        Some((entity, intersection)) => {
            let pos = intersection.position();

            let board_pos = BoardPosition {x: (pos.x + 4.).floor() as u8,
                y: (-pos.z + 4.).floor() as u8};
            shared_data.cursor_board_pos = board_pos;
        },
        None => ()
    }
}

fn check_mate(board: &Vec<Vec<Option<LogicChessPiece>>>, color: PieceColor) -> bool {
    let mut king_pos = BoardPosition {x: 0, y: 0};
    let mut possible_threats : Vec<(PieceType, BoardPosition)> = Vec::new();

    let enemy_color = match color {
        PieceColor::White => PieceColor::Black,
        PieceColor::Black => PieceColor::White
    };

    for (i, row) in board.iter().enumerate() {
        for (j, field) in row.iter().enumerate() {
            match field {
                None => {}
                Some(piece) => {
                    if let PieceType::King = piece.piece_type {
                        if color == piece.piece_color {
                            king_pos.x = j as u8;
                            king_pos.y = i as u8;
                        }
                    }

                    if let PieceType::Knight = piece.piece_type {
                        if piece.piece_color == enemy_color {
                            possible_threats.push((piece.piece_type,
                                                   BoardPosition { x: j as u8, y: i as u8 }));
                        }
                    }
                }
            }
        }
    }

    // Check left
    if king_pos.x > 0 {
        for i in (0..king_pos.x).rev() {
            match board[king_pos.y as usize][i as usize] {
                None => {}
                Some(piece) => {
                    if color != piece.piece_color {
                        possible_threats.push((piece.piece_type, BoardPosition { x: i, y: king_pos.y }));
                    }

                    break;
                }
            }
        }
    }

    // Check right
    for i in (king_pos.x + 1)..8 {
        match board[king_pos.y as usize][i as usize] {
            None => {}
            Some(piece) => {
                if enemy_color == piece.piece_color {
                    possible_threats.push((piece.piece_type, BoardPosition {x: i, y: king_pos.y}));
                }

                break;
            }
        }
    }

    // Check up
    for i in (king_pos.y + 1)..8 {
        match board[i as usize][king_pos.x as usize] {
            None => {}
            Some(piece) => {
                if enemy_color == piece.piece_color {
                    possible_threats.push((piece.piece_type, BoardPosition {x: king_pos.x, y: i}));
                }

                break;
            }
        }
    }

    // Check down
    if king_pos.y > 0 {
        for i in (0..king_pos.y).rev() {
            match board[i as usize][king_pos.x as usize] {
                None => {}
                Some(piece) => {
                    if enemy_color == piece.piece_color {
                        possible_threats.push((piece.piece_type, BoardPosition { x: king_pos.x, y: i }));
                    }

                    break;
                }
            }
        }
    }

    // Check upper-left
    if king_pos.x > 0 {
        let limit = cmp::min(king_pos.x + 1, 8 - king_pos.y);

        for i in 1..limit {
            match board[(king_pos.y + i) as usize][(king_pos.x - i) as usize] {
                None => {}
                Some(piece) => {
                    if enemy_color == piece.piece_color {
                        possible_threats.push((piece.piece_type, BoardPosition { x: king_pos.x - i, y: king_pos.y + i }));
                    }

                    break;
                }
            }
        }
    }

    // Check upper-right
    let limit = cmp::min(8 - king_pos.x, 8 - king_pos.y);

    for i in 1..limit {
        match board[(king_pos.y + i) as usize][(king_pos.x + i) as usize] {
            None => {}
            Some(piece) => {
                if enemy_color == piece.piece_color {
                    possible_threats.push((piece.piece_type, BoardPosition { x: king_pos.x + i, y: king_pos.y + i }));
                }

                break;
            }
        }
    }

    // Check lower-right
    if king_pos.y > 0 {
        let limit = cmp::min(8 - king_pos.x, king_pos.y + 1);

        for i in 1..limit {
            match board[(king_pos.y - i) as usize][(king_pos.x + i) as usize] {
                None => {}
                Some(piece) => {
                    if enemy_color == piece.piece_color {
                        possible_threats.push((piece.piece_type, BoardPosition { x: king_pos.x + i, y: king_pos.y - i }));
                    }

                    break;
                }
            }
        }
    }

    // Check lower-left
    if king_pos.x > 0 && king_pos.y > 0 {
        let limit = cmp::min(king_pos.x + 1, king_pos.y + 1);

        for i in 1..limit {
            match board[(king_pos.y - i) as usize][(king_pos.x - i) as usize] {
                None => {}
                Some(piece) => {
                    if enemy_color == piece.piece_color {
                        possible_threats.push((piece.piece_type, BoardPosition { x: king_pos.x - i, y: king_pos.y - i }));
                    }

                    break;
                }
            }
        }
    }

    for i in possible_threats.iter() {
        let mut possible = check_capture_pattern(i.0, enemy_color, (i.1, king_pos));

        if possible {
            return true;
        }
    }

    return false;
}

fn check_move_blocked(board: &Vec<Vec<Option<LogicChessPiece>>>,
                      piece_move: (BoardPosition, BoardPosition)) -> bool{

    let (from, to) = piece_move;
    let mut diff = Vec2 { x: to.x as f32 - from.x as f32,
        y: to.y as f32 - from.y as f32 };

    if diff.x == 0. && diff.y == 0. {
        return true;
    }

    let field = board[piece_move.0.y as usize][piece_move.0.x as usize];


    let moved_piece = match field {
        None => return true,
        Some(piece) => {
            piece
        }
    };

    let enemy_color = match moved_piece.piece_color {
        PieceColor::Black => PieceColor::White,
        PieceColor::White => PieceColor::Black
    };

    let dir = if diff.x == 0. {
        Vec2 { x: 0., y: diff.y / diff.y.abs() }
    } else if diff.y == 0. {
        Vec2 { x: diff.x / diff.x.abs(), y: 0. }
    } else {
        Vec2 { x: diff.x / diff.x.abs(), y: diff.y / diff.y.abs() }
    };

    let mut checked_pos = Vec2 { x: piece_move.0.x as f32 + dir.x, y: piece_move.0.y as f32 + dir.y };

    while !(checked_pos.x == piece_move.1.x as f32
        && checked_pos.y == piece_move.1.y as f32) {

        let curr_field = board[checked_pos.y as usize][checked_pos.x as usize];

        match curr_field {
            None => (),
            Some(curr_piece) => {
                return true;
            }
        }

        checked_pos.x += dir.x;
        checked_pos.y += dir.y;
    }

    return false;
}

fn check_move_pattern(piece_type: PieceType, piece_color: PieceColor, move_vec: Vec2) -> bool {
    if move_vec.x == 0. && move_vec.y == 0. {
        return false;
    }

    let move_vec = if let PieceColor::Black = piece_color {
        Vec2 { x: move_vec.x, y: -move_vec.y }
    } else {
        move_vec
    };

    return match piece_type {
        PieceType::King => {
            (move_vec.x.abs() == 0. || move_vec.x.abs() == 1.)
                && (move_vec.y.abs() == 0. || move_vec.y.abs() == 1.)
        }
        PieceType::Queen => {
            move_vec.x.abs() == 0. || move_vec.y.abs() == 0.
                || move_vec.x.abs() == move_vec.y.abs()
        }
        PieceType::Rook => {
            move_vec.x.abs() == 0. || move_vec.y.abs() == 0.
        }
        PieceType::Bishop => {
            move_vec.x.abs() == move_vec.y.abs()
        }
        PieceType::Knight => {
            move_vec.x.abs() == 1. && move_vec.y.abs() == 2.
                || move_vec.x.abs() == 2. && move_vec.y.abs() == 1.
        }
        PieceType::Pawn => {
            move_vec.x == 0. && move_vec.y == 1.
        }
    };
}

fn check_capture_pattern(piece_type: PieceType, piece_color: PieceColor,
                         piece_move: (BoardPosition, BoardPosition)) -> bool {
    let (from, to) = piece_move;

    let diff = Vec2 { x: to.x as f32 - from.x as f32, y: to.y as f32 - from.y as f32 };

    if let PieceType::Pawn = piece_type {
        let diff = if let PieceColor::Black = piece_color {
            Vec2 { x: diff.x, y: -diff.y }
        } else {
            diff
        };

        if diff.x.abs() == 1. && diff.y == 1. {
            return true;
        }
    } else {
        return check_move_pattern(piece_type, piece_color, diff);
    }

    return false;
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
        .with(PickableMesh::default())
        .with(InteractableMesh::default())
        .with(ChessBoard)
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
            game_state: WaitingForSelect,
            cursor_board_pos: BoardPosition {x: 0, y: 0},
            current_move: PieceColor::White,
            board: vec![vec![None; 8]; 8]
        });
}

fn print_board(board: &Vec<Vec<Option<LogicChessPiece>>>) {
    for i in board.iter().rev() {
        for j in i.iter() {
            let symbol = match j {
                None => " ",
                Some(piece) => {
                    if let PieceColor::White = piece.piece_color {
                        match piece.piece_type {
                            PieceType::King => "♔",
                            PieceType::Queen => "♕",
                            PieceType::Rook => "♖",
                            PieceType::Bishop => "♗",
                            PieceType::Knight => "♘",
                            PieceType::Pawn => "♙"
                        }
                    } else {
                        match piece.piece_type {
                            PieceType::King => "♚",
                            PieceType::Queen => "♛",
                            PieceType::Rook => "♜",
                            PieceType::Bishop => "♝",
                            PieceType::Knight => "♞",
                            PieceType::Pawn => "♟"
                        }
                    }
                }
            };

            print!("{}", symbol);

        }

        println!();
    }
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
        .add_system(board_raycast_system.system())
        .add_system(get_board_pos.system())
        .run();
}