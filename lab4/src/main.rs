use kiss3d::event::{WindowEvent, MouseButton, Action};
use kiss3d::nalgebra as na;
use kiss3d::light::Light;
use kiss3d::scene::PlanarSceneNode;
use kiss3d::window::Window;
use kiss3d::camera::{FixedView};
use kiss3d::text::Font;
use kiss3d::ncollide3d;
use kiss3d::conrod::{self, Sizeable, Colorable, Rect};
use na::{Translation2, Point3, Point2, Vector2, OPoint};

use conrod::{
    widget_ids, 
    UiCell, 
    widget, Widget,
    Positionable, Borderable, Labelable
};

use std::ops::{Add};
use std::cmp::Ordering;

use lazy_static::lazy_static;

use coordinate_converter::CoordinateConverter;

mod coordinate_converter;

const AXE_LENGTH_N: f32 = 1.6;

const X_INIT_POS_N: f32 = 0.0;
const Y_INIT_POS_N: f32 = 0.0;

lazy_static! {
    static ref SELECTION_LINE_COLOR: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
}

type Point = Point2<f32>;

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

struct RectangleSelection {
    // clockwise from top-left
    points: [Point; 4],
}

impl RectangleSelection {
    pub fn new(first: Point, second: Point) -> Self {
        let third = Point::new(first.x, second.y);
        let fourth = Point::new(second.x, first.y);

        let mut points = vec![first, second, third, fourth];
        points.sort_by(|a, b| {
            if a.y > b.y {
                Ordering::Greater
            } else if a.y == b.y {
                if a.x < b.x {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else {
                Ordering::Less
            }
        });

        Self {
            points: points.try_into().unwrap()
        }
    }

    pub fn draw(&self, window: &mut Window) {
        for (i, point) in self.points.iter().enumerate() {
            let start = point;
            let end = if i != self.points.len() - 1 {
                self.points[i + 1]
            } else {
                self.points[0]
            };

            window.draw_planar_line(start, &end, &SELECTION_LINE_COLOR);
        }
    }
}

struct RectangleSelectionBuilder {
    min_selection_diagonal: f32,
    start_point: Option<Point>,
    end_point: Option<Point>,
    is_pressed: bool,
}

impl RectangleSelectionBuilder {
    pub fn new(min_selection_diagonal: f32) -> Self {
        Self {
            min_selection_diagonal: min_selection_diagonal,
            start_point: None,
            end_point: None,
            is_pressed: false
        }
    }

    pub fn update_cursor(&mut self, cursor: Point, action: Option<Action>) {
        if let Some(action) = action {
            match action {
                Action::Press => self.mouse_pressed(cursor),
                Action::Release => self.mouse_released(cursor)
            }
        }

        if self.is_pressed {
            self.end_point = Some(cursor);
        }
    }

    fn mouse_pressed(&mut self, cursor: Point) {
        println!("Pressed: {:?}", cursor);
        self.start_point = Some(cursor);
        self.end_point = None;
        self.is_pressed = true;
    }

    fn mouse_released(&mut self, cursor: Point) {
        println!("Released: {:?}", cursor);
        // if let Some(press_point) = self.press_point {
        //     if self.is_selection(press_point, cursor) {
        //         self.start_point = Some(press_point);
        //         self.end_point = Some(cursor);
        //     }
        // }
        self.is_pressed = false;
    }

    pub fn build(&self) -> Option<RectangleSelection> {
        if self.start_point.is_some() && self.end_point.is_some() {
            Some(RectangleSelection::new (
                self.start_point.unwrap(),
                self.end_point.unwrap(),
            ))
        } else {
            None
        }
    }

    fn is_selection(&self, press_point: Point, release_point: Point) -> bool {
        ((press_point.x - release_point.x).powf(2.0) + 
         (press_point.y - release_point.y).powf(2.0))
        .powf(0.5) <= self.min_selection_diagonal
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
    let mut selection_builder = RectangleSelectionBuilder::new(30.0);

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
                    cursor = Point::new(x as f32, y as f32);
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