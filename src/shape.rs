extern crate chipmunk_sys as sys;

use crate::vect::*;
use crate::Body;
use std::ffi;

pub struct Shape(pub *mut sys::cpShape, pub bool);

impl Shape {
    pub unsafe fn null() -> Shape {
        Shape(std::ptr::null_mut(), false)
    }

    /// Allocate and initialize a circle shape.
    pub fn circle(body: &Body, radius: f64, offset: Vect) -> Shape {
        Shape(
            unsafe { sys::cpCircleShapeNew(body.0, radius, offset.0) },
            true,
        )
    }

    /// Get the radius of a circle shape.
    pub fn circle_radius(&self) -> f64 {
        unsafe { sys::cpCircleShapeGetRadius(self.0) }
    }

    /// Get the offset of a circle shape.
    pub fn circle_offset(&self) -> Vect {
        unsafe { sys::cpCircleShapeGetOffset(self.0) }.into()
    }

    /// Allocate and initialize a segment shape.
    pub fn segment(body: &Body, a: Vect, b: Vect, radius: f64) -> Shape {
        Shape(
            unsafe { sys::cpSegmentShapeNew(body.0, a.0, b.0, radius) },
            true,
        )
    }

    /// Get the first endpoint of a segment shape.
    pub fn segment_a(&self) -> Vect {
        unsafe { sys::cpSegmentShapeGetA(self.0) }.into()
    }

    /// Get the second endpoint of a segment shape.
    pub fn segment_b(&self) -> Vect {
        unsafe { sys::cpSegmentShapeGetB(self.0) }.into()
    }

    /// Get the first endpoint of a segment shape.
    pub fn segment_radius(&self) -> f64 {
        unsafe { sys::cpSegmentShapeGetRadius(self.0) }
    }

    /// Get the normal of a segment shape.
    pub fn segment_normal(&self) -> Vect {
        unsafe { sys::cpSegmentShapeGetNormal(self.0) }.into()
    }

    /// Allocate and initialize a polygon shape with rounded corners.
    /// The vertexes must be convex with a counter-clockwise winding.
    pub fn poly(body: &Body, verts: &[Vect], radius: f64) -> Shape {
        Shape(
            unsafe {
                sys::cpPolyShapeNewRaw(
                    body.0,
                    verts.len() as i32,
                    verts.as_ptr() as *const _ as *const sys::cpVect,
                    radius,
                )
            },
            true,
        )
    }

    /// Allocate and initialize a box shaped polygon shape.
    pub fn poly_box(body: &Body, width: f64, height: f64, radius: f64) -> Shape {
        Shape(
            unsafe { sys::cpBoxShapeNew(body.0, width, height, radius) },
            true,
        )
    }

    /// Allocate and initialize a box shaped polygon shape.
    pub fn poly_box_with_transform(
        body: &Body,
        width: f64,
        height: f64,
        radius: f64,
        offset: Vect,
        angle: f64,
    ) -> Shape {
        let width = width / 2.0;
        let height = height / 2.0;
        let rotation = Vect::from_angle(angle);

        let verts = [
            (offset + rotation.rotate(Vect::new(-width, -height))).0,
            (offset + rotation.rotate(Vect::new(width, -height))).0,
            (offset + rotation.rotate(Vect::new(width, height))).0,
            (offset + rotation.rotate(Vect::new(-width, height))).0,
        ];

        Shape(
            unsafe { sys::cpPolyShapeNewRaw(body.0, 4, verts.as_ptr(), radius) },
            true,
        )
    }

    // /// The cpSpace this body is added to.
    // CP_EXPORT cpSpace* cpShapeGetSpace(const cpShape *shape);

    // /// The cpBody this shape is connected to.
    // CP_EXPORT cpBody* cpShapeGetBody(const cpShape *shape);
    // /// Set the cpBody this shape is connected to.
    // /// Can only be used if the shape is not currently added to a space.
    // CP_EXPORT void cpShapeSetBody(cpShape *shape, cpBody *body);

    /// Get the mass of the shape if you are having Chipmunk calculate mass properties for you.
    pub fn mass(&self) -> f64 {
        unsafe { sys::cpShapeGetMass(self.0) }
    }

    /// Set the mass of this shape to have Chipmunk calculate mass properties for you.
    pub fn set_mass(&mut self, mass: f64) {
        unsafe { sys::cpShapeSetMass(self.0, mass) }
    }

    /// Get the density of the shape if you are having Chipmunk calculate mass properties for you.
    pub fn density(&self) -> f64 {
        unsafe { sys::cpShapeGetDensity(self.0) }
    }

    /// Set the density of this shape to have Chipmunk calculate mass properties for you.
    pub fn set_density(&mut self, density: f64) {
        unsafe { sys::cpShapeSetDensity(self.0, density) }
    }

    /// Get the calculated moment of inertia for this shape.
    pub fn moment(&self) -> f64 {
        unsafe { sys::cpShapeGetMoment(self.0) }
    }

    /// Get the calculated area of this shape.
    pub fn area(&self) -> f64 {
        unsafe { sys::cpShapeGetArea(self.0) }
    }

    /// Get the centroid of this shape.
    pub fn center_of_gravity(&self) -> Vect {
        unsafe { sys::cpShapeGetCenterOfGravity(self.0) }.into()
    }

    // /// Get the bounding box that contains the shape given it's current position and angle.
    // CP_EXPORT cpBB cpShapeGetBB(const cpShape *shape);

    /// Get if the shape is set to be a sensor or not.
    pub fn is_sensor(&self) -> bool {
        unsafe { sys::cpShapeGetSensor(self.0) != 0 }
    }

    /// Set if the shape is a sensor or not.
    pub fn set_sensor(&mut self, sensor: bool) {
        unsafe { sys::cpShapeSetSensor(self.0, sensor as u8) }
    }

    /// Get the elasticity of this shape.
    pub fn elasticity(&self) -> f64 {
        unsafe { sys::cpShapeGetElasticity(self.0) }
    }

    /// Set the elasticity of this shape.
    pub fn set_elasticity(&mut self, elasticity: f64) {
        unsafe { sys::cpShapeSetElasticity(self.0, elasticity) }
    }

    /// Get the friction of this shape.
    pub fn friction(&self) -> f64 {
        unsafe { sys::cpShapeGetFriction(self.0) }
    }

    /// Set the friction of this shape.
    pub fn set_friction(&mut self, friction: f64) {
        unsafe { sys::cpShapeSetFriction(self.0, friction) }
    }

    /// Get the surface velocity of this shape.
    pub fn surface_velocity(&self) -> Vect {
        unsafe { sys::cpShapeGetSurfaceVelocity(self.0) }.into()
    }

    /// Set the surface velocity of this shape.
    pub fn set_surface_velocity(&mut self, surface_velocity: Vect) {
        unsafe { sys::cpShapeSetSurfaceVelocity(self.0, surface_velocity.0) }
    }

    // /// Get the user definable data pointer of this shape.
    pub fn user_data(&self) -> *mut ffi::c_void {
        unsafe { sys::cpShapeGetUserData(self.0) }
    }

    // /// Set the user definable data pointer of this shape.
    pub fn set_user_data(&mut self, user_data: *mut ffi::c_void) {
        unsafe { sys::cpShapeSetUserData(self.0, user_data) }
    }

    // /// Set the collision type of this shape.
    // CP_EXPORT cpCollisionType cpShapeGetCollisionType(const cpShape *shape);
    // /// Get the collision type of this shape.
    // CP_EXPORT void cpShapeSetCollisionType(cpShape *shape, cpCollisionType collisionType);

    // /// Get the collision filtering parameters of this shape.
    // CP_EXPORT cpShapeFilter cpShapeGetFilter(const cpShape *shape);
    // /// Set the collision filtering parameters of this shape.
    // CP_EXPORT void cpShapeSetFilter(cpShape *shape, cpShapeFilter filter);
}

unsafe impl Send for Shape {}

impl Default for Shape {
    fn default() -> Shape {
        Shape(std::ptr::null_mut(), false)
    }
}

impl Drop for Shape {
    fn drop(&mut self) {
        if self.1 {
            unsafe { sys::cpShapeFree(self.0) };
        }
    }
}
