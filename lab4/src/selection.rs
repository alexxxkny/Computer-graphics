use std::cmp::{Ordering};

use kiss3d::event::Action;
use kiss3d::window::Window;

use nalgebra::{Point3};

use crate::common::{Point, LineClipping, Line};

use lazy_static::lazy_static;

lazy_static! {
    static ref SELECTION_LINE_COLOR: Point3<f32> = Point3::new(1.0, 0.0, 0.0);
}

pub struct RectangleSelection {
    // clockwise from top-left
    points: [Point; 4],
    // left, right, top, bottom
    borders: (f32, f32, f32, f32)
}

impl RectangleSelection {
    pub fn new(first: Point, second: Point) -> Self {
        let third = Point::new(first.x, second.y);
        let fourth = Point::new(second.x, first.y);

        let mut points = vec![first, second, third, fourth];
        points.sort_by(|b, a| {
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


        let points: [Point; 4] = points.try_into().unwrap();
        Self {
            points: points,
            borders: (points[0].x, points[1].x, points[0].y, points[2].y)
        }
    }

    pub fn clipping_check(&self, line: &Line) -> LineClipping {
        if let Some(clipping) = self.trivial_clipping_check(line) {
            clipping
        } else { 
            self.complex_clipping_check(line)
        }
    }

    fn complex_clipping_check(&self, line: &Line) -> LineClipping {
        let (l, r, t, b) = self.borders;
        let (start, end) = line;
        let (xs, xe, ys, ye) = (start.x, end.x, start.y, end.y);
        let (dx, dy) = (xe - xs, ye - ys);

        let xt = |t: f32| {
            xs + dx * t
        };

        let yt = |t: f32| {
            ys + dy * t
        };

        let t_i = vec![
            (l - xs) / dx, // left
            (r - xs) / dx, // right
            (t - ys) / dy, // top
            (b - ys) / dy, // bottom
        ];

        let point_i = vec![
            Point::new(xt(t_i[0]), yt(t_i[0])),
            Point::new(xt(t_i[1]), yt(t_i[1])),
            Point::new(xt(t_i[2]), yt(t_i[2])),
            Point::new(xt(t_i[3]), yt(t_i[3])),
        ];

        let mut first_point: Option<Point> = None;
        for (i, point) in point_i.iter().enumerate() {
            if 0.0 <= t_i[i] && t_i[i] <= 1.0 {
                if self.is_point_on_border(&point) {
                    if first_point.is_none() {
                        first_point = Some(*point);
                    } else {
                        return LineClipping::PartlyInside((first_point.unwrap(), *point));
                    }
                }
            }
        }

        if first_point.is_none() {
            LineClipping::Outside
        } else {
            let first_point = first_point.unwrap();
            let first_candidate = Point::new(xt(0.0), yt(0.0));
            let second_candidate = Point::new(xt(1.0), yt(1.0));
            if self.clipping_byte_code(&first_candidate) == 0u8 {
                LineClipping::PartlyInside((first_candidate, first_point))
            } else {
                LineClipping::PartlyInside((first_point, second_candidate))
            }
        }
    }

    fn trivial_clipping_check(&self, line: &Line) -> Option<LineClipping> {
        let (start, end) = line;

        let start_byte_code = self.clipping_byte_code(start);
        let end_byte_code = self.clipping_byte_code(end);

        if start_byte_code | end_byte_code == 0u8 {
            Some(LineClipping::Inside)
        } else if start_byte_code & end_byte_code != 0u8 {
            Some(LineClipping::Outside)
        } else {
            None
        }
    }

    fn clipping_byte_code(&self, point: &Point) -> u8 {
        let mut code = 0u8;
        let (l, r, t, b) = self.borders;
        let x = point.x;
        let y = point.y;

        if x < l { code |= 1u8 << 3; }
        if x > r { code |= 1u8 << 2; }
        if y < b { code |= 1u8 << 1; }
        if y > t { code |= 1u8; }

        code
    }

    fn is_point_on_border(&self, point: &Point) -> bool {
        // acceptable error
        let e = 2.0;
        let (l, r, t, b) = self.borders;
        let (x, y) = (point.x, point.y);

        (l - e < x && x < l + e && b < y && y < t ) || // left border
        (r - e < x && x < r + e && b < y && y < t ) || // right border
        (t - e < y && y < t + e && l < x && x < r ) || // top border
        (b - e < y && y < b + e && l < x && x < r ) // bottom border
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
                Action::Release => self.mouse_released()
            }
        }

        if self.is_pressed {
            self.end_point = Some(cursor);
        }
    }

    fn mouse_pressed(&mut self, cursor: Point) {
        self.start_point = Some(cursor);
        self.end_point = None;
        self.is_pressed = true;
    }

    fn mouse_released(&mut self) {
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