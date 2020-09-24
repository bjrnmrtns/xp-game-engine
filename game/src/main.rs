use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

use game::command_queue::CommandQueue;
use game::graphics::clipmap;
use game::*;
use nalgebra_glm::{identity, ortho, perspective, Mat4};
use winit::event::DeviceEvent::MouseMotion;
use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use xp_ui::Widget::LabelW;
use xp_ui::{ActionType, Label, DEFAULT_LAYOUT, UI};

#[derive(Debug, StructOpt)]
#[structopt(name = "options", about = "command line options")]
pub struct Options {
    #[structopt(long = "recording", parse(from_os_str))]
    record_path: Option<PathBuf>,

    #[structopt(long = "replay", parse(from_os_str))]
    replay_path: Option<PathBuf>,
}

pub struct UIContext {
    pub ui_enabled: bool,
    pub camera: camera::CameraType,
}

fn game(options: Options) {
    let mut game_state = UIContext {
        ui_enabled: false,
        camera: camera::CameraType::FreeLook,
    };
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Could not create window");

    let mut player = entity::Entity::new();

    //    let player_mesh = mesh::create_mesh_from("obj/arrow.obj");
    let player_mesh = mesh::create_player_sphere();
    let axis_mesh = mesh::create_mesh_from("obj/axis.obj");

    let mut ui = UI::<UIContext, u32>::new(
        window.inner_size().width as f32,
        window.inner_size().height as f32,
    );
    let fps_label_id = ui.add(LabelW(
        DEFAULT_LAYOUT,
        Label::build("fps").with_color([255, 255, 0, 255]),
    ));
    let camera_button_id = ui.add(LabelW(
        DEFAULT_LAYOUT,
        Label::build("camera").with_color([0, 0, 255, 255]),
    ));
    ui.add_action_for_id(
        camera_button_id,
        ActionType::OnClick,
        |context| match context.camera {
            camera::CameraType::Follow => context.camera = camera::CameraType::FreeLook,
            camera::CameraType::FreeLook => context.camera = camera::CameraType::FreeLook2,
            camera::CameraType::FreeLook2 => context.camera = camera::CameraType::Follow,
        },
    );
    ui.layout();
    let mut graphics = futures::executor::block_on(graphics::Graphics::new(&window))
        .expect("Could not create graphics renderer");
    let mut renderables = futures::executor::block_on(graphics::Renderables::new(
        &graphics.device,
        &graphics.queue,
        &graphics.sc_descriptor,
    ))
    .expect("Could not create graphics renderer");
    renderables
        .default
        .create_drawable(&graphics.device, &player_mesh);
    renderables
        .default
        .create_drawable(&graphics.device, &axis_mesh);

    let mut previous_time = Instant::now();

    let mut inputs = input::InputQueue::new();
    let mut commands_queue = CommandQueue::new();
    let mut simulation = simulation::Simulation::new();
    let mut client = local_client::LocalClient::new();
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
        let commands_received = client::receive(&mut client, frame_counter.count());
        client::send(&mut *record, commands_received.as_slice());
        for frame in &commands_received {
            let _ = simulation.handle_frame(frame, &game_state.camera, &mut player);
        }

        match event {
            Event::RedrawRequested(_) => {
                // first rotate all vertices on 0,0,0 (rotate around origin), then translate all points towards location.
                // for the view matrix we can also use player_move and player_rotate, and use the inverse of the resulting matrix
                let view = match game_state.camera {
                    camera::CameraType::FreeLook => simulation.freelook_camera.view(),
                    camera::CameraType::FreeLook2 => simulation.freelook_camera2.view(),
                    camera::CameraType::Follow => camera::view_on(&player.pose).0,
                };
                let current_time = Instant::now();
                let fps = (1000.0 / (current_time - previous_time).as_millis() as f32) as u32;
                if let Some(fps_label) = ui.try_get_mut_label(fps_label_id) {
                    fps_label.text.text = fps.to_string();
                }
                previous_time = current_time;
                let projection_2d = ortho(
                    0.0,
                    graphics.sc_descriptor.width as f32,
                    0.0,
                    graphics.sc_descriptor.height as f32,
                    -1.0,
                    1.0,
                );
                let projection_3d = perspective(
                    graphics.sc_descriptor.width as f32 / graphics.sc_descriptor.height as f32,
                    45.0,
                    0.1,
                    10000.0,
                );

                // update all renderers
                let mut instances = Vec::new();
                instances.push(graphics::default::Instance {
                    model: player.pose.to_mat4(),
                });
                instances.push(graphics::default::Instance { model: identity() });
                renderables.default.update(
                    graphics::default::Uniforms {
                        projection: projection_3d.clone() as Mat4,
                        view: view.clone() as Mat4,
                    },
                    instances,
                );
                let time_before_clipmap_update = std::time::Instant::now();
                renderables.clipmap.update(clipmap::Uniforms {
                    projection: projection_3d.clone() as Mat4,
                    view: view.clone() as Mat4,
                    camera_position: camera::view_on(&player.pose).1, //simulation.freelook_camera.position,
                });
                let time_after_clipmap_update = std::time::Instant::now();
                renderables
                    .ui
                    .create_drawable(&graphics.device, Some(mesh::create_mesh(&ui)));
                renderables.ui.update(
                    graphics::ui::Uniforms {
                        projection: projection_2d,
                    },
                    game_state.ui_enabled,
                );

                let target = &graphics
                    .swap_chain
                    .get_current_frame()
                    .expect("failed to get next texture")
                    .output
                    .view;
                let time_before_render = std::time::Instant::now();
                graphics::render_loop(
                    &renderables,
                    &graphics.device,
                    &graphics.queue,
                    target,
                    &graphics.depth_texture.view,
                );
                let time_after_render = std::time::Instant::now();
                println!(
                    "clipmap-update us: {} render us: {}",
                    (time_after_clipmap_update - time_before_clipmap_update).as_micros(),
                    (time_after_render - time_before_render).as_micros()
                );
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::DeviceEvent { ref event, .. } => match event {
                MouseMotion { delta } => {
                    if !game_state.ui_enabled {
                        inputs.push_mouse_movement(delta);
                    }
                }
                _ => (),
            },
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                #[allow(deprecated)]
                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    modifiers: _,
                } => {
                    ui.update_cursor_position(position.x as f32, position.y as f32);
                }
                #[allow(deprecated)]
                WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                    modifiers: _,
                } => match (state, button) {
                    (ElementState::Pressed, MouseButton::Left) => {
                        if game_state.ui_enabled {
                            ui.click(&mut game_state);
                        }
                    }
                    (_, _) => (),
                },
                WindowEvent::Resized(physical_size) => {
                    ui.update_window_size(physical_size.width as f32, physical_size.height as f32);
                    futures::executor::block_on(graphics.resize(*physical_size));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    ui.update_window_size(
                        new_inner_size.width as f32,
                        new_inner_size.height as f32,
                    );
                    futures::executor::block_on(graphics.resize(**new_inner_size));
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match (input, game_state.ui_enabled) {
                    (
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        _,
                    ) => game_state.ui_enabled = !game_state.ui_enabled,
                    (
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Q),
                            ..
                        },
                        _,
                    ) => *control_flow = ControlFlow::Exit,
                    (
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::C),
                            ..
                        },
                        _,
                    ) => match game_state.camera {
                        camera::CameraType::Follow => {
                            game_state.camera = camera::CameraType::FreeLook
                        }
                        camera::CameraType::FreeLook => {
                            game_state.camera = camera::CameraType::FreeLook2
                        }
                        camera::CameraType::FreeLook2 => {
                            game_state.camera = camera::CameraType::Follow
                        }
                    },
                    _ => inputs.push_keyboard_input(input),
                },
                _ => {}
            },
            _ => {}
        }
    });
}

fn main() {
    let options = Options::from_args();
    game(options)
}
