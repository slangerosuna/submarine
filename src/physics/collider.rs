use crate::physics::*;

pub enum Collider {
    Sphere(SphereCollider),
    Rectangle(RectangleCollider),
}

#[derive(Copy, Clone)]
pub struct SphereCollider {
    pub radius: f32,
}

impl SphereCollider {
    fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[derive(Copy, Clone)]
pub struct RectangleCollider {
    pub width: f32,
    pub height: f32,
    pub depth: f32,

    minimum_distance: f32,
}

impl RectangleCollider {
    fn new(width: f32, height: f32, depth: f32) -> Self {
        let minimum_distance = f32::sqrt(width * width 
                                        + height * height 
                                        + depth * depth);
        Self {
            width,
            height,
            depth,
            minimum_distance,
        }
    }
}

pub struct Collision {
    pub normal: Vector3,
    pub penetration: f32,
    pub contact_point: Vector3,
}

fn sphere_sphere_collision(
    a: &SphereCollider, a_pos: &Vector3,
    b: &SphereCollider, b_pos: &Vector3,
) -> Option<Collision> {
    
    let distance = a.radius + b.radius;
    let normal = b_pos.sub(a_pos);
    let penetration = distance - normal.magnitude();

    if penetration < 0.0 {
        return None;
    }

    Some(Collision {
        normal: normal.normalized(),
        penetration,
        contact_point: a_pos.add(&normal.normalized().multiply(a.radius)),
    })
}

//TODO
fn rectangle_sphere_collision(
    rect: &RectangleCollider, rect_pos: &Vector3, rect_rot: &Matrix4,
    sphere: &SphereCollider, sphere_pos: &Vector3,
) -> Option<Collision> {
    panic!("rectangle-sphere collision not implemented");
}

//TODO
fn rectangle_rectangle_collison(
    a: &RectangleCollider, a_pos: &Vector3, a_rot: &Matrix4,
    b: &RectangleCollider, b_pos: &Vector3, b_rot: &Matrix4,
) -> Option<Collision> {
    panic!("rectangle-rectangle collision not implemented");
}
/**
 * @summary Calculates the collision between two colliders
 */
pub fn calc_collision(
    a: &Collider, a_pos: &Vector3, a_rot: &Matrix4,
    b: &Collider, b_pos: &Vector3, b_rot: &Matrix4,
) -> Option<Collision> {
    match a {
        Collider::Sphere(a) => match b {
            Collider::Sphere(b) => sphere_sphere_collision(a, a_pos, b, b_pos),
            Collider::Rectangle(b) => rectangle_sphere_collision(b, b_pos, b_rot, a, a_pos),
        },
        Collider::Rectangle(a) => match b {
            Collider::Sphere(b) => rectangle_sphere_collision(a, a_pos, a_rot, b, b_pos),
            Collider::Rectangle(b) => rectangle_rectangle_collison(a, a_pos, a_rot, b, b_pos, b_rot),
        },
    }
}
