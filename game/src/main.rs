use std::path::PathBuf;
use structopt::StructOpt;

use game::client::recording;
use game::configuration::Camera::{Follow, Freelook};
use game::configuration::Config;
use game::graphics::clipmap;
use game::process_input::process_input;
use game::scene;
use game::window_input::input_handler::InputHandler;
use game::*;
use nalgebra_glm::{identity, perspective, quat_identity, vec3, Mat4};
use std::collections::HashMap;
use window_input::WindowEvent;
use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use xp_math::model_matrix;
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

const FPS: u64 = 60;

fn game(options: Options) {
    let config = Config::load_config("config.ron");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Could not create window");
    let mut winit_handler = winit_impl::WinitHandler::new();

    let mut graphics = futures::executor::block_on(graphics::Graphics::new(&window))
        .expect("Could not create graphics renderer");
    let mut renderers = futures::executor::block_on(graphics::Renderers::new(
        &graphics.device,
        &graphics.queue,
        &graphics.sc_descriptor,
    ))
    .expect("Could not create graphics renderer");
    let meshes = from_config::create_model_meshes(config.models.as_slice());
    let (mapping, mut entities) = from_config::create_entities(config.entities.as_slice());
    let mut cameras = from_config::create_cameras(config.cameras.as_slice());

    for m in meshes {
        renderers
            .default
            .add_mesh_with_name(&graphics.device, m.0, m.1.into_iter());
    }
    renderers.default.add_entities(mapping.as_slice());

    let mut last_frame: Option<u64> = None;
    let mut frame_counter = counter::FrameCounter::new(FPS);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match winit_handler.handle_event(&event, &window) {
            Some(WindowEvent::Redraw) => {
                cameras.toggle(winit_handler.get_camera_toggled() as usize);
                frame_counter.run();
                let current_frame = frame_counter.count();
                let selected_camera = cameras.get_selected();
                let frame_commands = process_input(
                    winit_handler.get_input_state(),
                    last_frame,
                    current_frame,
                    selected_camera,
                );
                last_frame = Some(current_frame);

                simulation::handle_frame(
                    frame_commands,
                    entities.get_player().unwrap(),
                    1.0 / FPS as f32,
                    &renderers.clipmap,
                );

                let view = if let Some(scene::Entity::Player { pose, .. }) = entities.get_player() {
                    scene::view_on(&pose).0
                } else {
                    identity()
                };
                let projection_3d = perspective(
                    graphics.sc_descriptor.width as f32 / graphics.sc_descriptor.height as f32,
                    45.0,
                    0.1,
                    10000.0,
                );

                let time_before_clipmap_update = std::time::Instant::now();
                if let Some(scene::Entity::Player { pose, .. }) = entities.get_player() {
                    renderers.clipmap.pre_render(
                        &graphics.queue,
                        clipmap::Uniforms {
                            projection: projection_3d.clone() as Mat4,
                            view,
                            camera_position: scene::view_on(pose).1,
                        },
                    );
                }
                let time_after_clipmap_update = std::time::Instant::now();
                let target = &graphics
                    .swap_chain
                    .get_current_frame()
                    .expect("failed to get next texture")
                    .output
                    .view;
                let time_before_render = std::time::Instant::now();
                let mut id_with_model = HashMap::new();
                id_with_model.extend(entities.entities.iter().map(|(id, e)| match e {
                    scene::Entity::Player { pose, .. } => {
                        (*id, model_matrix(&pose.position, &pose.orientation))
                    }
                    scene::Entity::Static { pose, .. } => {
                        (*id, model_matrix(&pose.position, &pose.orientation))
                    }
                }));
                graphics::render_loop(
                    &renderers,
                    id_with_model,
                    projection_3d.clone(),
                    view.clone(),
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
            Some(WindowEvent::Resize(width, height)) => {
                futures::executor::block_on(graphics.resize(width, height))
            }
            None => (),
        }
    });
}

fn main() {
    let options = Options::from_args();
    game(options)
}
