use crate::helpers::{col_to_rgb_u32, Col};
use crate::scene::Camera;
use cgmath::{EuclideanSpace, Matrix4, Point3, Vector2, Vector3, Vector4};

pub struct Line2d {
    pub p1: Vector2<i32>,
    pub p2: Vector2<i32>,
    pub color: Col,
}

impl Line2d {
    pub fn new(p1: Vector2<i32>, p2: Vector2<i32>, color: Col) -> Line2d {
        Line2d { p1, p2, color }
    }

    pub fn clamp(&self, display_width: usize, display_height: usize) -> Option<Line2d> {
        let p1: Vector2<f32> = Vector2::new(self.p1.x as f32, self.p1.y as f32);
        let p2: Vector2<f32> = Vector2::new(self.p2.x as f32, self.p2.y as f32);

        // Clamp p1

        let r = p2 - p1;
        let t = if p2.x < 0.0 {
            -p1.x / r.x
        } else if p2.x > display_width as f32 {
            (display_width as f32 - p1.x) / r.x
        } else {
            1.0
        };
        let p2 = p1 + r * t;

        let r = p2 - p1;
        let t = if p2.y < 0.0 {
            -p1.y / r.y
        } else if p2.y > display_height as f32 {
            (display_height as f32 - p1.y) / r.y
        } else {
            1.0
        };
        let p2 = p1 + r * t;

        // Clamp p2

        let r = p1 - p2;
        let t = if p1.x < 0.0 {
            -p2.x / r.x
        } else if p1.x > display_width as f32 {
            (display_width as f32 - p2.x) / r.x
        } else {
            1.0
        };
        let p1 = p2 + r * t;

        let r = p1 - p2;
        let t = if p1.y < 0.0 {
            -p2.y / r.y
        } else if p1.y > display_height as f32 {
            (display_height as f32 - p2.y) / r.y
        } else {
            1.0
        };
        let p1 = p2 + r * t;

        if !(p1.x + p2.x + p1.y + p2.y).is_nan() && !(p1.x + p2.x + p1.y + p2.y).is_infinite() {
            Some(Line2d {
                p1: Vector2::new(p1.x as i32, p1.y as i32),
                p2: Vector2::new(p2.x as i32, p2.y as i32),
                color: self.color,
            })
        } else {
            None
        }
    }

    fn plot(
        &self,
        display_width: &'static usize,
        display_height: &'static usize,
    ) -> impl Iterator<Item = (i32, i32)> {
        let x1 = self.p1.x;
        let y1 = self.p1.y;
        let x2 = self.p2.x;
        let y2 = self.p2.y;

        fn plot_line_low(x1: i32, y1: i32, x2: i32, y2: i32) -> impl Iterator<Item = (i32, i32)> {
            let dx = x2 - x1;
            let mut dy = y2 - y1;
            let mut yi = 1;
            if dy < 0 {
                yi = -1;
                dy = -dy;
            }
            let mut d = 2 * dy - dx;
            let mut y = y1;
            return (x1..x2).map(move |x| {
                let coordinates = (x, y);
                if d > 0 {
                    y = y + yi;
                    d = d - 2 * dx;
                }
                d = d + 2 * dy;
                coordinates
            });
        };
        let coordinates = if (y2 - y1).abs() < (x2 - x1).abs() {
            if x1 > x2 {
                plot_line_low(x2, y2, x1, y1)
            } else {
                plot_line_low(x1, y1, x2, y2)
            }
        } else {
            if y1 > y2 {
                plot_line_low(y2, x2, y1, x1)
            } else {
                plot_line_low(y1, x1, y2, x2)
            }
        };
        coordinates
            .filter(move |(x, y)| {
                *x < (*display_width as i32) && *x > 0 && *y < (*display_height as i32) && *y > 0
            })
            .map(move |(x, y)| {
                if (y2 - y1).abs() < (x2 - x1).abs() {
                    (x, y)
                } else {
                    (y, x)
                }
            })
    }

    pub fn render(
        &self,
        buffer: &mut Vec<u32>,
        display_width: &'static usize,
        display_height: &'static usize,
    ) {
        for (x, y) in self.plot(display_width, display_height) {
            let col = col_to_rgb_u32(self.color);

            buffer[display_width * y as usize + (display_width - x as usize)] = col;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line3d {
    pub p1: Vector3<f32>,
    pub p2: Vector3<f32>,
    pub color: Col,
}

impl Line3d {
    pub fn new(p1: Vector3<f32>, p2: Vector3<f32>, color: Col) -> Line3d {
        Line3d {
            p1: p1,
            p2: p2,
            color: color,
        }
    }

    pub fn render_line(
        &self,
        camera: &Camera,
        display_width: &'static usize,
        display_height: &'static usize,
    ) -> impl Iterator<Item = (i32, i32)> {
        let matrix: Matrix4<f32> = cgmath::PerspectiveFov {
            fovy: cgmath::Rad(camera.fov * std::f32::consts::PI / 180.0),
            aspect: 1.0,
            near: 1.0,
            far: 10.0,
        }
        .into();

        let coord1 = (self.p1 - camera.pos).extend(0.0);
        let coord2 = (self.p2 - camera.pos).extend(0.0);

        let rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(camera.rot.z))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(camera.rot.y))
            * cgmath::Matrix4::from_angle_x(cgmath::Rad(camera.rot.x));

        let dir = rot * Vector4::new(0.0, 1.0, 0.0, 0.0);

        let dir = Matrix4::look_at_dir(
            Point3::from_vec(camera.pos),
            dir.truncate(),
            Vector3::new(0.0, 0.0, 1.0),
        );

        let coord1 = matrix * dir * coord1;
        let coord2 = matrix * dir * coord2;

        let half_display_height = *display_height as f32 / -2.0;
        let half_display_width = *display_width as i32 / 2;

        let line = Line2d::new(
            Vector2::new(
                (half_display_height * coord1.x / coord1.w) as i32 + half_display_width,
                (half_display_height * coord1.y / coord1.w) as i32 + -half_display_height as i32,
            ),
            Vector2::new(
                (half_display_height * coord2.x / coord2.w) as i32 + half_display_width,
                (half_display_height * coord2.y / coord2.w) as i32 + -half_display_height as i32,
            ),
            self.color,
        )
        .clamp(*display_width, *display_height);

        if coord1.w > 0.0 && coord2.w > 0.0 {
            if let Some(line) = line {
                either::Right(line.plot(display_width, display_height))
            } else {
                either::Left(std::iter::empty())
            }
        } else {
            either::Left(std::iter::empty())
        }
    }
}

pub struct Box {
    pub points: Vec<Vector2<i32>>,
    pub color: Col,
}

impl Box {
    fn lines(&self) -> Vec<Line2d> {
        vec![
            Line2d::new(self.points[0], self.points[1], self.color),
            Line2d::new(self.points[1], self.points[2], self.color),
            Line2d::new(self.points[2], self.points[3], self.color),
            Line2d::new(self.points[3], self.points[0], self.color),
        ]
    }
    pub fn draw(
        &self,
        buffer: &mut Vec<u32>,
        display_width: &'static usize,
        display_height: &'static usize,
    ) {
        for line in self.lines() {
            line.render(buffer, display_width, display_height);
        }
    }
}
