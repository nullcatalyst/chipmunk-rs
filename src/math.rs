extern crate chipmunk_sys as sys;

use crate::vect::*;

/// Calculate the moment of inertia for a circle.
/// @c r1 and @c r2 are the inner and outer diameters. A solid circle has an inner diameter of 0.
#[inline]
pub fn moment_for_circle(mass: f64, inner_radius: f64, outer_radius: f64, offset: Vect) -> f64 {
    unsafe { sys::cpMomentForCircle(mass, inner_radius, outer_radius, offset.0) }
}

/// Calculate area of a hollow circle.
/// @c r1 and @c r2 are the inner and outer diameters. A solid circle has an inner diameter of 0.
#[inline]
pub fn area_for_circle(inner_radius: f64, outer_radius: f64) -> f64 {
    unsafe { sys::cpAreaForCircle(inner_radius, outer_radius) }
}

/// Calculate the moment of inertia for a line segment.
/// Beveling radius is not supported.
#[inline]
pub fn moment_for_segment(mass: f64, a: Vect, b: Vect, radius: f64) -> f64 {
    unsafe { sys::cpMomentForSegment(mass, a.0, b.0, radius) }
}

/// Calculate the area of a fattened (capsule shaped) line segment.
#[inline]
pub fn area_for_segment(a: Vect, b: Vect, radius: f64) -> f64 {
    unsafe { sys::cpAreaForSegment(a.0, b.0, radius) }
}

/// Calculate the moment of inertia for a solid box.
#[inline]
pub fn moment_for_box(mass: f64, width: f64, height: f64) -> f64 {
    unsafe { sys::cpMomentForBox(mass, width, height) }
}
