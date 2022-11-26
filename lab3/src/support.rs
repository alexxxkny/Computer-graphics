use std::f32::consts::PI;
use nalgebra::{UnitQuaternion, Vector3, Point3};

pub struct Rotation {
    pub x_angle: f32,
    pub y_angle: f32,
    pub z_angle: f32,
}

impl Rotation {
    pub fn x(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.x_angle / 180.0 * PI)
    }

    pub fn y(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.y_angle / 180.0 * PI)
    }

    pub fn z(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_axis_angle(&Vector3::z_axis(), self.z_angle / 180.0 * PI)
    }
}

pub struct BilinearSurface {
    vertices_v: [Vector3<f32>; 4],
}

impl BilinearSurface {
    pub fn new(vertices: &[Point3<f32>]) -> Self {
        let mut vertices_v = <Vec<Vector3<f32>>>::with_capacity(4);

        for v in vertices.iter() {
            vertices_v.push(Vector3::new(v.x, v.y, v.z));
        }

        Self {
            vertices_v: vertices_v.try_into().unwrap()
        }
    }

    pub fn point(&self, u: f32, w: f32) -> Point3<f32>{
        let p1 = self.vertices_v[0];
        let p2 = self.vertices_v[1];
        let p3 = self.vertices_v[2];
        let p4 = self.vertices_v[3];

        let t = p1 * ((1.0 - u) * (1.0 - w))
            + p2 * ((1.0 - u) * w)
            + p3 * (u * (1.0 - w))
            + p4 * u * w;

        Point3::from(t)
    }
}