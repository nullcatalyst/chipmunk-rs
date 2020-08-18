extern crate chipmunk_sys as sys;

/// Calculate the moment of inertia for a circle.
/// @c r1 and @c r2 are the inner and outer diameters. A solid circle has an inner diameter of 0.
#[inline]
pub fn moment_for_circle(
    mass: f64,
    inner_radius: f64,
    outer_radius: f64,
    offset: sys::cpVect,
) -> f64 {
    unsafe { sys::cpMomentForCircle(mass, inner_radius, outer_radius, offset) }
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
pub fn moment_for_segment(mass: f64, a: sys::cpVect, b: sys::cpVect, radius: f64) -> f64 {
    unsafe { sys::cpMomentForSegment(mass, a, b, radius) }
}

/// Calculate the area of a fattened (capsule shaped) line segment.
#[inline]
pub fn area_for_segment(a: sys::cpVect, b: sys::cpVect, radius: f64) -> f64 {
    unsafe { sys::cpAreaForSegment(a, b, radius) }
}

/// Calculate the moment of inertia for a solid box.
#[inline]
pub fn moment_for_box(mass: f64, width: f64, height: f64) -> f64 {
    unsafe { sys::cpMomentForBox(mass, width, height) }
}
