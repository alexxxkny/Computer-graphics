use kiss3d::event::{WindowEvent, MouseButton, Action};
use kiss3d::nalgebra as na;
use kiss3d::light::Light;
use kiss3d::scene::PlanarSceneNode;
use kiss3d::window::Window;
use kiss3d::camera::{FixedView};
use kiss3d::text::Font;
use kiss3d::ncollide3d;
use na::{Translation2, Point3, Point2, Vector2, OPoint};

use std::ops::{Add};

use coordinate_converter::CoordinateConverter;

mod coordinate_converter;

const AXE_LENGTH_N: f32 = 1.6;

const X_INIT_POS_N: f32 = 0.0;
const Y_INIT_POS_N: f32 = 0.0;

struct DragAndDrop {
    is_hovering: bool,
    mouse_pressed: bool,
    control_point_index: Option<u32>,
}

impl DragAndDrop {
    pub fn new() -> Self {
        DragAndDrop {
            is_hovering: false,
            mouse_pressed: false,
            control_point_index: None
        }
    }

    pub fn set_hovering(&mut self, point_index: Option<u32>) {
        if self.is_hovering && self.mouse_pressed {return}

        if let Some(point_index) = point_index {
            self.is_hovering = true;
            self.control_point_index = Some(point_index);
        } else {
            self.is_hovering = false;
            self.control_point_index = None;
        }
    }

    pub fn set_mouse_pressed(&mut self, mouse_pressed: bool) {
        self.mouse_pressed = mouse_pressed;
    }

    pub fn is_dragging(&self) -> Option<u32> {
        if self.mouse_pressed {
            self.control_point_index
        } else {
            None
        }
    }
}

fn draw_axes(window: &mut Window, length_normalized: f32, cc: &CoordinateConverter) {
    let color = Point3::new(0.0, 0.0, 0.0);
    let init_shift = Vector2::new(X_INIT_POS_N, Y_INIT_POS_N);

    let half_axe = length_normalized / 2.0;

    window.set_line_width(1.0);
    window.draw_planar_line(
        &Point2::new(cc.x_centered_n_to_p(half_axe), 0.0).add(init_shift),
        &Point2::new(cc.x_centered_n_to_p(-half_axe), 0.0).add(init_shift), 
        &color
    );
    window.draw_planar_line(
        &Point2::new(0.0, cc.y_centered_n_to_p(half_axe)).add(init_shift),
        &Point2::new(0.0, cc.y_centered_n_to_p(-half_axe)).add(init_shift), 
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
        &Point2::new(cc.x_centered_n_to_p(half_axe) + label_shift + label_w, 0.0).add(init_shift),
        &Point2::new(cc.x_centered_n_to_p(half_axe) + label_shift, label_h).add(init_shift), 
        &color
    );
    // \
    window.draw_planar_line(
        &Point2::new(cc.x_centered_n_to_p(half_axe) + label_shift, 0.0).add(init_shift),
        &Point2::new(cc.x_centered_n_to_p(half_axe) + label_shift + label_w, label_h).add(init_shift), 
        &color
    );
    // Y
    let v_ratio = 0.55;
    // |
    window.draw_planar_line(
        &Point2::new(0.0, cc.y_centered_n_to_p(half_axe) + label_shift).add(init_shift),
        &Point2::new(0.0, cc.y_centered_n_to_p(half_axe) + label_shift + label_h * v_ratio).add(init_shift), 
        &color
    );
    // /
    window.draw_planar_line(
        &Point2::new(0.0, cc.y_centered_n_to_p(half_axe) + label_shift + label_h * v_ratio).add(init_shift),
        &Point2::new(label_w / 2.0, cc.y_centered_n_to_p(half_axe) + label_shift + label_h).add(init_shift), 
        &color
    );
    // \
    window.draw_planar_line(
        &Point2::new(0.0, cc.y_centered_n_to_p(half_axe) + label_shift + label_h * v_ratio).add(init_shift),
        &Point2::new(-label_w / 2.0, cc.y_centered_n_to_p(half_axe) + label_shift + label_h).add(init_shift), 
        &color
    );
}

fn draw_point_coordinates(window: &mut Window, point: &Point2<f32>, circle_radius: f32, cc: &CoordinateConverter) {
    let text_color = Point3::new(0.0, 0.0, 0.0);
    let font = 35.0;
    let font_height = font / 3.0;
    let font_width = font_height / 2.0;
    let text = format!("{:.2} {:.2}", point.x, point.y);
    let text_shift = Vector2::new(-(text.len() as f32 / 2.0 * font_width), circle_radius + font_height + 10.0);
    
    let shifted_point = point.add(text_shift);
    let position = Point2::new(cc.x_centered_to_top_left_p(shifted_point.x), cc.y_centered_to_top_left_p(shifted_point.y));

    window.draw_text(
        &text, 
        &position,
        font, 
        &Font::default(), 
        &text_color
    );
}

fn is_point_in_circle(point: &Point2<f32>, circle_center: &Point2<f32>, radius: f32) -> bool {
    let x = point.x - circle_center.x;
    let y = point.y - circle_center.y;

    x.powi(2) + y.powi(2) <= radius.powi(2)
}

fn main() {
    // Window
    let mut window = Window::new("Kiss3d: obj");
    window.set_light(Light::StickToCamera);
    window.set_background_color(1.0, 1.0, 1.0);

    // Camera
    let mut camera = FixedView::new();

    // State
    let mut cursor = Point2::new(0.0, 0.0);

    // Settings
    let circle_radius = 10.0;
    let control_point_color = Point3::new(0.6, 0.0, 0.0);
    let control_line_color = Point3::new(0.6, 0.0, 0.0);
    let bezier_curve_color = Point3::new(1.0, 0.0, 0.0);

    // Control points
    let mut control_points_2d_n = vec![
        Point2::new(-0.4f32, -0.1),
        Point2::new(-0.2f32, 0.4),
        Point2::new(0.2f32, 0.4),
        Point2::new(0.4f32, -0.1),
    ];
    let control_points_count = control_points_2d_n.len();

    // Control point's circles
    let mut control_point_circles = <Vec<PlanarSceneNode>>::with_capacity(control_points_count);
    for _ in 0..control_points_count {
        let mut circle = window.add_circle(circle_radius);
        circle.set_color(control_point_color.x, control_point_color.y, control_point_color.z);
        control_point_circles.push(circle);
    }

    // Drag and drop helper
    let mut dd = DragAndDrop::new();
    
    while window.render_with_camera(&mut camera) {
        let window_width = window.width();
        let window_height = window.height();

        // Coordinate system helper
        let cc = CoordinateConverter::new(window_width, window_height);

        // Map control points to 2d centered coordinate system
        let control_points_2d: Vec<OPoint<f32, na::Const<2>>> = control_points_2d_n
            .iter()
            .map(|point| {
                Point2::new(
                    cc.x_centered_n_to_p(point.x),
                    cc.y_centered_n_to_p(point.y),
                )
            })
            .collect();

        // Get bezier curve points
        let control_points_3d: Vec<Point3<f32>> = control_points_2d_n
            .iter()
            .map(|point| {
                Point3::new(point.x, point.y, 0.0)
            })
            .collect();
        let bezier: Vec<OPoint<f32, na::Const<2>>> = ncollide3d::procedural::bezier_curve(&control_points_3d, 100)
        .iter()
        .map(|point| {
            Point2::new(
                cc.x_centered_n_to_p(point.x),
                cc.y_centered_n_to_p(point.y),
            )
        })
        .collect();

        // Check points hovering
        dd.set_hovering(None);
        for (index, point) in control_points_2d.iter().enumerate() {
            let hovering = is_point_in_circle(&cursor, &point, circle_radius);
            if hovering {
                dd.set_hovering(Some(index as u32));
                draw_point_coordinates(&mut window, point, circle_radius, &cc);
            }
        }

        // Proceed drag and drop
        if let Some(control_point_index) = dd.is_dragging() {
            control_points_2d_n[control_point_index as usize] = Point2::new(
                cc.x_centered_p_to_n(cursor.x),
                cc.y_centered_p_to_n(cursor.y),
            );
        }

        // Translate point circles
        for i in 0..control_points_2d.len() {
            let circle = &mut control_point_circles[i];
            let point = &control_points_2d[i];
            circle.set_local_translation(Translation2::new(point.x, point.y));
        }

        // Control lines
        //window.set_line_width(1.0);
        for i in 0..control_points_2d.len() - 1 {
            let start = control_points_2d[i];
            let end = control_points_2d[i + 1];
            window.draw_planar_line(&start, &end, &control_line_color);
        }

        // Bezier
        window.set_line_width(20.0);
        for i in 0..bezier.len() - 1 {
            window.draw_planar_line(&bezier[i], &bezier[i + 1], &bezier_curve_color);
        }

        draw_axes(&mut window, AXE_LENGTH_N, &cc);

        for event in window.events().iter() {
            match event.value {
                WindowEvent::CursorPos(x, y, _modif) => {
                    cursor = na::Point2::new(
                        cc.x_top_left_to_centered_p(x as f32), 
                        cc.y_top_left_to_centered_p(y as f32)
                    );
                },
                WindowEvent::MouseButton(btn, action ,_) => {
                    if let MouseButton::Button1 = btn {
                        if let Action::Press = action {
                            dd.set_mouse_pressed(true);
                        } else {
                            dd.set_mouse_pressed(false);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}