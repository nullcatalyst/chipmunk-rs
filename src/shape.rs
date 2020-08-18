extern crate chipmunk_sys as sys;

use crate::Body;

pub struct Shape(pub *mut sys::cpShape);

unsafe impl Send for Shape {}

impl Shape {
    /// Allocate and initialize a circle shape.
    pub fn circle(body: &Body, radius: f64, offset: sys::cpVect) -> Shape {
        Shape(unsafe { sys::cpCircleShapeNew(body.0, radius, offset) })
    }

    /// Get the radius of a circle shape.
    pub fn circle_radius(&self) -> f64 {
        unsafe { sys::cpCircleShapeGetRadius(self.0) }
    }

    /// Get the offset of a circle shape.
    pub fn circle_offset(&self) -> sys::cpVect {
        unsafe { sys::cpCircleShapeGetOffset(self.0) }
    }

    /// Allocate and initialize a segment shape.
    pub fn segment(body: &Body, a: sys::cpVect, b: sys::cpVect, radius: f64) -> Shape {
        Shape(unsafe { sys::cpSegmentShapeNew(body.0, a, b, radius) })
    }

    /// Get the first endpoint of a segment shape.
    pub fn segment_a(&self) -> sys::cpVect {
        unsafe { sys::cpSegmentShapeGetA(self.0) }
    }

    /// Get the second endpoint of a segment shape.
    pub fn segment_b(&self) -> sys::cpVect {
        unsafe { sys::cpSegmentShapeGetB(self.0) }
    }

    /// Get the first endpoint of a segment shape.
    pub fn segment_radius(&self) -> f64 {
        unsafe { sys::cpSegmentShapeGetRadius(self.0) }
    }

    /// Get the normal of a segment shape.
    pub fn segment_normal(&self) -> sys::cpVect {
        unsafe { sys::cpSegmentShapeGetNormal(self.0) }
    }

    /// Allocate and initialize a box shaped polygon shape.
    pub fn poly_box(body: &Body, width: f64, height: f64, radius: f64) -> Shape {
        Shape(unsafe { sys::cpBoxShapeNew(body.0, width, height, radius) })
    }

    /// Allocate and initialize a polygon shape with rounded corners.
    /// The vertexes must be convex with a counter-clockwise winding.
    pub fn poly(body: &Body, verts: &[sys::cpVect], radius: f64) -> Shape {
        Shape(unsafe { sys::cpPolyShapeNewRaw(body.0, verts.len() as i32, verts.as_ptr(), radius) })
    }
}

impl Drop for Shape {
    fn drop(&mut self) {
        unsafe {
            sys::cpShapeFree(self.0);
        }
    }
}
