use std::cmp::Ordering;

use kiss3d::event::Action;
use kiss3d::window::Window;

use nalgebra::Point3;

use crate::common::Point;

use lazy_static::lazy_static;

lazy_static! {
    static ref SELECTION_LINE_COLOR: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
}

pub struct RectangleSelection {
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
        points.swap(2, 3);

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

pub struct RectangleSelectionBuilder {
    start_point: Option<Point>,
    end_point: Option<Point>,
    is_pressed: bool,
}

impl RectangleSelectionBuilder {
    pub fn new() -> Self {
        Self {
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
}