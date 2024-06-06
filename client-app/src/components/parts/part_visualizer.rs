use std::collections::HashMap;

use three_d::{
    degrees, pick, vec3, AmbientLight, Camera, ClearState, DirectionalLight, Event,
    FrameInputGenerator, Model, MouseButton, OrbitControl, PhysicalMaterial, Srgba, Viewport,
    WindowedContext,
};
use three_d_asset::AxisAlignedBoundingBox;
use winit::window::{WindowBuilder, WindowId};

pub(crate) struct Scene {
    camera: Camera,
    part: Model<PhysicalMaterial>,
    control: OrbitControl,
}

// #[component]
pub fn part_window(window_builders_and_models: Vec<(WindowBuilder, three_d_asset::Model)>) {
    // -- variables -- //

    // let canvas_ref = create_node_ref::<Canvas>();
    // let is_visible = use_element_visibility(canvas_ref);

    let event_loop = winit::event_loop::EventLoop::new();
    let windows: HashMap<
        WindowId,
        (
            winit::window::Window,
            WindowedContext,
            FrameInputGenerator,
            Scene,
        ),
    > = window_builders_and_models
        .into_iter()
        .map(|(window_builder, model)| {
            let window = window_builder.build(&event_loop).unwrap();
            let context = WindowedContext::from_winit_window(
                &window,
                three_d::SurfaceSettings {
                    vsync: false, // Wayland hangs in swap_buffers when one window is minimized or occluded
                    ..three_d::SurfaceSettings::default()
                },
            )
            .unwrap();

            let mut model = model.clone();
            let part = Model::<PhysicalMaterial>::new(&context, &model).unwrap();

            let bounding_boxes: Vec<AxisAlignedBoundingBox> = model
                .geometries
                .iter_mut()
                .map(|geometry| geometry.compute_aabb())
                .collect();
            let bounding_box = bounding_boxes[0];

            let center = bounding_box.center();

            let max_vec = bounding_box.max();
            let min_vec = bounding_box.min();
            let dist = f32::sqrt(
                (max_vec.x - min_vec.x).powf(2.0)
                    + (max_vec.y - min_vec.y).powf(2.0)
                    + (max_vec.z - min_vec.z).powf(2.0),
            );

            let position = vec3(dist, dist, dist);

            let camera = Camera::new_perspective(
                Viewport::new_at_origo(0, 0),
                position,
                center,
                vec3(0.0, 1.0, 0.0),
                degrees(45.0),
                0.01,
                dist * 1000.0,
            );

            let control = OrbitControl::new(*camera.target(), dist / 4.0, dist * 2.5);

            let frame_input_generator = three_d::FrameInputGenerator::from_winit_window(&window);

            (
                window.id(),
                (
                    window,
                    context,
                    frame_input_generator,
                    Scene {
                        camera,
                        part,
                        control,
                    },
                ),
            )
        })
        .collect();

    renderer(event_loop, windows);

    // view! {
    // <canvas id=canvas_id class="bg-red-400 h-64 w-full rounded" ref=canvas_ref/>
    // }
}

fn renderer(
    event_loop: winit::event_loop::EventLoop<()>,
    mut windows: HashMap<
        WindowId,
        (
            winit::window::Window,
            WindowedContext,
            FrameInputGenerator,
            Scene,
        ),
    >,
) {
    event_loop.run(move |event, _, control_flow| match &event {
        winit::event::Event::MainEventsCleared => {
            for (window, _, _, _) in windows.values() {
                window.request_redraw();
            }
        }
        winit::event::Event::RedrawRequested(window_id) => {
            if let Some((window, context, frame_input_generator, scene)) =
                windows.get_mut(window_id)
            {
                context.make_current().unwrap();
                let mut frame_input = frame_input_generator.generate(context);

                scene.camera.set_viewport(frame_input.viewport);
                scene.part.animate(frame_input.accumulated_time as f32);

                let ambient = AmbientLight::new(&context, 0.4, Srgba::WHITE);
                let directional1 =
                    DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(-1.0, -1.0, -1.0));
                let directional2 =
                    DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(1.0, -1.0, -1.0));
                let directional3 =
                    DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(-1.0, -1.0, 1.0));
                let directional4 =
                    DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(1.0, -1.0, 1.0));

                let mut change = frame_input.first_frame;
                change |= scene.camera.set_viewport(frame_input.viewport);

                for event in frame_input.events.iter() {
                    if let Event::MousePress {
                        button, position, ..
                    } = *event
                    {
                        if button == MouseButton::Left {
                            if let Some(_) = pick(&context, &scene.camera, position, &scene.part) {
                                change = true;
                            }
                        }
                    }
                }

                change |= scene
                    .control
                    .handle_events(&mut scene.camera, &mut frame_input.events);

                // draw
                if change {
                    frame_input
                        .screen()
                        .clear(ClearState::color_and_depth(0.65, 0.65, 0.65, 1.0, 1.0))
                        .render(
                            &scene.camera,
                            scene.part.into_iter(),
                            &[
                                &ambient,
                                &directional1,
                                &directional2,
                                &directional3,
                                &directional4,
                            ],
                        );
                }

                context.swap_buffers().unwrap();
                control_flow.set_poll();
                window.request_redraw();
            }
        }

        winit::event::Event::WindowEvent { event, window_id } => {
            if let Some((_, context, frame_input_generator, _)) = windows.get_mut(window_id) {
                frame_input_generator.handle_winit_window_event(event);
                match event {
                    winit::event::WindowEvent::Destroyed => {
                        log::info!("Window destroyed!");
                    }
                    winit::event::WindowEvent::Resized(physical_size) => {
                        context.resize(*physical_size);
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        context.resize(**new_inner_size);
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        if let Some((_, context, _, _)) = windows.get_mut(window_id) {
                            context.make_current().unwrap();
                        }

                        windows.remove(window_id);

                        if windows.is_empty() {
                            control_flow.set_exit();
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => {}
    });
}
