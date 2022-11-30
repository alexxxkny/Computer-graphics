use kiss3d::event::{WindowEvent, MouseButton};
use kiss3d::nalgebra as na;
use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::{FixedView};
use kiss3d::conrod::{self, Sizeable, Colorable};
use na::{Point2, Point3};

use std::ops::Range;

use conrod::{
    widget_ids, 
    UiCell, 
    widget, Widget,
    Positionable, Borderable, Labelable
};

use lazy_static::lazy_static;

use coordinate_converter::CoordinateConverter;
use common::*;
use rand::Rng;
use selection::*;

mod coordinate_converter;
mod common;
mod selection;

lazy_static! {
    static ref LINE_COLOR: Point3<f32> = Point3::new(0.6, 0.6, 0.6);
    static ref SELECTED_LINE_COLOR: Point3<f32> = Point3::new(1.0, 0.0, 0.0);
}

struct LinesManager {
    lines_count: u32,
    lines: Vec<Line>,
    x_range: Range<f32>,
    y_range: Range<f32>
}

impl LinesManager {
    pub fn new() -> Self {
        Self {
            lines_count: 0,
            lines: vec![],
            x_range: 0.0..0.0,
            y_range: 0.0..0.0
        }
    }

    pub fn set_draw_area_size(&mut self, width: f32, height: f32) {
        let x_abs = width / 2.0;
        let y_abs = height / 2.0;
        self.x_range = -x_abs..x_abs;
        self.y_range = -y_abs..y_abs;
    }

    pub fn set_lines_count(&mut self, lines_count: u32) {
        self.lines_count = lines_count;
        self.generate_lines();
    }

    pub fn draw(&mut self, window: &mut Window) {
        for line in &self.lines {
            window.draw_planar_line(&line.0, &line.1, &LINE_COLOR);
        }
    }

    pub fn draw_with_selection_check(&mut self, window: &mut Window, selection: RectangleSelection) {
        for line in &self.lines {
            match selection.clipping_check(line) {
                LineClipping::Inside => {
                    window.draw_planar_line(&line.0, &line.1, &SELECTED_LINE_COLOR);
                },
                LineClipping::PartlyInside(inside_line_part) => {
                    window.draw_planar_line(&line.0, &line.1, &LINE_COLOR);
                    window.draw_planar_line(&inside_line_part.0, &inside_line_part.1, &SELECTED_LINE_COLOR);
                },
                LineClipping::Outside => {
                    window.draw_planar_line(&line.0, &line.1, &LINE_COLOR);
                }
            }
        }
    }

    fn generate_lines(&mut self) {
        let mut rng = rand::thread_rng();

        let mut random_point = || {
            Point::new(
                rng.gen_range(self.x_range.clone()),
                rng.gen_range(self.y_range.clone()),
            )
        };

        let mut lines = <Vec<Line>>::with_capacity(self.lines_count as usize);
        for _ in 0..self.lines_count {
            let line = (random_point(), random_point());
            lines.push(line);
        }
        self.lines = lines;
    }
}

widget_ids! {
    pub struct Ids {
        canvas,
        line_counter
    }
}

fn proceed_ui(ui_cell: &mut UiCell, ids: &Ids, lines_manager: &mut LinesManager) {
    let dialer_margin = 20.0;
    let dialer_w = 120.0;
    let dialer_h = 30.0;

    widget::Canvas::new()
        .w_h(dialer_w + dialer_margin, dialer_h + dialer_margin)
        .rgb(1.0, 1.0, 1.0)
        .border_rgb(1.0, 1.0, 1.0)
        .top_left()
        .set(ids.canvas, ui_cell);

    for value in widget::NumberDialer::new(lines_manager.lines_count as f32, 0.0, 100.0, 0)
        .w_h(dialer_w, dialer_h)
        .top_left_with_margin_on(ids.canvas, dialer_margin)
        .label("Lines")
        .set(ids.line_counter, ui_cell) 
    {
        lines_manager.set_lines_count(value as u32);
    }
}

fn main() {
    // Window
    let mut window = Window::new("Kiss3d: obj");
    window.set_line_width(2.0);
    window.set_light(Light::StickToCamera);
    window.set_background_color(1.0, 1.0, 1.0);

    // Camera
    let mut camera = FixedView::new();

    // LinesManager
    let mut cursor = Point2::new(0.0, 0.0);
    let mut lines_manager = LinesManager::new();
    let mut selection_builder = RectangleSelectionBuilder::new();

    // UI
    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());

    while window.render_with_camera(&mut camera) {
        let window_width = window.width();
        let window_height = window.height();

        let draw_area_part = 0.9;
        lines_manager.set_draw_area_size(
            window_width as f32 * draw_area_part, 
            window_height as f32 * draw_area_part
        );

        // Coordinate system helper
        let cc = CoordinateConverter::new(window_width, window_height);

        let mut ui_cell = window.conrod_ui_mut().set_widgets();
        proceed_ui(&mut ui_cell, &ids, &mut lines_manager);
        drop(ui_cell);

        for event in window.events().iter() {
            match event.value {
                WindowEvent::CursorPos(x, y, _modif) => {
                    cursor = Point::new(
                        cc.x_top_left_to_centered_p(x as f32), 
                        cc.y_top_left_to_centered_p(y as f32)
                    );
                    selection_builder.update_cursor(cursor, None);
                },
                WindowEvent::MouseButton(btn, action ,_) => {
                    if let MouseButton::Button1 = btn {
                        selection_builder.update_cursor(cursor, Some(action));
                    }
                }
                _ => {}
            }
        }

        if let Some(selection) = selection_builder.build() {
            selection.draw(&mut window);
            lines_manager.draw_with_selection_check(&mut window, selection);
        } else {
            lines_manager.draw(&mut window);
        }
    }
}