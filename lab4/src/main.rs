use kiss3d::event::{WindowEvent, MouseButton};
use kiss3d::nalgebra as na;
use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::{FixedView};
use kiss3d::conrod::{self, Sizeable, Colorable};
use na::{Point2};

use conrod::{
    widget_ids, 
    UiCell, 
    widget, Widget,
    Positionable, Borderable, Labelable
};

use coordinate_converter::CoordinateConverter;
use common::*;
use selection::*;

mod coordinate_converter;
mod common;
mod selection;

struct State {
    pub lines_count: u32
}

impl State {
    pub fn new(lines_count: u32) -> Self {
        Self { lines_count }
    }

    pub fn set_lines_count(&mut self, lines_count: u32) {
        self.lines_count = lines_count;
    }
}

widget_ids! {
    pub struct Ids {
        canvas,
        line_counter
    }
}

fn proceed_ui(ui_cell: &mut UiCell, ids: &Ids, state: &mut State) {
    let dialer_margin = 20.0;
    let dialer_w = 120.0;
    let dialer_h = 30.0;

    widget::Canvas::new()
        .w_h(dialer_w + dialer_margin, dialer_h + dialer_margin)
        .rgb(1.0, 1.0, 1.0)
        .border_rgb(1.0, 1.0, 1.0)
        .top_left()
        .set(ids.canvas, ui_cell);

    for value in widget::NumberDialer::new(state.lines_count as f32, 0.0, 100.0, 0)
        .w_h(dialer_w, dialer_h)
        .top_left_with_margin_on(ids.canvas, dialer_margin)
        .label("Lines")
        .set(ids.line_counter, ui_cell) 
    {
        state.set_lines_count(value as u32);
    }
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
    let mut state = State::new(0);
    let mut selection_builder = RectangleSelectionBuilder::new();

    // UI
    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());

    while window.render_with_camera(&mut camera) {
        let window_width = window.width();
        let window_height = window.height();

        // Coordinate system helper
        let cc = CoordinateConverter::new(window_width, window_height);

        let mut ui_cell = window.conrod_ui_mut().set_widgets();
        proceed_ui(&mut ui_cell, &ids, &mut state);
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
        }

    }
}