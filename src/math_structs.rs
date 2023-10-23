
#[derive(Copy, Clone)]
struct Vec3([f32; 3]);

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3([x, y, z])
    }
    fn x(&self) -> f32 {
        self.0[0]
    }
    fn y(&self) -> f32 {
        self.0[1]
    }
    fn z(&self) -> f32 {
        self.0[2]
    }

    fn set_x(&mut self, x: f32) {
        self.0[0] = x;
    }
    fn set_y(&mut self, y: f32) {
        self.0[1] = y;
    }
    fn set_z(&mut self, z: f32) {
        self.0[2] = z;
    }
    
    fn add(&mut self, other: Vec3) {
        self.0[0] += other.0[0];
        self.0[1] += other.0[1];
        self.0[2] += other.0[2];
    }
    fn sub(&mut self, other: Vec3) {
        self.0[0] -= other.0[0];
        self.0[1] -= other.0[1];
        self.0[2] -= other.0[2];
    }
    fn mul(&mut self, other: f32) {
        self.0[0] *= other;
        self.0[1] *= other;
        self.0[2] *= other;
    }
    fn div(&mut self, other: f32) {
        self.0[0] /= other;
        self.0[1] /= other;
        self.0[2] /= other;
    }

    fn dot(&self, other: Vec3) -> f32 {
        self.0[0] * other.0[0] +
        self.0[1] * other.0[1] +
        self.0[2] * other.0[2]
    }
    fn cross(&self, other: Vec3) -> Vec3 {
        Vec3([
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0],
        ])
    }
}

#[derive(Copy, Clone)]
struct Vec2([f32; 2]);

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Vec2([x, y])
    }
    fn x(&self) -> f32 {
        self.0[0]
    }
    fn y(&self) -> f32 {
        self.0[1]
    }
    fn set_x(&mut self, x: f32) {
        self.0[0] = x;
    }
    fn set_y(&mut self, y: f32) {
        self.0[1] = y;
    }
    fn add(&mut self, other: Vec2) {
        self.0[0] += other.0[0];
        self.0[1] += other.0[1];
    }
    fn sub(&mut self, other: Vec2) {
        self.0[0] -= other.0[0];
        self.0[1] -= other.0[1];
    }
    fn mul(&mut self, other: f32) {
        self.0[0] *= other;
        self.0[1] *= other;
    }
    fn div(&mut self, other: f32) {
        self.0[0] /= other;
        self.0[1] /= other;
    }
    fn dot(&self, other: Vec2) -> f32 {
        self.0[0] * other.0[0] +
        self.0[1] * other.0[1]
    }
    fn cross(&self, other: Vec2) -> f32 {
        self.0[0] * other.0[1] -
        self.0[1] * other.0[0]
    }
}

#[derive(Copy, Clone)]
struct Quat([f32; 4]);

impl Quat {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quat([x, y, z, w])
    }
    fn from_euler_angles(euler_angles: Vec3) -> Self {
        let c1 = euler_angles.x().cos();
        let c2 = euler_angles.y().cos();
        let c3 = euler_angles.z().cos();
        let s1 = euler_angles.x().sin();
        let s2 = euler_angles.y().sin();
        let s3 = euler_angles.z().sin();
        Quat([
            s1 * c2 * c3 + c1 * s2 * s3,
            c1 * s2 * c3 - s1 * c2 * s3,
            c1 * c2 * s3 - s1 * s2 * c3,
            c1 * c2 * c3 + s1 * s2 * s3,
        ])
    }
    fn x(&self) -> f32 {
        self.0[0]
    }
    fn y(&self) -> f32 {
        self.0[1]
    }
    fn z(&self) -> f32 {
        self.0[2]
    }
    fn w(&self) -> f32 {
        self.0[3]
    }
    fn set_x(&mut self, x: f32) {
        self.0[0] = x;
    }
    fn set_y(&mut self, y: f32) {
        self.0[1] = y;
    }
    fn set_z(&mut self, z: f32) {
        self.0[2] = z;
    }
    fn set_w(&mut self, w: f32) {
        self.0[3] = w;
    }
    fn add(&mut self, other: Quat) {
        self.0[0] += other.0[0];
        self.0[1] += other.0[1];
        self.0[2] += other.0[2];
        self.0[3] += other.0[3];
    }
    fn sub(&mut self, other: Quat) {
        self.0[0] -= other.0[0];
        self.0[1] -= other.0[1];
        self.0[2] -= other.0[2];
        self.0[3] -= other.0[3];
    }
    fn mul(&mut self, other: Quat) {
        let x = self.0[0] * other.0[3] + self.0[1] * other.0[2] - self.0[2] * other.0[1] + self.0[3] * other.0[0];
        let y = -self.0[0] * other.0[2] + self.0[1] * other.0[3] + self.0[2] * other.0[0] + self.0[3] * other.0[1];
        let z = self.0[0] * other.0[1] - self.0[1] * other.0[0] + self.0[2] * other.0[3] + self.0[3] * other.0[2];
        let w = -self.0[0] * other.0[0] - self.0[1] * other.0[1] - self.0[2] * other.0[2] + self.0[3] * other.0[3];
        self.0[0] = x;
        self.0[1] = y;
        self.0[2] = z;
        self.0[3] = w;
    }
    fn rotate(&self, other: Vec3) -> Vec3 {
        let x = self.0[1] * other.0[2] - self.0[2] * other.0[1] + self.0[3] * other.0[0];
        let y = -self.0[0] * other.0[2] + self.0[3] * other.0[1];
        let z = self.0[0] * other.0[1] - self.0[1] * other.0[0] + self.0[3] * other.0[2];
        Vec3([x, y, z])
    }
}
