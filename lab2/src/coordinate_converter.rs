struct Bounds {
    pub x_right: f32,
    pub x_left: f32,
    pub y_top: f32,
    pub y_bottom: f32,
}

impl Bounds {
    pub fn new_centered_p(window_width: u32, window_height: u32) -> Self {
        Self {
            x_right: window_width as f32 / 2.0,
            x_left: -(window_width as f32 / 2.0),
            y_top: window_height as f32 / 2.0,
            y_bottom: -(window_height as f32 / 2.0),
        }
    }

    pub fn new_centered_n() -> Self {
        Self {
            x_right: 1.0,
            x_left: -1.0,
            y_top: 1.0,
            y_bottom: -1.0,
        }
    }

    pub fn new_top_left_p(window_width: u32, window_height: u32) -> Self {
        Self {
            x_right: window_width as f32,
            x_left: -(window_width as f32),
            y_top: window_height as f32,
            y_bottom: -(window_height as f32),
        }
    }
}

// Centered <--> TopLeft converter
pub struct CoordinateConverter {
    window_width: u32,
    window_height: u32,
    centered_pixel_bounds: Bounds,
    centered_normalized_bounds: Bounds,
    top_left_pixel_bounds: Bounds,
}

impl CoordinateConverter {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        Self { 
            window_width, 
            window_height, 
            centered_pixel_bounds: Bounds::new_centered_p(window_width, window_height),
            centered_normalized_bounds: Bounds::new_centered_n(),
            top_left_pixel_bounds: Bounds::new_top_left_p(window_width, window_height),
        }
    }

    fn x_centered_pixel_bounded(&self, x: f32) -> f32 {
        if x < self.centered_pixel_bounds.x_left {
            self.centered_pixel_bounds.x_left
        } else if x > self.centered_pixel_bounds.x_right {
            self.centered_pixel_bounds.x_right
        } else {
            x
        }
    }

    fn y_centered_pixel_bounded(&self, y: f32) -> f32 {
        if y < self.centered_pixel_bounds.y_bottom {
            self.centered_pixel_bounds.y_bottom
        } else if y > self.centered_pixel_bounds.y_top {
            self.centered_pixel_bounds.y_top
        } else {
            y
        }
    }

    fn x_centered_normalized_bounded(&self, x: f32) -> f32 {
        if x < self.centered_normalized_bounds.y_bottom {
            self.centered_normalized_bounds.y_bottom
        } else if x > self.centered_normalized_bounds.y_top {
            self.centered_normalized_bounds.y_top
        } else {
            x
        }
    }

    fn y_centered_normalized_bounded(&self, y: f32) -> f32 {
        if y < self.centered_normalized_bounds.y_bottom {
            self.centered_normalized_bounds.y_bottom
        } else if y > self.centered_normalized_bounds.y_top {
            self.centered_normalized_bounds.y_top
        } else {
            y
        }
    }

    fn x_top_left_pixel_bounded(&self, x: f32) -> f32 {
        if x < self.top_left_pixel_bounds.x_left {
            self.top_left_pixel_bounds.x_left
        } else if x > self.top_left_pixel_bounds.x_right {
            self.top_left_pixel_bounds.x_right
        } else {
            x
        }
    }

    fn y_top_left_pixel_bounded(&self, y: f32) -> f32 {
        if y < self.top_left_pixel_bounds.y_bottom {
            self.top_left_pixel_bounds.y_bottom
        } else if y > self.top_left_pixel_bounds.y_top {
            self.top_left_pixel_bounds.y_top
        } else {
            y
        }
    }

    pub fn x_centered_n_to_p(&self, x_normalized: f32) -> f32 {
        let x_normalized_bounded = self.x_centered_normalized_bounded(x_normalized);
        x_normalized_bounded * (self.window_width as f32) / 2.0
    }

    pub fn y_centered_n_to_p(&self, y_normalized: f32) -> f32 {
        let y_normalized_bounded = self.y_centered_normalized_bounded(y_normalized);
        y_normalized_bounded * (self.window_height as f32) / 2.0
    }

    pub fn x_centered_p_to_n(&self, x_pixels: f32) -> f32 {
        let x_pixels_bounded = self.x_centered_pixel_bounded(x_pixels);
        x_pixels_bounded / ((self.window_width as f32) / 2.0)
    }

    pub fn y_centered_p_to_n(&self, y_pixels: f32) -> f32 {
        let y_pixels_bounded = self.y_centered_pixel_bounded(y_pixels);
        y_pixels_bounded / ((self.window_height as f32) / 2.0)
    }

    pub fn x_top_left_to_centered_p(&self, x_pixels: f32) -> f32 {
        let x_pixels_bounded = self.x_top_left_pixel_bounded(x_pixels);
        x_pixels_bounded - self.window_width as f32 / 2.0
    }

    pub fn y_top_left_to_centered_p(&self, y_pixels: f32) -> f32 {
        let y_pixels_bounded = self.y_top_left_pixel_bounded(y_pixels);
        -(y_pixels_bounded - self.window_height as f32 / 2.0)
    }

    pub fn x_centered_to_top_left_p(&self, x_pixels: f32) -> f32 {
        let x_pixels_bounded = self.x_centered_pixel_bounded(x_pixels);
        2.0 * x_pixels_bounded + self.window_width as f32
    }

    pub fn y_centered_to_top_left_p(&self, y_pixels: f32) -> f32 {
        let y_pixels_bounded = self.x_centered_pixel_bounded(y_pixels);
        -(2.0 * y_pixels_bounded - self.window_height as f32)
    }
}