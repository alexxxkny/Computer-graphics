use glium::glutin::{self};
use glutin::event::*;
use glium::{uniform, Surface};
use std::f32::consts::PI;
use shaders::*;
use transformations::*;

mod teapot;
mod shaders;
mod transformations;

pub struct UserDefinedParams {
    rotation: Rotation
}

pub struct Rotation {
    x: f32,
    y: f32,
    z: f32
}

impl Rotation {
    fn vec(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    fn mutate_x_normalized(&mut self, dif: f32) {
        self.x = (self.x + dif) % (2. * PI)
    }

    fn mutate_y_normalized(&mut self, dif: f32) {
        self.y = (self.y + dif) % (2. * PI)
    }

    fn mutate_z_normalized(&mut self, dif: f32) {
        self.z = (self.z + dif) % (2. * PI)
    }
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Flying teapot");
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES
    ).unwrap();

    let program = glium::Program::from_source(&display, VERTEX_SHADER, pixel_shaders::ROYAL_BLUE, None).unwrap();

    let mut user_params = UserDefinedParams {
        rotation: Rotation { x: 0.0, y: 0.0, z: 0.0 }
    };

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_millis(16);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match event {
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                WindowEvent::KeyboardInput { input,.. } => match input.state {
                    ElementState::Pressed => {
                        match input.virtual_keycode {
                            Some(keycode) => key_pressed(keycode, &mut user_params),
                            _ => ()
                        }
                    },
                    _ => ()
                },
                _ => return
            },
            _ => ()
        }

        let mut target = display.draw();

        let model = Model {
            position: [0.0, 0.0, 2.0],
            scale: 0.01
        };

        let (width, height) = target.get_dimensions();
        let perspective = Perspective {
            width: width as f32,
            height: height as f32,
            front_of_view: 3.141592 / 3.0,
            z_far: 1024.,
            z_near: 0.1
        };

        let u_light = [-1.0, 0.4, 0.9f32];

        let view = View {
            position: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 1.0],
            up: [0.0, 1.0, 0.0]
        };

        let uniform = uniform! {
            model: model.matrix(),
            perspective: perspective.matrix(),
            u_light: u_light,
            view: view.matrix(),
            rotation_angles: user_params.rotation.vec()
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
        target.draw((&positions, &normals), &indices, &program, &uniform, &params).unwrap();
        target.finish().unwrap();
    });
}

fn key_pressed(key: VirtualKeyCode, params: &mut UserDefinedParams) {
    let rotation: &mut Rotation = &mut params.rotation;
    match key {
        VirtualKeyCode::D => {
            rotation.mutate_y_normalized(-0.05);
        },
        VirtualKeyCode::A => {
            rotation.mutate_y_normalized(0.05);
        },
        VirtualKeyCode::S => {
            rotation.mutate_x_normalized(-0.05);
        },
        VirtualKeyCode::W => {
            rotation.mutate_x_normalized(0.05);
        },
        VirtualKeyCode::E => {
            rotation.mutate_z_normalized(-0.05);
        },
        VirtualKeyCode::Q => {
            rotation.mutate_z_normalized(0.05);
        },
        _ => ()
    }
}