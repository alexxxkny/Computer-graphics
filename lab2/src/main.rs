extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::conrod::{self, widget, UiCell, Colorable, Borderable};
use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::{FixedView};
use kiss3d::ncollide3d;
use na::{Translation3, UnitQuaternion, Vector3, Point3, Point2, Const, Translation2, Vector2};

use std::f32::consts::PI;
use std::path::Path;
use std::ops::Add;

use conrod::{Sizeable, Positionable, Labelable, Widget, widget_ids};

const UI_WIDTH_P: f64 = 150.;

const AXE_LENGTH_N: f32 = 1.6;

const X_INIT_POS_N: f32 = 0.0;
const Y_INIT_POS_N: f32 = 0.0;
const Z_INIT_POS_N: f32 = -2.0;

const Y_INIT_ROT: f32 = -45.0 / 180.0 * PI;

struct Rotation {
    x_angle: f32,
    y_angle: f32,
    z_angle: f32,
}

impl Rotation {
    fn x(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.x_angle / 180.0 * PI)
    }

    fn y(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.y_angle / 180.0 * PI)
    }

    fn z(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_axis_angle(&Vector3::z_axis(), self.z_angle / 180.0 * PI)
    }
}

// Struct to transform top-left coordinate system to centered
struct CenteredCS {
    window_width: u32,
    window_height: u32,
    axe_length: f32,
}

impl CenteredCS {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        CenteredCS { window_width, window_height, axe_length: 2.0 }
    }

    fn belongs_n(&self, coordinate_normalized: f32) -> bool {
        -self.axe_length / 2.0 <= coordinate_normalized && coordinate_normalized <= self.axe_length / 2.0
    }

    fn x_belongs_p(&self, x_pixels: f32) -> bool {
        -(self.window_width as f32 / 2.0) <= x_pixels && x_pixels <= self.window_width as f32 / 2.0
    }

    fn y_belongs_p(&self, y_pixels: f32) -> bool {
        -(self.window_height as f32 / 2.0) <= y_pixels && y_pixels <= self.window_height as f32 / 2.0
    }

    fn ensure_belongs_n(&self, coordinate_normalized: f32) -> Result<(), &str> {
        if self.belongs_n(coordinate_normalized) {
            Ok(())
        } else {
            Err("Coordinate value does not belong to coordinates system")
        }
    }

    fn ensure_x_belongs_p(&self, x_pixels: f32) -> Result<(), &str> {
        if self.x_belongs_p(x_pixels) {
            Ok(())
        } else {
            Err("Coordinate value does not belong to coordinates system")
        }
    }

    fn ensure_y_belongs_p(&self, y_pixels: f32) -> Result<(), &str> {
        if self.y_belongs_p(y_pixels) {
            Ok(())
        } else {
            Err("Coordinate value does not belong to coordinates system")
        }
    }

    pub fn x_p(&self, x_normalized: f32) -> f32 {
        self.ensure_belongs_n(x_normalized).unwrap();

        x_normalized * (self.window_width as f32) / 2.0
    }

    pub fn y_p(&self, y_normalized: f32) -> f32 {
        self.ensure_belongs_n(y_normalized).unwrap();
        
        y_normalized * (self.window_height as f32) / 2.0
    }

    pub fn x_n(&self, x_pixels: f32) -> f32 {
        self.ensure_x_belongs_p(x_pixels).unwrap();

        x_pixels - (self.window_width as f32) / 2.0
    }

    pub fn y_n(&self, y_pixels: f32) -> f32 {
        self.ensure_y_belongs_p(y_pixels).unwrap();
        
        y_pixels - (self.window_height as f32) / 2.0
    }

    pub fn width_n_to_p(&self, width_normalized: f32) -> f32 {
        assert!(width_normalized >= 0.0 && width_normalized <= self.axe_length, "Given width is out of the scope!");

        self.window_width as f32 * width_normalized
    }
}

widget_ids! {
    pub struct Ids {
        canvas,
        slider_x,
        slider_y,
        slider_z,
        angle_x,
        angle_y,
        angle_z,
    }
}

fn draw_ui(ui_cell: &mut UiCell, ids: &Ids, rot: &mut Rotation) {
    let slider_w_p = 16.0;
    let slider_h_p = 180.0;
    let sliders_gap_p = 33.0;
    let font_size = 11;

    widget::Canvas::new()
        .align_left()
        .w(UI_WIDTH_P)
        .rgb(1.0, 1.0, 1.0)
        .border_rgb(1.0, 1.0, 1.0)
        .set(ids.canvas, ui_cell);

    for v in widget::Slider::new(rot.x_angle, -180.0, 180.0)
        .label("X")
        .w(slider_w_p)
        .h(slider_h_p)
        .left_from(ids.slider_y, sliders_gap_p)
        .set(ids.slider_x, ui_cell) 
    {
        rot.x_angle = v;
    }

    widget::Text::new(&format!("{:.1}°", rot.x_angle))
        .font_size(font_size)
        .down_from(ids.slider_x, 10.)
        .align_middle_x_of(ids.slider_x)
        .set(ids.angle_x, ui_cell);

    for v in widget::Slider::new(rot.y_angle, -180.0, 180.0)
        .label("Y")
        .w(slider_w_p)
        .h(slider_h_p)
        .middle_of(ids.canvas)
        .set(ids.slider_y, ui_cell) 
    {
        rot.y_angle = v;
    }

    widget::Text::new(&format!("{:.1}°", rot.y_angle))
        .font_size(font_size)
        .down_from(ids.slider_y, 10.)
        .align_middle_x_of(ids.slider_y)
        .set(ids.angle_y, ui_cell);

    for v in widget::Slider::new(rot.z_angle, -180.0, 180.0)
        .label("Z")
        .w(slider_w_p)
        .h(slider_h_p)
        .right_from(ids.slider_y, sliders_gap_p)
        .set(ids.slider_z, ui_cell) 
    {
        rot.z_angle = v;
    }

    widget::Text::new(&format!("{:.1}°", rot.z_angle))
        .font_size(font_size)
        .down_from(ids.slider_z, 10.)
        .align_middle_x_of(ids.slider_z)
        .set(ids.angle_z, ui_cell);
}

fn draw_axes(window: &mut Window, length_normalized: f32, cs: &CenteredCS) {
    let color = Point3::new(0.0, 0.0, 0.0);
    let init_shift = Vector2::new(X_INIT_POS_N, Y_INIT_POS_N);

    let half_axe = length_normalized / 2.0;

    window.set_line_width(1.0);
    window.draw_planar_line(
        &Point2::new(cs.x_p(half_axe), 0.0).add(init_shift),
        &Point2::new(cs.x_p(-half_axe), 0.0).add(init_shift), 
        &color
    );
    window.draw_planar_line(
        &Point2::new(0.0, cs.y_p(half_axe)).add(init_shift),
        &Point2::new(0.0, cs.y_p(-half_axe)).add(init_shift), 
        &color
    );

    // Labels
    let label_w = 20.0;
    let label_h = 30.0;
    let label_shift = 20.0;
    
    window.set_line_width(2.0);
    // X
    // /
    window.draw_planar_line(
        &Point2::new(cs.x_p(half_axe) + label_shift + label_w, 0.0).add(init_shift),
        &Point2::new(cs.x_p(half_axe) + label_shift, label_h).add(init_shift), 
        &color
    );
    // \
    window.draw_planar_line(
        &Point2::new(cs.x_p(half_axe) + label_shift, 0.0).add(init_shift),
        &Point2::new(cs.x_p(half_axe) + label_shift + label_w, label_h).add(init_shift), 
        &color
    );
    // Y
    let v_ratio = 0.55;
    // |
    window.draw_planar_line(
        &Point2::new(0.0, cs.y_p(half_axe) + label_shift).add(init_shift),
        &Point2::new(0.0, cs.y_p(half_axe) + label_shift + label_h * v_ratio).add(init_shift), 
        &color
    );
    // /
    window.draw_planar_line(
        &Point2::new(0.0, cs.y_p(half_axe) + label_shift + label_h * v_ratio).add(init_shift),
        &Point2::new(label_w / 2.0, cs.y_p(half_axe) + label_shift + label_h).add(init_shift), 
        &color
    );
    // \
    window.draw_planar_line(
        &Point2::new(0.0, cs.y_p(half_axe) + label_shift + label_h * v_ratio).add(init_shift),
        &Point2::new(-label_w / 2.0, cs.y_p(half_axe) + label_shift + label_h).add(init_shift), 
        &color
    );
}

fn main() {
    // Window
    let mut window = Window::new("Kiss3d: obj");
    window.set_light(Light::StickToCamera);
    window.set_background_color(1.0, 1.0, 1.0);

    // Camera
    let mut camera = FixedView::new();

    // State
    let init_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), Y_INIT_ROT);
    let mut rotation = Rotation {x_angle: 0.0, y_angle: 0.0, z_angle: 0.0};
    
    // UI
    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());

    let control_points = [
        Point3::new(-0.4f32, -0.1, Z_INIT_POS_N),
        Point3::new(-0.2f32, 0.4, Z_INIT_POS_N),
        Point3::new(0.2f32, 0.4, Z_INIT_POS_N),
        Point3::new(0.4f32, -0.1, Z_INIT_POS_N),
    ];

    let bezier = ncollide3d::procedural::bezier_curve(&control_points, 100);
    
    while window.render_with_camera(&mut camera) {
        let window_width = window.width();
        let window_height = window.height();

        let cs = CenteredCS::new(window_width, window_height);

        window.draw_planar_line(
            &Point2::new(100f32, 0.0), 
            &Point2::new(-100f32, 0.0), 
            &Point3::new(0.0, 0.0, 0.0)
        );

        window.draw_planar_line(
            &Point2::new(0.0f32, 0.5), 
            &Point2::new(0.0f32, -0.5), 
            &Point3::new(0.0, 0.0, 0.0)
        );
    
        for i in 0..control_points.len() - 1 {
            window.draw_line(&control_points[i], &control_points[i + 1], &Point3::new(1.0, 0.0, 0.0));
        }

        for point in control_points {
            let mut circle = window.add_sphere(0.02);
            circle.set_local_translation(Translation3::new(point.x, point.y, point.z));
        }

        for i in 0..bezier.len() - 1 {
            window.draw_line(&bezier[i], &bezier[i + 1], &Point3::new(1.0, 0.0, 0.0));
        }

        draw_axes(&mut window, AXE_LENGTH_N, &cs);

        // let mut ui_cell = window.conrod_ui_mut().set_widgets();
        // draw_ui(&mut ui_cell, &ids, &mut rotation);
    }
}