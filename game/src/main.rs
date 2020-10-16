use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

use game::client::local_client::LocalClient;
use game::client::recording;
use game::command_queue::CommandQueue;
use game::configuration::Config;
use game::entities::{Entities, EntityType};
use game::graphics::clipmap;
use game::window_input::input_handler::InputHandler;
use game::*;
use nalgebra_glm::{ortho, perspective, quat_identity, vec3, Mat4};
use std::collections::HashMap;
use winit::event::DeviceEvent::MouseMotion;
use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
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

pub struct UIContext {
    pub ui_enabled: bool,
    pub camera: camera::CameraType,
}

const FPS: u64 = 60;

fn game(options: Options) {
    let config = Config::load_config("config.ron");
    let mut game_state = UIContext {
        ui_enabled: false,
        camera: camera::CameraType::Follow,
    };
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Could not create window");
    let mut winit_handler = winit_impl::WinitHandler::new();

    let meshes: Vec<(String, xp_mesh::mesh::Obj)> = config
        .models
        .iter()
        .map(|model| {
            (
                model.name.clone(),
                xp_mesh::mesh::Obj::load(&model.location).unwrap(),
            )
        })
        .collect();

    /*    let mut ui = UI::<UIContext, u32>::new(
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
            camera::CameraType::FreeLook => context.camera = camera::CameraType::Follow,
        },
    );

    ui.layout();
     */
    let mut freelook = camera::FreeLook::new(vec3(0.0, 3.0, 3.0), vec3(0.0, -1.0, -1.0));
    let mut graphics = futures::executor::block_on(graphics::Graphics::new(&window))
        .expect("Could not create graphics renderer");
    let mut renderers = futures::executor::block_on(graphics::Renderers::new(
        &graphics.device,
        &graphics.queue,
        &graphics.sc_descriptor,
    ))
    .expect("Could not create graphics renderer");
    for m in meshes {
        renderers
            .default
            .add_mesh_with_name2(&graphics.device, m.0, m.1.into_iter());
    }
    let mut entities = Entities::new();
    for config_entity in config.entities {
        let id = entities.add(
            config_entity.entity_type,
            config_entity.start_position.into(),
            quat_identity(),
            config_entity.max_velocity,
        );
        renderers.default.add_entity(id, &config_entity.model_name);
    }

    let mut previous_time = Instant::now();

    let mut commands_queue = CommandQueue::new();
    let mut client = LocalClient::new();
    let mut record = recording::try_create_recorder(options.record_path);
    let replaying = options.replay_path != None;
    let mut replay = recording::try_create_replayer(options.replay_path);

    let mut frame_counter = counter::FrameCounter::new(FPS);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        frame_counter.run();
        let frame_commands = commands_queue
            .input_to_commands(winit_handler.get_input_state(), frame_counter.count());
        simulation::handle_frame(
            frame_commands,
            &mut entities,
            1.0 / FPS as f32,
            &renderers.clipmap,
        );

        match event {
            Event::RedrawRequested(_) => {
                // first rotate all vertices on 0,0,0 (rotate around origin), then translate all points towards location.
                // for the view matrix we can also use player_move and player_rotate, and use the inverse of the resulting matrix
                let view = match game_state.camera {
                    camera::CameraType::FreeLook => freelook.view(),
                    camera::CameraType::Follow => {
                        camera::view_on(entities.get_first_entity_with(EntityType::Player).unwrap())
                            .0
                    }
                };
                /*
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
                );*/
                let projection_3d = perspective(
                    graphics.sc_descriptor.width as f32 / graphics.sc_descriptor.height as f32,
                    45.0,
                    0.1,
                    10000.0,
                );

                let time_before_clipmap_update = std::time::Instant::now();
                renderers.clipmap.pre_render(
                    &graphics.queue,
                    clipmap::Uniforms {
                        projection: projection_3d.clone() as Mat4,
                        view: view.clone() as Mat4,
                        camera_position: camera::view_on(
                            entities.get_first_entity_with(EntityType::Player).unwrap(),
                        )
                        .1, //simulation.freelook_camera.position,
                    },
                );
                let time_after_clipmap_update = std::time::Instant::now();
                /*   renderers
                                    .ui
                                    .create_drawable(&graphics.device, Some(graphics::ui::create_mesh(&ui)));
                                renderers.ui.pre_render(
                                    &graphics.queue,
                                    graphics::ui::Uniforms {
                                        projection: projection_2d,
                                    },
                                    game_state.ui_enabled,
                                );
                */
                let target = &graphics
                    .swap_chain
                    .get_current_frame()
                    .expect("failed to get next texture")
                    .output
                    .view;
                let time_before_render = std::time::Instant::now();
                let mut id_with_model = HashMap::new();
                id_with_model.extend(
                    entities
                        .get_entities()
                        .iter()
                        .map(|e| (e.id, model_matrix(&e.position, &e.orientation))),
                );
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
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                winit_handler.handle_event(&event);
                match event {
                    #[allow(deprecated)]
                    WindowEvent::Resized(physical_size) => {
                        //ui.update_window_size(physical_size.width as f32, physical_size.height as f32);
                        futures::executor::block_on(graphics.resize(*physical_size));
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        /*ui.update_window_size(
                            new_inner_size.width as f32,
                            new_inner_size.height as f32,
                        );*/
                        futures::executor::block_on(graphics.resize(**new_inner_size));
                    }
                    _ => (),
                }
            }
            _ => {}
        }
    });
}

fn main() {
    let options = Options::from_args();
    game(options)
}
