use kiss3d::nalgebra as na;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::camera::{FixedView};
use kiss3d::conrod;
use na::{Translation3, Point3, Vector3, UnitQuaternion};

use std::ops::{Add};
use std::f32::consts::PI;

use conrod::{Borderable, Colorable, Sizeable, Positionable, Labelable, Widget, widget_ids, widget, UiCell};

mod support;
use support::*;

const UI_WIDTH_P: f64 = 150.;

const AXE_LENGTH_N: f32 = 0.24;

const X_INIT_POS_N: f32 = 0.05;
const Y_INIT_POS_N: f32 = -0.06;
const Z_INIT_POS_N: f32 = -0.4;

const Y_INIT_ROT: f32 = -45.0 / 180.0 * PI;

const CIRCLE_COLORS: [(f32, f32, f32); 4] = [
    (1.0, 0.0, 0.0),
    (0.0, 1.0, 0.0),
    (0.0, 0.0, 1.0),
    (1.0, 1.0, 0.0),
];

widget_ids! {
    pub struct Ids {
        canvas,
        slider_x,
        slider_y,
        slider_z,
        angle_x,
        angle_y,
        angle_z,
        point1_canvas,
        x1_dialog, y1_dialog, z1_dialog,
        point2_canvas,
        x2_dialog, y2_dialog, z2_dialog,
        point3_canvas,
        x3_dialog, y3_dialog, z3_dialog,
        point4_canvas,
        x4_dialog, y4_dialog, z4_dialog,
    }
}

fn draw_rotation_ui(ui_cell: &mut UiCell, ids: &Ids, rot: &mut Rotation) {
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
        .mid_bottom_with_margin_on(ids.canvas, 45.0)
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

fn draw_points_ui(ui_cell: &mut UiCell, ids: &Ids, vertices: &mut Vec<Point3<f32>>) {
    let point_canvas_height = 35.0;
    let point_canvas_width = 250.0;
    let point_canvas_gap = 15.0;

    let number_dialer_width = 65.0;
    let number_dialer_height = 25.0;
    let number_dialer_gap = 15.0;

    let canvas_ids = [
        ids.point1_canvas,
        ids.point2_canvas,
        ids.point3_canvas,
        ids.point4_canvas,
    ];

    let dialer_ids = [
        [ids.x1_dialog, ids.y1_dialog, ids.z1_dialog],
        [ids.x2_dialog, ids.y2_dialog, ids.z2_dialog],
        [ids.x3_dialog, ids.y3_dialog, ids.z3_dialog],
        [ids.x4_dialog, ids.y4_dialog, ids.z4_dialog],
    ];

    let dialer_labels = ["X", "Y", "Z"];

    for (canvas_index, canvas_id) in canvas_ids.iter().enumerate() {
        let canvas_vertice = &mut vertices[canvas_index];

        let canvas_widget = widget::Canvas::new()
            .h(point_canvas_height)
            .w(point_canvas_width)
            .rgb(1.0, 1.0, 1.0)
            .border_rgb(CIRCLE_COLORS[canvas_index].0, CIRCLE_COLORS[canvas_index].1, CIRCLE_COLORS[canvas_index].2);
            
        if canvas_index == 0 {
            canvas_widget
                .top_left_with_margin_on(ids.canvas, point_canvas_gap)
                .set(*canvas_id, ui_cell);
        } else {
            let prev_canvas_id = canvas_ids[canvas_index - 1];
            canvas_widget
                .down_from(prev_canvas_id, point_canvas_gap)
                .set(*canvas_id, ui_cell);
        }

        for dialer_index in 0..dialer_labels.len() {
            let to_dialer_format = |value: f32| {
                (value + AXE_LENGTH_N / 2.0) / AXE_LENGTH_N * 100.0
            };

            let dialer_value = match dialer_labels[dialer_index] {
                "X" => to_dialer_format(canvas_vertice.x),
                "Y" => to_dialer_format(canvas_vertice.y),
                "Z" => to_dialer_format(canvas_vertice.z),
                _ => 0.0
            };

            let mut dialer = widget::NumberDialer::new(dialer_value, 0.0, 100.0, 0)
                .w_h(number_dialer_width, number_dialer_height)
                .border_rgb(1.0, 1.0, 1.0)
                .label(dialer_labels[dialer_index]);

            let from_dialer_format = |value: f32| {
                value / 100.0 * AXE_LENGTH_N - AXE_LENGTH_N / 2.0
            };

            if dialer_index == 0 {
                dialer = dialer.mid_left_with_margin_on(*canvas_id, 10.0);
            } else {
                let prev_dialer_id = dialer_ids[canvas_index][dialer_index - 1];
                dialer = dialer.right_from(prev_dialer_id, number_dialer_gap);
            }

            for value in dialer.set(dialer_ids[canvas_index][dialer_index], ui_cell) {
                match dialer_labels[dialer_index] {
                    "X" => canvas_vertice.x = from_dialer_format(value),
                    "Y" => canvas_vertice.y = from_dialer_format(value),
                    "Z" => canvas_vertice.z = from_dialer_format(value),
                    _ => {}
                }
            }
        }
    }
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

fn move_points(points_spheres: &mut Vec<SceneNode>, vertices: &Vec<Point3<f32>>) {
    for (sphere, vertice) in points_spheres.iter_mut().zip(vertices.iter()) {
        sphere.set_local_translation(Translation3::new(vertice.x, vertice.y, vertice.z));
    }
}

fn main() {
    // Window
    let mut window = Window::new("Kiss3d: obj");
    let mut scene = window.add_group();
    window.set_light(Light::StickToCamera);
    window.set_background_color(1.0, 1.0, 1.0);

    // State
    let init_translation = Translation3::new(X_INIT_POS_N, Y_INIT_POS_N, Z_INIT_POS_N);
    let mut rotation = Rotation {x_angle: 0.0, y_angle: 0.0, z_angle: 0.0};
    let mut vertices = vec![
        Point3::new(0.1, 0.1, 0.0),
        Point3::new(-0.1, 0.0, 0.1),
        Point3::new(0.0, 0.1, -0.1),
        Point3::new(0.1, -0.1, 0.1),
    ];

    // Bilinear surface
    let points_count = 50;
    let mut quad = scene.add_quad(100.0, 100.0, points_count - 1, points_count - 1);
    quad.set_color(0.7, 0.3, 0.7);

    // Control points
    let radius = 0.004;
    let mut points_spheres = <Vec<SceneNode>>::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        let mut sphere = scene.add_sphere(radius);
        sphere.set_color(CIRCLE_COLORS[i].0, CIRCLE_COLORS[i].1, CIRCLE_COLORS[i].2);
        points_spheres.push(sphere);
    }

    // Camera
    let mut camera = FixedView::new();

    // UI
    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());
    
    while window.render_with_camera(&mut camera) {
    //while window.render() {
        let surface = BilinearSurface::new(&vertices);

        quad.modify_vertices(&mut |coords| {
            for (i, v) in coords.iter_mut().enumerate() {
                let u = (i % points_count + 1) as f32 / points_count as f32;
                let w = (i / points_count + 1) as f32 / points_count as f32;

                *v = surface.point(u, w);
            }
        });
        quad.recompute_normals();
        
        move_points(&mut points_spheres, &vertices);
        
        draw_axes(&mut window);
        
        let mut ui_cell = window.conrod_ui_mut().set_widgets();
        draw_rotation_ui(&mut ui_cell, &ids, &mut rotation);
        draw_points_ui(&mut ui_cell, &ids, &mut vertices);

        scene.set_local_translation(init_translation);
        scene.set_local_rotation(rotation.x() * rotation.y() * rotation.z());
    }
}