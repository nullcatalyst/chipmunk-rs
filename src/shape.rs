extern crate chipmunk_sys as sys;

use crate::Body;

pub struct Shape(pub(crate) *mut sys::cpShape);

unsafe impl Send for Shape {}

impl Shape {
    pub fn circle(body: Body, radius: f64, offset: sys::cpVect) -> Shape {
        unsafe { Shape(sys::cpCircleShapeNew(body.0, radius, offset)) }
    }

    pub fn circle_radius(&self) -> f64 {
        unsafe { sys::cpCircleShapeGetRadius(self.0) }
    }

    pub fn circle_offset(&self) -> sys::cpVect {
        unsafe { sys::cpCircleShapeGetOffset(self.0) }
    }

    pub fn segment(body: Body, a: sys::cpVect, b: sys::cpVect, radius: f64) -> Shape {
        unsafe { Shape(sys::cpSegmentShapeNew(body.0, a, b, radius)) }
    }

    pub fn segment_a(&self) -> sys::cpVect {
        unsafe { sys::cpSegmentShapeGetA(self.0) }
    }

    pub fn segment_b(&self) -> sys::cpVect {
        unsafe { sys::cpSegmentShapeGetB(self.0) }
    }

    pub fn segment_radius(&self) -> f64 {
        unsafe { sys::cpSegmentShapeGetRadius(self.0) }
    }

    pub fn segment_normal(&self) -> sys::cpVect {
        unsafe { sys::cpSegmentShapeGetNormal(self.0) }
    }
}

impl Drop for Shape {
    fn drop(&mut self) {
        unsafe {
            sys::cpShapeFree(self.0);
        }
    }
}
