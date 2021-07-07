use bevy::prelude::*;
use std::f32::consts::{PI, FRAC_PI_2};
use bevy_mod_picking::*;
use rand::random;
use crate::GameState::{WaitingForSelect, PawnPromoting};
use bevy::input::gamepad::GamepadButtonType::Select;
use bevy::window::WindowId;
use std::cmp;
use crate::PieceType::Knight;
use bevy::input::mouse::{MouseMotion, MouseButtonInput, MouseWheel};
use bevy::render::camera::Camera;

struct ChessPiece;
struct ChessBoard;
struct PromotionSelector;

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
    board: Vec<Vec<Option<LogicChessPiece>>>,
    initial_pos: Vec<Vec<bool>>,
    rotating: bool,
    rotation_angle: Vec3,
    camera_distance: f32
}

enum GameState {
    WaitingForSelect,
    PieceSelected,
    PieceMoving,
    SpawnPromotionSelector(BoardPosition, PieceColor),
    PawnPromoting(BoardPosition, PieceColor)
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
                    &mut shared_data);
        spawn_piece(commands, &textures, &mut materials, &meshes,
                    PieceType::Pawn, PieceColor::Black,
                    BoardPosition { x: i, y: 6 },
                    &mut shared_data);
    }

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::White,
                BoardPosition {x: 0, y: 0},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::White,
                BoardPosition {x: 7, y: 0},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::Black,
                BoardPosition {x: 0, y: 7},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Rook, PieceColor::Black,
                BoardPosition {x: 7, y: 7},
                &mut shared_data);

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::White,
                BoardPosition {x: 1, y: 0},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::White,
                BoardPosition {x: 6, y: 0},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::Black,
                BoardPosition {x: 1, y: 7},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Knight, PieceColor::Black,
                BoardPosition {x: 6, y: 7},
                &mut shared_data);

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::White,
                BoardPosition {x: 2, y: 0},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::White,
                BoardPosition {x: 5, y: 0},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::Black,
                BoardPosition {x: 2, y: 7},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Bishop, PieceColor::Black,
                BoardPosition {x: 5, y: 7},
                &mut shared_data);

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Queen, PieceColor::White,
                BoardPosition {x: 3, y: 0},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::Queen, PieceColor::Black,
                BoardPosition {x: 3, y: 7},
                &mut shared_data);

    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::King, PieceColor::White,
                BoardPosition {x: 4, y: 0},
                &mut shared_data);
    spawn_piece(commands, &textures, &mut materials, &meshes,
                PieceType::King, PieceColor::Black,
                BoardPosition {x: 4, y: 7},
                &mut shared_data);
}

fn spawn_piece(commands: &mut Commands, textures: &Res<Textures>,
               materials: &mut ResMut<Assets<StandardMaterial>>, meshes: &Res<Meshes>,
               piece_type: PieceType, color: PieceColor, position: BoardPosition,
               shared_data: &mut SharedData) {

    let (board, initial_pos) = (&mut shared_data.board, &mut shared_data.initial_pos);

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
    initial_pos[usize::from(position.y)][usize::from(position.x)] = true;

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
    mut query3: Query<(&InteractableMesh, &mut Transform, &mut BoardPosition, &PieceColor, &PieceType, Entity), Without<SelectedPiece>>,
    textures: Res<Textures>,
    mut materials: ResMut<Assets<StandardMaterial>>, mut shared_data: ResMut<SharedData>) {

    let mut captured_color = PieceColor::White;
    let mut captured_position = BoardPosition { x: 0, y: 0 };
    let mut should_capture = false;
    let mut should_castle = false;

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

                let mut possible = match dest_field {
                    None => {
                        let initial_pos = shared_data.initial_pos[usize::from(board_position.y)][usize::from(board_position.x)];

                        if *piece_type != PieceType::Knight {
                            check_move_pattern(*piece_type, *piece_color, move_vec, initial_pos)
                            && !check_move_blocked(&shared_data.board, (*board_position, shared_data.cursor_board_pos))
                        } else {
                            check_move_pattern(*piece_type, *piece_color, move_vec, initial_pos)
                        }
                    },
                    Some(dest_piece) => {
                        if dest_piece.piece_color == *piece_color {
                            if *piece_type == PieceType::King && dest_piece.piece_type == PieceType::Rook
                            && check_castling(&shared_data.board, &shared_data.initial_pos, (*board_position, shared_data.cursor_board_pos)) {
                                should_castle = true;
                                true
                            } else {
                                false
                            }
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

                if !should_castle {
                    let mut board_copy = shared_data.board.clone();
                    board_copy[usize::from(board_position.y)][usize::from(board_position.x)] = None;
                    board_copy[usize::from(shared_data.cursor_board_pos.y)][usize::from(shared_data.cursor_board_pos.x)] = Some(LogicChessPiece {
                        piece_color: piece_color.clone(),
                        piece_type: piece_type.clone()
                    });

                    possible = !check_mate(&board_copy, *piece_color);
                }

                if !possible {
                    continue;
                }

                shared_data.initial_pos[usize::from(board_position.y)][usize::from(board_position.x)] = false;
                shared_data.board[usize::from(board_position.y)][usize::from(board_position.x)] = None;

                board_position.x = shared_data.cursor_board_pos.x;
                board_position.y = shared_data.cursor_board_pos.y;

                if should_castle {
                    if board_position.x == 0 {
                        board_position.x = 2;
                    } else {
                        board_position.x = 6;
                    }
                } else {
                    if *piece_type == PieceType::Pawn
                    && (*piece_color == PieceColor::Black && board_position.y == 0 ||
                        *piece_color == PieceColor::White && board_position.y == 7) {
                        commands.insert(entity, (SelectedPiece, ));

                        let material = materials.get_mut(material_handle).unwrap();

                        material.albedo_texture = None;
                        material.albedo = Color::rgb(0.0, 0.0, 1.0);


                        shared_data.game_state = GameState::SpawnPromotionSelector (board_position.clone(),
                                                                                    piece_color.clone());
                    }
                }

                transform.translation = board_to_global(*board_position);

                shared_data.board[usize::from(board_position.y)][usize::from(board_position.x)] = Some(LogicChessPiece {
                    piece_color: piece_color.clone(),
                    piece_type: piece_type.clone()
                });

                shared_data.current_move = match shared_data.current_move {
                    PieceColor::White => PieceColor::Black,
                    PieceColor::Black => PieceColor::White
                };

                if should_capture || should_castle {
                    captured_color = piece_color.clone();
                    captured_position = shared_data.cursor_board_pos.clone();
                }

                break;
            }

            if should_capture || should_castle {
                for (mesh, mut transform, mut board_position,
                     piece_color, piece_type, entity) in query3.iter_mut() {
                    if board_position.x == captured_position.x
                        && board_position.y == captured_position.y {

                        if should_capture && *piece_color != captured_color {
                            commands.despawn(entity);
                        }

                        if should_castle && *piece_color == captured_color {
                            if let PieceType::Rook = *piece_type {
                                shared_data.board[board_position.y as usize][board_position.x as usize] = None;

                                if board_position.x == 0 {
                                    board_position.x = 3;
                                } else {
                                    board_position.x = 5;
                                }

                                transform.translation = board_to_global(*board_position);
                                shared_data.board[board_position.y as usize][board_position.x as usize] = Some(LogicChessPiece {
                                    piece_color: piece_color.clone(),
                                    piece_type: PieceType::Rook
                                });
                            }
                        }
                    }
                }
            }

            print_board(&shared_data.board);
        }
    }
}

fn spawn_promotion_selector(commands: &mut Commands, textures: Res<Textures>,
                            mut materials: ResMut<Assets<StandardMaterial>>, meshes: Res<Meshes>,
                            mut shared_data: ResMut<SharedData>) {
    let meshes = [ meshes.queen.clone(), meshes.rook.clone(),
                                    meshes.bishop.clone(), meshes.knight.clone()];
    let piece_types = [ PieceType::Queen, PieceType::Rook,
                                      PieceType::Bishop, PieceType::Knight ];

    let (board_position, piece_color) = match shared_data.game_state {
        GameState::SpawnPromotionSelector(board_position, piece_color) => {
            (board_position, piece_color)
        },
        _ => return
    };

    let texture = match piece_color {
        PieceColor::White => textures.texture_white.clone(),
        PieceColor::Black => textures.texture_black.clone()
    };

    let mut rotation_rad: f32 = match piece_color {
        PieceColor::White => 0.0,
        PieceColor::Black => PI
    };

    let mut position = board_to_global(board_position);
    position.x -= 1.5;
    position.y += 0.6;
    position.z += match piece_color {
        PieceColor::Black => 1.,
        PieceColor::White => -1.
    };

    for i in 0..4 {
        rotation_rad = if piece_types[i] == PieceType::Knight {
            rotation_rad - FRAC_PI_2
        } else {
            rotation_rad
        };

        commands.spawn(PbrBundle {
                mesh: meshes[i].clone(),
                material: materials.add(StandardMaterial {
                    albedo_texture: Some(texture.clone()),
                    ..Default::default()
                }),
                transform: Transform {
                    translation: position.clone(),
                    rotation: Quat::from_rotation_y(rotation_rad),
                    ..Default::default()
                },
                ..Default::default()})
                .with(PickableMesh::default())
                .with(InteractableMesh::default())
                .with(piece_types[i])
                .with(position)
                .with(PromotionSelector);

        position.x += 1.;
    }

    shared_data.game_state = PawnPromoting(board_position.clone(), piece_color.clone());
}

fn selector_system(commands: &mut Commands,
                   mut query: Query<(&InteractableMesh, &PieceType, Entity), With<PromotionSelector>>,
                   mut query2: Query<(&PieceType, Entity, ), With<SelectedPiece>>,
                   textures: Res<Textures>, mut materials: ResMut<Assets<StandardMaterial>>, meshes: Res<Meshes>,
                   mut shared_data: ResMut<SharedData>) {

    let (board_position, piece_color) = match shared_data.game_state {
        GameState::PawnPromoting(board_position, piece_color) => {
            (board_position, piece_color)
        },
        _ => return
    };

    let mut selected = false;
    let mut piece_type = PieceType::Queen;

    for (interactable, selected_piece_type, entity) in query.iter() {
        let mouse_down_event = interactable
            .mouse_down_event(&Group::default(), MouseButton::Left)
            .unwrap();

        if mouse_down_event.is_none() {
            continue;
        }

        if let MouseDownEvents::MouseJustReleased = mouse_down_event {
            selected = true;
            piece_type = selected_piece_type.clone();
        }
    }

    if !selected {
        return;
    }

    for (interactable, selected_piece_type, entity) in query.iter() {
        commands.despawn(entity);
    }

    for (_, entity) in query2.iter() {
        commands.despawn(entity);
    }

    spawn_piece(commands, &textures, &mut materials, &meshes, piece_type, piece_color, board_position, &mut shared_data);
    shared_data.initial_pos[board_position.y as usize][board_position.x as usize] = false;

    shared_data.game_state = GameState::WaitingForSelect;
}

fn camera_rotation_system(
    evt_motion: Res<Events<MouseMotion>>,
    mut evr_motion: Local<EventReader<MouseMotion>>,
    evt_mousebtn: Res<Events<MouseButtonInput>>,
    mut evr_mousebtn: Local<EventReader<MouseButtonInput>>,
    evt_scroll: Res<Events<MouseWheel>>,
    mut evr_scroll: Local<EventReader<MouseWheel>>,

    mut shared_data: ResMut<SharedData>,
    mut query: Query<(&mut Transform, ), With<Camera>>,
    mut query2: Query<(&mut Transform, ), With<Light>>) {

    let mut update_needed = false;

    for e in evr_scroll.iter(&evt_scroll) {
        shared_data.camera_distance -= e.y * 0.8;

        if shared_data.camera_distance < 9. {
            shared_data.camera_distance = 9.
        } else if shared_data.camera_distance > 20. {
            shared_data.camera_distance = 20.
        }

        update_needed = true;
    }

    if shared_data.rotating {
        for e in evr_motion.iter(&evt_motion) {
            shared_data.rotation_angle.x += e.delta.y / (16. * PI);
            shared_data.rotation_angle.y += e.delta.x / (16. * PI);

            if shared_data.rotation_angle.x > PI / 2. - PI / 360. {
                shared_data.rotation_angle.x = PI / 2. - PI / 360.;
            }

            if shared_data.rotation_angle.x < 5. * PI / 180. {
                shared_data.rotation_angle.x = 5. * PI / 180.;
            }

            while shared_data.rotation_angle.y >= 2. * PI {
                shared_data.rotation_angle.y -= 2. * PI;
            }

            update_needed = true;
        }
    }

    if update_needed {
        let radius = shared_data.camera_distance;

        let tmp_transform = Transform::from_translation(Vec3::new(
            radius * shared_data.rotation_angle.x.cos() * shared_data.rotation_angle.y.cos(),
            radius * shared_data.rotation_angle.x.sin(),
            radius * shared_data.rotation_angle.x.cos() * shared_data.rotation_angle.y.sin()))
            .looking_at(Vec3::default(), Vec3::unit_y());

        for (mut transform, ) in query.iter_mut() {
            transform.translation = tmp_transform.translation;
            transform.rotation = tmp_transform.rotation;
        }

        for (mut transform, ) in query2.iter_mut() {
            transform.translation = tmp_transform.translation;
            transform.rotation = tmp_transform.rotation;
        }
    }

    for e in evr_mousebtn.iter(&evt_mousebtn) {

        if e.button != MouseButton::Right {
            continue;
        }

        if e.state.is_pressed() {
            shared_data.rotating = true;
        } else {
            shared_data.rotating = false;
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

fn check_move_pattern(piece_type: PieceType, piece_color: PieceColor,
                      move_vec: Vec2, initial_pos: bool) -> bool {
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
            if initial_pos {
                move_vec.x == 0. && (move_vec.y == 1. || move_vec.y == 2.)
            } else {
                move_vec.x == 0. && move_vec.y == 1.
            }
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
        return check_move_pattern(piece_type, piece_color, diff, false);
    }

    return false;
}

fn check_castling(board: &Vec<Vec<Option<LogicChessPiece>>>, initial_pos: &Vec<Vec<bool>>,
                  checked_move: (BoardPosition, BoardPosition)) -> bool {
    let (from, to) = checked_move;

    if !initial_pos[from.y as usize][from.x as usize]
        || !initial_pos[to.y as usize][to.x as usize] {
        return false;
    }

    let source_field = board[from.y as usize][from.x as usize];
    let mut piece_color = PieceColor::White;

    match source_field {
        None => { return false },
        Some(source_piece) => {
            piece_color = source_piece.piece_color.clone();
        }
    }

    let dir: i8 = if checked_move.0.x > checked_move.1.x {
        -1
    } else {
        1
    };

    let mut checked_pos = from.clone();

    for i in 1u8..3 {
        checked_pos.x = (checked_pos.x as i8 + dir) as u8;
        let checked_field = board[checked_pos.y as usize][checked_pos.x as usize];

        match checked_field {
            None => {
                let mut board_copy = board.clone();
                board_copy[checked_move.0.y as usize][checked_move.0.x as usize] = None;
                board_copy[checked_pos.y as usize][checked_pos.x as usize] = Some(LogicChessPiece {
                    piece_color: piece_color.clone(),
                    piece_type: PieceType::King
                });

                if check_mate(&board_copy, piece_color) {
                    return false;
                }
            },
            Some(_) => {
                return false;
            }
        }
    }

    if checked_move.1.x == 0 && !matches!(board[checked_move.1.y as usize][1], None) {
        return false;
    }

    return true;
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
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 10.0))
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
            board: vec![vec![None; 8]; 8],
            initial_pos: vec![vec![false; 8]; 8],
            rotating: false,
            rotation_angle: Vec3::new(PI / 4., PI / 2., 0.),
            camera_distance: (200.0_f32).sqrt()
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
        .add_resource(WindowDescriptor {
            title: "rusty_chess".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_pieces", SystemStage::single(piece_spawner.system()))
        .add_system(piece_raycast_system.system())
        .add_system(board_raycast_system.system())
        .add_system(get_board_pos.system())
        .add_system(spawn_promotion_selector.system())
        .add_system(selector_system.system())
        .add_system(camera_rotation_system.system())
        .run();
}