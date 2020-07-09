use std::time::{Instant};
use std::path::PathBuf;
use structopt::StructOpt;

use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{WindowEvent, ElementState, VirtualKeyCode, Event, KeyboardInput, MouseButton};
use winit::window::WindowBuilder;
use winit::event::DeviceEvent::{MouseMotion};
use nalgebra_glm::{identity, triangle_normal, make_vec3};
use game::*;
use game::command_queue::CommandQueue;
use game::entity::{Posable, Followable};
use xp_ui::{UI, Widget, DEFAULT_LAYOUT, Label, ActionType};
use xp_ui::Widget::LabelW;
use std::convert::TryInto;

#[derive(Debug, StructOpt)]
#[structopt(name = "options", about = "command line options")]
pub struct Options {
    #[structopt(long = "recording", parse(from_os_str))]
    record_path: Option<PathBuf>,

    #[structopt(long = "replay", parse(from_os_str))]
    replay_path: Option<PathBuf>,
}

pub fn create_terrain_mesh_from_tile(tile_x: i32, tile_z: i32, lod: usize) -> graphics::Mesh<graphics::Vertex> {
    const LOD_COLOR: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let mut terrain = graphics::Mesh { vertices: Vec::new(), indices: Vec::new() };
    let grid_0_0_0 = terrain::Tile::new(tile_x, tile_z, lod);
    for z in 0..terrain::TILE_SIZE {
        for x in 0..terrain::TILE_SIZE {
            terrain.vertices.push(graphics::Vertex {
                position: grid_0_0_0.get_element(x, z).p.clone(),
                normal: [0.0, 1.0, 0.0],
                color: LOD_COLOR[lod],
            });
        }
    }
    const S: u32 = terrain::TILE_SIZE as u32;
    const S_MIN_1: u32 = S - 1;
    for x in 0..S_MIN_1 {
        for z in 0..S {
            let index = x + z * S;
            match (x, z) {
                (_, 0) => {
                    let p1 = index;
                    let p2 = index + S;
                    let p3 = index + 1;
                    terrain.indices.push(p1);
                    terrain.indices.push(p2);
                    terrain.indices.push(p3);
                    terrain.vertices[index as usize].normal = triangle_normal(&make_vec3(&terrain.vertices[p1 as usize].position), &make_vec3(&terrain.vertices[p2 as usize].position), &make_vec3(&terrain.vertices[p3 as usize].position)).as_slice().try_into().unwrap();
                },
                (_, S_MIN_1) => {
                    let p1 = index;
                    let p2 = index + 1;
                    let p3 = index - S + 1;
                    terrain.indices.push(p1);
                    terrain.indices.push(p2);
                    terrain.indices.push(p3);
                    terrain.vertices[index as usize].normal = triangle_normal(&make_vec3(&terrain.vertices[p1 as usize].position), &make_vec3(&terrain.vertices[p2 as usize].position), &make_vec3(&terrain.vertices[p3 as usize].position)).as_slice().try_into().unwrap();
                },
                (_, _) => {

                    let p1 = index;
                    let p2 = index + S;
                    let p3 = index + 1;
                    terrain.indices.push(p1);
                    terrain.indices.push(p2);
                    terrain.indices.push(p3);
                    terrain.vertices[index as usize].normal = triangle_normal(&make_vec3(&terrain.vertices[p1 as usize].position), &make_vec3(&terrain.vertices[p2 as usize].position), &make_vec3(&terrain.vertices[p3 as usize].position)).as_slice().try_into().unwrap();

                    // duplicate this vertex, as provoking vertices cannot be re-used
                    terrain.vertices.push(terrain.vertices[index as usize]);
                    let p4 = terrain.vertices.len() as u32 - 1;
                    let p5 = index + 1;
                    let p6 = index - S + 1;
                    terrain.indices.push(p4);
                    terrain.indices.push(p5);
                    terrain.indices.push(p6);
                    terrain.vertices[p4 as usize].normal = triangle_normal(&make_vec3(&terrain.vertices[p4 as usize].position), &make_vec3(&terrain.vertices[p5 as usize].position), &make_vec3(&terrain.vertices[p6 as usize].position)).as_slice().try_into().unwrap();
                }
            }
        }
    }
    terrain
}

pub fn create_mesh_from(obj_file_name: &str) -> graphics::Mesh<graphics::Vertex> {
    let (models, materials) = tobj::load_obj(obj_file_name, true).expect(format!("Could not read obj file: {}", obj_file_name).as_str());
    let mut mesh = graphics::Mesh { vertices: Vec::new(), indices: Vec::new() };
    for model in models {
        let color = if let Some(material_id) = model.mesh.material_id {
            materials[material_id].diffuse
        } else {
            [0.8, 0.0, 0.8]
        };
        let model_mesh = graphics::make_mesh_from_flat_obj(model.mesh.positions.as_ref(), model.mesh.indices.as_ref(), &color);
        let index_offset = mesh.vertices.len() as u32;
        mesh.vertices.extend(model_mesh.vertices);
        mesh.indices.extend(model_mesh.indices.iter().map(|i| i + index_offset ));
    }
    mesh
}

pub struct UIContext {
    pub ui_enabled: bool,
    pub camera: camera::CameraType,
}

pub fn create_mesh(ui: &UI<UIContext, u32>) -> (graphics::Mesh::<graphics::UIVertex>, Vec<graphics::Text>) {
    let mut mesh = graphics::Mesh::<graphics::UIVertex> { vertices: Vec::new(), indices: Vec::new() };
    let mut text = Vec::new();
    for (_, widget) in ui.widgets() {
        match widget {
            Widget::LabelW(layout, label) => {
                let top_left = graphics::UIVertex {
                    position: [layout.position.x, layout.position.y],
                    uv: [0.0, 0.0],
                    color: label.color,
                };
                let bottom_left = graphics::UIVertex {
                    position: [layout.position.x, layout.position.y - layout.size.height],
                    uv: [0.0, 0.0],
                    color: label.color,
                };
                let top_right = graphics::UIVertex {
                    position: [layout.position.x + layout.size.width, layout.position.y],
                    uv: [0.0, 0.0],
                    color: label.color,
                };
                let bottom_right = graphics::UIVertex {
                    position: [layout.position.x + layout.size.width, layout.position.y - layout.size.height],
                    uv: [0.0, 0.0],
                    color: label.color,
                };
                text.push(graphics::Text{
                    pos: (layout.position.x, layout.position.y - ui.window_size.1),
                    text: label.text.text.clone(),
                    font_size: label.text.font_size,
                    color: label.text.color,
                });
                let offset = mesh.vertices.len() as u32;
                mesh.indices.extend_from_slice(&[offset + 0, offset + 1, offset + 2, offset + 2, offset + 1, offset + 3]);
                mesh.vertices.extend_from_slice(&[top_left, bottom_left, top_right, bottom_right]);
            },
        }
    }
    (mesh, text)
}

fn game(options: Options) {
    let mut game_state = UIContext { ui_enabled: false, camera: camera::CameraType::FreeLook, };
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop).expect("Could not create window");

    let mut player = entity::Entity::new();

    let player_mesh = create_mesh_from("obj/arrow.obj");
    let axis_mesh = create_mesh_from("obj/axis.obj");

    let mut ui = UI::<UIContext, u32>::new(window.inner_size().width as f32, window.inner_size().height as f32);
    let fps_label_id = ui.add(LabelW(DEFAULT_LAYOUT, Label::build("fps").with_color([255, 255, 0, 255])));
    let camera_button_id =ui.add(LabelW(DEFAULT_LAYOUT, Label::build("camera").with_color([0, 0, 255, 255])));
    ui.add_action_for_id(camera_button_id, ActionType::OnClick, |context|{ match context.camera {
        camera::CameraType::Follow => { context.camera = camera::CameraType::FreeLook },
        camera::CameraType::FreeLook => { context.camera = camera::CameraType::Follow },
    }});
    ui.layout();
    let mut renderer = futures::executor::block_on(graphics::Renderer::new(&window, create_mesh(&ui).0)).expect("Could not create graphics renderer");
    renderer.create_drawable_from_mesh(&player_mesh);
    let mut terrain = Vec::new();
    terrain.push(renderer.create_drawable_from_mesh2(&create_terrain_mesh_from_tile(0, 0, 0)));
    terrain.push(renderer.create_drawable_from_mesh2(&create_terrain_mesh_from_tile(0, 0, 1)));
    terrain.push(renderer.create_drawable_from_mesh2(&create_terrain_mesh_from_tile(0, 0, 2)));
    renderer.create_drawable_from_mesh(&axis_mesh);


    let mut previous_time = Instant::now();

    let mut inputs = input::InputQueue::new();
    let mut commands_queue = CommandQueue::new();
    let mut simulation = simulation::Simulation::new();
    let mut client= local_client::LocalClient::new();
    let mut record = recording::try_create_recorder(options.record_path);
    let replaying = options.replay_path != None;
    let mut replay = recording::try_create_replayer(options.replay_path);

    /* every client creates if possible for every frame its commands, if there are no commands then
       the server will do nothing. every client runs its own local simulation. when the server
       returns the frames it merged from all clients (this means the server also needs to wait
        on slow clients). the "real" simulation is applied and the
       local simulation is rebased on that. In this way user input acts fast and the simulation
       state is equal on all clients at the cost rebasing the simulation every time the server
       sends an update.
       */
    let mut frame_counter = counter::FrameCounter::new(60);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        frame_counter.run();
        if !replaying {
            let input_commands = commands_queue.handle_input(&mut inputs, frame_counter.count());
            client::send(&mut client, input_commands.as_slice());
        } else {
            let replay_commands = client::receive(&mut *replay, frame_counter.count());
            client::send(&mut client, replay_commands.as_slice());
        }
        let commands_received = client::receive(&mut client,frame_counter.count());
        client::send(&mut *record, commands_received.as_slice());
        for frame in &commands_received {
            let _ = simulation.handle_frame(frame, &game_state.camera, &mut player);
        }

        match event {
            Event::RedrawRequested(_) => {
                // first rotate all vertices on 0,0,0 (rotate around origin), then translate all points towards location.
                renderer.update(player.pose(), identity(), identity());
                // for the view matrix we can also use player_move and player_rotate, and use the inverse of the resulting matrix
                let view = match game_state.camera {
                    camera::CameraType::FreeLook => simulation.freelook_camera.view(),
                    camera::CameraType::Follow => player.follow(),
                };
                let current_time = Instant::now();
                let fps = (1000.0 / (current_time - previous_time).as_millis() as f32) as u32;
                if let Some(fps_label) = ui.try_get_mut_label(fps_label_id) {
                    fps_label.text.text = fps.to_string();
                }
                previous_time = current_time;
                futures::executor::block_on(renderer.render(terrain.as_slice(), view, game_state.ui_enabled, Some(create_mesh(&ui))));
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::DeviceEvent {
                ref event,
                ..
            } =>
            match event {
                MouseMotion { delta } => {
                    if !game_state.ui_enabled {
                        inputs.push_mouse_movement(delta);
                    }
                },
                _ => (),
            },
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                #[allow(deprecated)]
                WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } => {
                    ui.update_cursor_position(position.x as f32, position.y as f32);
                }
                #[allow(deprecated)]
                WindowEvent::MouseInput { device_id: _, state, button, modifiers: _ } => {
                    match (state, button) {
                        (ElementState::Pressed, MouseButton::Left) => {
                            if game_state.ui_enabled {
                                ui.click(&mut game_state);
                            }
                        }
                        (_, _) => (),
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    ui.update_window_size(physical_size.width as f32, physical_size.height as f32);
                    futures::executor::block_on(renderer.resize(*physical_size));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, ..} => {
                    ui.update_window_size(new_inner_size.width as f32, new_inner_size.height as f32);
                    futures::executor::block_on(renderer.resize(**new_inner_size));
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input,
                    ..
                } => {
                    match (input, game_state.ui_enabled) {
                        (KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        }, _) => game_state.ui_enabled = !game_state.ui_enabled,
                        (KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Q),
                            ..
                        }, true) => *control_flow = ControlFlow::Exit,
                        _ => inputs.push_keyboard_input(input),
                    }
                }
                _ => {}
            }
            _ => {}
        }
    });
}

fn main() {
    let options = Options::from_args();
    game(options)
}
