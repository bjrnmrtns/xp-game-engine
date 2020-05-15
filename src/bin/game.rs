use std::fs::File;
use std::io::BufReader;
use std::time::{Instant};
use std::path::PathBuf;
use structopt::StructOpt;

use xp::{*, command_queue::CommandQueue, obj};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{WindowEvent, ElementState, VirtualKeyCode, Event, KeyboardInput};
use winit::window::WindowBuilder;
use nalgebra_glm::{rotate, identity, vec3};
use winit::event::DeviceEvent::MouseMotion;
use std::collections::{HashSet};

#[derive(Debug, StructOpt)]
#[structopt(name = "options", about = "command line options")]
pub struct Options {
    #[structopt(long = "recording", parse(from_os_str))]
    record_path: Option<PathBuf>,

    #[structopt(long = "replay", parse(from_os_str))]
    replay_path: Option<PathBuf>,
}

fn ensure_unique_provoking_vertices(vertices: &[[f32; 3]], indices: &[u32]) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut new_vertices= vertices.to_vec();
    let mut new_indices = indices.to_vec();
    let mut provs_used: HashSet<u32> = HashSet::new();
    for face in indices.chunks(3).enumerate() {
        // first vertex of face is a provoking vertex
        if provs_used.contains(&face.1[0]) {
            new_vertices.push(vertices[face.1[0] as usize].clone());
            new_indices[&face.0 * 3] = new_vertices.len() as u32 - 1;
        } else {
            provs_used.insert(face.1[0]);
        }
    }
    (new_vertices, new_indices)
}

fn game(options: Options) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop).expect("Could not create window");

    let obj_file_name = "obj/ah/african_head.obj";
    let obj_file = &mut BufReader::new(File::open(obj_file_name).expect(format!("Could not open obj file: {}", obj_file_name).as_str()));
    let (vertices, indices) = obj::parse_obj(obj_file).expect(format!("Could not parse obj file: {}", obj_file_name).as_str());
    let (vertices, indices) = ensure_unique_provoking_vertices(vertices.as_slice(), indices.as_slice());
    let mesh = graphics::Mesh { vertices: vertices.iter().enumerate().map(|v| graphics::Vertex { position: *v.1, color_id: (v.0 % 3) as u32, }).collect(), indices, };
    let mut renderer = futures::executor::block_on(graphics::Renderer::new(&window, &mesh)).expect("Could not create graphics renderer");

    let mut previous_time = Instant::now();
    let mut rot: f32 = 0.0;

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
            let _ = simulation.handle_frame(frame);
        }

        match event {
            Event::RedrawRequested(_) => {
                rot = rot + 0.01;
                let model = rotate(&identity(), rot, &vec3(0.0, 1.0, 0.0));
                renderer.update(model);
                futures::executor::block_on(renderer.render(&camera::view(&simulation.camera_position, &simulation.camera_direction)));
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
