use std::time::{Instant};
use std::path::PathBuf;
use structopt::StructOpt;

use xp::{*, command_queue::CommandQueue};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{WindowEvent, ElementState, VirtualKeyCode, Event, KeyboardInput};
use winit::window::WindowBuilder;
use winit::event::DeviceEvent::MouseMotion;
use xp::entity::{Posable, Followable};
use nalgebra_glm::identity;

#[derive(Debug, StructOpt)]
#[structopt(name = "options", about = "command line options")]
pub struct Options {
    #[structopt(long = "recording", parse(from_os_str))]
    record_path: Option<PathBuf>,

    #[structopt(long = "replay", parse(from_os_str))]
    replay_path: Option<PathBuf>,
}

pub fn create_mesh_from(obj_file_name: &str) -> graphics::Mesh {
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

fn game(options: Options) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop).expect("Could not create window");

    let camera = camera::CameraType::FreeLook;
    let mut player = entity::Entity::new();

    let player_mesh = create_mesh_from("obj/arrow.obj");
    let terrain_mesh = create_mesh_from("obj/ground-plane-20x20.obj");
    let axis_mesh = create_mesh_from("obj/axis.obj");

    let mut renderer = futures::executor::block_on(graphics::Renderer::new(&window)).expect("Could not create graphics renderer");
    renderer.create_drawable_from_mesh(&player_mesh);
    renderer.create_drawable_from_mesh(&terrain_mesh);
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
            let _ = simulation.handle_frame(frame, &camera, &mut player);
        }

        match event {
            Event::RedrawRequested(_) => {
                // first rotate all vertices on 0,0,0 (rotate around origin), then translate all points towards location.
                renderer.update(player.pose(), identity(), identity());
                // for the view matrix we can also use player_move and player_rotate, and use the inverse of the resulting matrix
                let view = match camera {
                    camera::CameraType::FreeLook => simulation.freelook_camera.view(),
                    camera::CameraType::Follow => player.follow(),
                };
                futures::executor::block_on(renderer.render(view));
                let current_time = Instant::now();
                println!("fps: {}", (current_time - previous_time).as_millis());
                previous_time = current_time;
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::DeviceEvent {
                ref event,
                ..
            } => match event {
                MouseMotion { delta} => {
                    inputs.push_mouse_movement(delta);
                },
                _ => (),
            },
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::Resized(physical_size) => {
                    futures::executor::block_on(renderer.resize(*physical_size));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, ..} => {
                    futures::executor::block_on(renderer.resize(**new_inner_size));
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input,
                    ..
                } => {
                    match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
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
