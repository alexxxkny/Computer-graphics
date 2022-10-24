pub struct Model {
    pub position: [f32; 3],
    pub scale: f32
}

impl Model {
    pub fn matrix(&self) -> [[f32; 4]; 4] {
        let (x, y, z) = (self.position[0], self.position[1], self.position[2]);
        let scale = self.scale;
        [
            [1.0 * scale, 0.0, 0.0, 0.0],
            [0.0, 1.0 * scale, 0.0, 0.0],
            [0.0, 0.0, 1.0 * scale, 0.0],
            [x, y, z, 1.0f32]
        ]
    }
}

pub struct View {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub up: [f32; 3]
}

impl View {
    pub fn matrix(&self) -> [[f32; 4]; 4] {
        let position = self.position;
        let direction = self.direction;
        let up = self.up;


        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];
    
        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };
    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    
        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}

pub struct Perspective {
    pub width: f32,
    pub height: f32,
    pub front_of_view: f32,
    pub z_far: f32,
    pub z_near: f32
}

impl Perspective {
    pub fn matrix(&self) -> [[f32; 4]; 4] {
        let aspect_ratio = self.height as f32 / self.width as f32;

        let fov = self.front_of_view;
        let z_far = self.z_far;
        let z_near = self.z_near;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (z_far + z_near) / (z_far - z_near)    ,   1.0],
            [         0.0         ,    0.0, -(2.0 * z_far * z_near) / (z_far - z_near),   0.0],
        ]
    }
}