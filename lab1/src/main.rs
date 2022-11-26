extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::conrod::{self, widget, UiCell, Colorable, Borderable};
use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::{FixedView};
use na::{Translation3, UnitQuaternion, Vector3, Point3};

use std::f32::consts::PI;
use std::path::Path;
use std::ops::Add;

use conrod::{Sizeable, Positionable, Labelable, Widget, widget_ids};

const UI_WIDTH_P: f64 = 150.;

const AXE_LENGTH_N: f32 = 0.24;

const X_INIT_POS_N: f32 = 0.05;
const Y_INIT_POS_N: f32 = -0.06;
const Z_INIT_POS_N: f32 = -0.4;

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

fn draw_axes(window: &mut Window) {
    let color = Point3::new(0.0, 0.0, 0.0);
    let init_shift = Vector3::new(X_INIT_POS_N, Y_INIT_POS_N, Z_INIT_POS_N);
    let init_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), Y_INIT_ROT);

    // Axes
    let half_axe = AXE_LENGTH_N / 2.0;

    window.set_line_width(1.0);
    window.draw_line(
        &init_rot.transform_point(&Point3::new(half_axe, 0.0, 0.0)).add(init_shift),
        &init_rot.transform_point(&Point3::new(-half_axe, 0.0, 0.0)).add(init_shift), 
        &color
    );
    window.draw_line(
        &Point3::new(0.0, half_axe, 0.0).add(init_shift), 
        &Point3::new(0.0, -half_axe, 0.0).add(init_shift), 
        &color
    );
    window.draw_line(
        &init_rot.transform_point(&Point3::new(0.0, 0.0, half_axe)).add(init_shift), 
        &init_rot.transform_point(&Point3::new(0.0, 0.0, -half_axe)).add(init_shift), 
        &color
    );

    // Labels
    let label_w = 0.01;
    let label_h = 0.015;
    let label_shift = 0.01;
    
    window.set_line_width(2.0);
    // X
    // /
    window.draw_line(
        &init_rot.transform_point(&Point3::new(half_axe + label_shift + label_w, 0.0, 0.0)).add(init_shift),
        &init_rot.transform_point(&Point3::new(half_axe + label_shift, label_h, 0.0)).add(init_shift), 
        &color
    );
    // \
    window.draw_line(
        &init_rot.transform_point(&Point3::new(half_axe + label_shift, 0.0, 0.0)).add(init_shift),
        &init_rot.transform_point(&Point3::new(half_axe + label_shift + label_w, label_h, 0.0)).add(init_shift), 
        &color
    );
    // Y
    let v_ratio = 0.55;
    // |
    window.draw_line(
        &Point3::new(0.0, half_axe + label_shift, 0.0).add(init_shift),
        &Point3::new(0.0, half_axe + label_shift + label_h * v_ratio, 0.0).add(init_shift), 
        &color
    );
    // /
    window.draw_line(
        &Point3::new(0.0,half_axe + label_shift + label_h * v_ratio, 0.0).add(init_shift),
        &Point3::new(label_w / 2.0, half_axe + label_shift + label_h, 0.0).add(init_shift), 
        &color
    );
    // \
    window.draw_line(
        &Point3::new(0.0,half_axe + label_shift + label_h * v_ratio, 0.0).add(init_shift),
        &Point3::new(-label_w / 2.0, half_axe + label_shift + label_h, 0.0).add(init_shift), 
        &color
    );
    // Z
    // _
    window.draw_line(
        &init_rot.transform_point(&Point3::new(0.0, 0.0, half_axe + label_shift + label_w)).add(init_shift),
        &init_rot.transform_point(&Point3::new(0.0, 0.0, half_axe + label_shift)).add(init_shift), 
        &color
    );
    // -
    window.draw_line(
        &init_rot.transform_point(&Point3::new(0.0, label_h, half_axe + label_shift + label_w)).add(init_shift),
        &init_rot.transform_point(&Point3::new(0.0, label_h, half_axe + label_shift)).add(init_shift), 
        &color
    );
    // /
    window.draw_line(
        &init_rot.transform_point(&Point3::new(0.0, 0.0, half_axe + label_shift + label_w)).add(init_shift),
        &init_rot.transform_point(&Point3::new(0.0, label_h, half_axe + label_shift)).add(init_shift), 
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

    // Teapot
    let obj_path = Path::new("./src/media/teapot.obj");
    let mtl_path = Path::new("./src/media");

    let mut teapot = window.add_obj(obj_path, mtl_path, Vector3::new(0.001, 0.001, 0.001));
    
    // UI
    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());

    while window.render_with_camera(&mut camera) {
    //while window.render() {
        teapot.set_local_translation(Translation3::new(X_INIT_POS_N, Y_INIT_POS_N, Z_INIT_POS_N));
        teapot.set_local_rotation(init_rot * rotation.x() * rotation.y() * rotation.z());

        draw_axes(&mut window);

        let mut ui_cell = window.conrod_ui_mut().set_widgets();
        draw_ui(&mut ui_cell, &ids, &mut rotation);
    }
}