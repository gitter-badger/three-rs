use std::collections::HashSet;
use std::time;
use glutin;

use {Camera, Projection, Key, Scene};
use render::Renderer;
use factory::Factory;


struct Input {
    last_time: time::Instant,
    keys: HashSet<Key>,
    hit: HashSet<Key>,
    mouse_pos: (f32, f32), // normalized to NDC
}

pub struct Events {
    pub time_delta: f32,
    pub keys: HashSet<Key>,
    pub hit: HashSet<Key>,
    pub mouse_pos: (f32, f32),
}

pub struct Window {
    event_loop: glutin::EventsLoop,
    window: glutin::Window,
    input: Input,
    pub renderer: Renderer,
    pub factory: Factory,
    pub scene: Scene,
}

impl Window {
    pub fn new(title: &str, shader_path: &str) -> Self {
        let builder = glutin::WindowBuilder::new()
                             .with_title(title)
                             .with_vsync();
        let event_loop = glutin::EventsLoop::new();
        let (renderer, window, mut factory) = Renderer::new(builder, &event_loop, shader_path);
        let scene = factory.scene();
        Window {
            event_loop,
            window,
            input: Input {
                last_time: time::Instant::now(),
                keys: HashSet::new(),
                hit: HashSet::new(),
                mouse_pos: (0.0, 0.0),
            },
            renderer,
            factory,
            scene,
        }
    }

    pub fn update(&mut self) -> Option<Events> {
        let mut running = true;
        let renderer = &mut self.renderer;
        let input = &mut self.input;
        input.hit.clear();

        self.window.swap_buffers().unwrap();
        let window = &self.window;

        self.event_loop.poll_events(|glutin::Event::WindowEvent {event, ..}| {
            use glutin::ElementState::*;
            use glutin::WindowEvent::*;
            use glutin::VirtualKeyCode as Key;
            match event {
                Resized(..) => {
                    renderer.resize(window);
                }
                KeyboardInput(_, _, Some(Key::Escape), _) |
                Closed => {
                    running = false
                }
                KeyboardInput(Pressed, _, Some(key), _) => {
                    input.keys.insert(key);
                    input.hit.insert(key);
                }
                KeyboardInput(Released, _, Some(key), _) => {
                    input.keys.remove(&key);
                }
                MouseMoved(x, y) => {
                    input.mouse_pos = renderer.map_to_ndc(x, y);
                }
                _ => ()
            }
        });

        if running {
            let now = time::Instant::now();
            let dt = now - input.last_time;
            input.last_time = now;
            Some(Events {
                time_delta: dt.as_secs() as f32 + 1e-9 * dt.subsec_nanos() as f32,
                keys: input.keys.clone(),
                hit: input.hit.clone(),
                mouse_pos: input.mouse_pos,
            })
        } else {
            None
        }
    }

    pub fn render<P: Projection>(&mut self, camera: &Camera<P>) {
        self.renderer.render(&self.scene, camera);
    }
}
