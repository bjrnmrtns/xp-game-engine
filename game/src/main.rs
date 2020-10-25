use crate::window_input::InputHandler;
use game::{
    configuration, counter, from_config, graphics, process_input, scene, simulation, window_input,
    winit_impl,
};
use nalgebra_glm::{perspective, vec3};
use std::collections::HashMap;

const FPS: u64 = 60;

fn main() {
    let config = configuration::Config::load_config("config.ron");
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .expect("Could not create window");
    let mut winit_handler = winit_impl::WinitHandler::new();
    let mut graphics = futures::executor::block_on(graphics::Graphics::new(&window))
        .expect("Could not create graphics renderer");
    let meshes = from_config::create_model_meshes(config.models.as_slice());
    let (mapping, mut entities) = from_config::create_entities(config.entities.as_slice());
    let mut cameras = from_config::create_cameras(config.cameras.as_slice());

    for m in meshes {
        graphics.add_mesh_with_name(m.0, m.1.into_iter());
    }
    graphics.add_entities(mapping.as_slice());

    let mut frame_counter = counter::FrameCounter::new(FPS);
    let mut client = simulation::Client::new(1.0 / FPS as f64);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        if winit_handler.quit() {
            *control_flow = winit::event_loop::ControlFlow::Exit
        }
        match winit_handler.handle_event(&event, &window) {
            Some(window_input::WindowEvent::Redraw) => {
                cameras.toggle(winit_handler.get_camera_toggled() as usize);
                let (time_elapsed, frames) = frame_counter.frames();
                let selected_camera = cameras.get_selected();
                process_input::process_input(
                    winit_handler.get_input_state(),
                    frames,
                    1.0 / FPS as f32,
                    time_elapsed,
                    selected_camera,
                    entities.get_player().unwrap(),
                    &mut client,
                );

                let view = cameras.get_view(&entities.get_player().unwrap());

                let projection_3d = perspective(
                    graphics.sc_descriptor.width as f32 / graphics.sc_descriptor.height as f32,
                    45.0,
                    0.1,
                    10000.0,
                );

                let player_view_position =
                    if let Some(scene::Entity::Player { pose, .. }) = entities.get_player() {
                        scene::view_on(pose).1
                    } else {
                        assert!(false);
                        vec3(0.0, 0.0, 0.0)
                    };
                let time_before_render = std::time::Instant::now();
                let mut id_with_model = HashMap::new();
                id_with_model.extend(entities.entities.iter().map(|(id, e)| match e {
                    scene::Entity::Player { pose, .. } => (
                        *id,
                        xp_math::model_matrix(&pose.position, &pose.orientation),
                    ),
                    scene::Entity::Static { pose, .. } => (
                        *id,
                        xp_math::model_matrix(&pose.position, &pose.orientation),
                    ),
                }));
                graphics.render_loop(
                    id_with_model,
                    projection_3d.clone(),
                    view.clone(),
                    player_view_position,
                );
                let time_after_render = std::time::Instant::now();
                println!(
                    "render us: {}",
                    (time_after_render - time_before_render).as_micros()
                );
            }
            Some(window_input::WindowEvent::Resize(width, height)) => {
                futures::executor::block_on(graphics.resize(width, height))
            }
            None => (),
        }
    });
}
