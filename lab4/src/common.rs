use nalgebra::Point2;

pub type Point = Point2<f32>;
pub type Line = (Point, Point);

#[derive(Debug)]
pub enum LineClipping {
    Inside,
    Outside,
    PartlyInside(Line)
}