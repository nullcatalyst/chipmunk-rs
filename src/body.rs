extern crate chipmunk_sys as sys;

use crate::vect::*;
use std::ffi;

pub struct Body(pub *mut sys::cpBody, pub bool);

unsafe impl Send for Body {}

impl Body {
    pub unsafe fn null() -> Body {
        Body(std::ptr::null_mut(), false)
    }

    pub fn new(mass: f64, moment: f64) -> Body {
        Body(unsafe { sys::cpBodyNew(mass, moment) }, true)
    }

    pub fn new_kinematic() -> Body {
        Body(unsafe { sys::cpBodyNewKinematic() }, true)
    }

    pub fn new_static() -> Body {
        Body(unsafe { sys::cpBodyNewStatic() }, true)
    }

    /// Wake up a sleeping or idle body.
    pub fn activate(&mut self) {
        unsafe { sys::cpBodyActivate(self.0) };
    }

    /// Wake up any sleeping or idle bodies touching a static body.
    // CP_EXPORT void cpBodyActivateStatic(cpBody *body, cpShape *filter);

    /// Force a body to fall asleep immediately.
    pub fn sleep(&mut self) {
        unsafe { sys::cpBodySleep(self.0) };
    }

    /// Force a body to fall asleep immediately along with other bodies in a group.
    // CP_EXPORT void cpBodySleepWithGroup(cpBody *body, cpBody *group);

    /// Returns true if the body is sleeping.
    pub fn is_sleeping(&mut self) -> bool {
        unsafe { sys::cpBodyIsSleeping(self.0) != 0 }
    }

    /// Get the type of the body.
    // CP_EXPORT cpBodyType cpBodyGetType(cpBody *body);
    /// Set the type of the body.
    // CP_EXPORT void cpBodySetType(cpBody *body, cpBodyType type);

    /// Get the space this body is added to.
    // CP_EXPORT cpSpace* cpBodyGetSpace(const cpBody *body);

    /// Get the mass of the body.
    pub fn mass(&self) -> f64 {
        unsafe { sys::cpBodyGetMass(self.0) }
    }

    /// Set the mass of the body.
    pub fn set_mass(&mut self, mass: f64) {
        unsafe { sys::cpBodySetMass(self.0, mass) }
    }

    /// Get the moment of inertia of the body.
    pub fn moment(&self) -> f64 {
        unsafe { sys::cpBodyGetMoment(self.0) }
    }

    /// Set the moment of inertia of the body.
    pub fn set_moment(&mut self, moment: f64) {
        unsafe { sys::cpBodySetMoment(self.0, moment) }
    }

    /// Set the position of a body.
    pub fn position(&self) -> Vect {
        unsafe { sys::cpBodyGetPosition(self.0) }.into()
    }

    /// Set the position of the body.
    pub fn set_position(&mut self, position: Vect) {
        unsafe { sys::cpBodySetPosition(self.0, position.0) }
    }

    /// Get the offset of the center of gravity in body local coordinates.
    pub fn center_of_gravity(&self) -> Vect {
        unsafe { sys::cpBodyGetCenterOfGravity(self.0) }.into()
    }

    /// Set the offset of the center of gravity in body local coordinates.
    pub fn set_center_of_gravity(&mut self, center_of_gravity: Vect) {
        unsafe { sys::cpBodySetCenterOfGravity(self.0, center_of_gravity.0) }
    }

    /// Get the velocity of the body.
    pub fn velocity(&self) -> Vect {
        unsafe { sys::cpBodyGetVelocity(self.0) }.into()
    }

    /// Set the velocity of the body.
    pub fn set_velocity(&mut self, velocity: Vect) {
        unsafe { sys::cpBodySetVelocity(self.0, velocity.0) }
    }

    /// Get the force applied to the body for the next time step.
    pub fn force(&self) -> Vect {
        unsafe { sys::cpBodyGetForce(self.0) }.into()
    }

    /// Set the force applied to the body for the next time step.
    pub fn set_force(&mut self, force: Vect) {
        unsafe { sys::cpBodySetForce(self.0, force.0) }
    }

    /// Get the angle of the body.
    pub fn angle(&self) -> f64 {
        unsafe { sys::cpBodyGetAngle(self.0) }
    }

    /// Set the angle of a body.
    pub fn set_angle(&mut self, angle: f64) {
        unsafe { sys::cpBodySetAngle(self.0, angle) }
    }

    /// Get the angular velocity of the body.
    pub fn angular_velocity(&self) -> f64 {
        unsafe { sys::cpBodyGetAngularVelocity(self.0) }
    }

    /// Set the angular velocity of the body.
    pub fn set_angular_velocity(&mut self, angular_velocity: f64) {
        unsafe { sys::cpBodySetAngularVelocity(self.0, angular_velocity) }
    }

    /// Get the torque applied to the body for the next time step.
    pub fn torque(&self) -> f64 {
        unsafe { sys::cpBodyGetTorque(self.0) }
    }

    /// Set the torque applied to the body for the next time step.
    pub fn set_torque(&mut self, torque: f64) {
        unsafe { sys::cpBodySetTorque(self.0, torque) }
    }

    /// Get the rotation vector of the body. (The x basis vector of it's transform.)
    pub fn rotation(&self) -> Vect {
        unsafe { sys::cpBodyGetRotation(self.0) }.into()
    }

    /// Get the user data pointer assigned to the body.
    pub fn user_data(&self) -> *mut ffi::c_void {
        unsafe { sys::cpBodyGetUserData(self.0) }
    }

    /// Set the user data pointer assigned to the body.
    pub fn set_user_data(&mut self, user_data: *mut ffi::c_void) {
        unsafe { sys::cpBodySetUserData(self.0, user_data) }
    }

    // /// Set the callback used to update a body's velocity.
    // CP_EXPORT void cpBodySetVelocityUpdateFunc(cpBody *body, cpBodyVelocityFunc velocityFunc);
    // /// Set the callback used to update a body's position.
    // /// NOTE: It's not generally recommended to override this unless you call the default position update function.
    // CP_EXPORT void cpBodySetPositionUpdateFunc(cpBody *body, cpBodyPositionFunc positionFunc);

    // /// Default velocity integration function..
    // CP_EXPORT void cpBodyUpdateVelocity(cpBody *body, cpVect gravity, cpFloat damping, cpFloat dt);
    // /// Default position integration function.
    // CP_EXPORT void cpBodyUpdatePosition(cpBody *body, cpFloat dt);

    // /// Convert body relative/local coordinates to absolute/world coordinates.
    // CP_EXPORT cpVect cpBodyLocalToWorld(const cpBody *body, const cpVect point);
    // /// Convert body absolute/world coordinates to  relative/local coordinates.
    // CP_EXPORT cpVect cpBodyWorldToLocal(const cpBody *body, const cpVect point);

    // /// Apply a force to a body. Both the force and point are expressed in world coordinates.
    // CP_EXPORT void cpBodyApplyForceAtWorldPoint(cpBody *body, cpVect force, cpVect point);
    // /// Apply a force to a body. Both the force and point are expressed in body local coordinates.
    // CP_EXPORT void cpBodyApplyForceAtLocalPoint(cpBody *body, cpVect force, cpVect point);

    // /// Apply an impulse to a body. Both the impulse and point are expressed in world coordinates.
    // CP_EXPORT void cpBodyApplyImpulseAtWorldPoint(cpBody *body, cpVect impulse, cpVect point);
    // /// Apply an impulse to a body. Both the impulse and point are expressed in body local coordinates.
    // CP_EXPORT void cpBodyApplyImpulseAtLocalPoint(cpBody *body, cpVect impulse, cpVect point);

    // /// Get the velocity on a body (in world units) at a point on the body in world coordinates.
    // CP_EXPORT cpVect cpBodyGetVelocityAtWorldPoint(const cpBody *body, cpVect point);
    // /// Get the velocity on a body (in world units) at a point on the body in local coordinates.
    // CP_EXPORT cpVect cpBodyGetVelocityAtLocalPoint(const cpBody *body, cpVect point);

    // /// Get the amount of kinetic energy contained by the body.
    // CP_EXPORT cpFloat cpBodyKineticEnergy(const cpBody *body);

    // /// Body/shape iterator callback function type.
    // typedef void (*cpBodyShapeIteratorFunc)(cpBody *body, cpShape *shape, void *data);
    // /// Call @c func once for each shape attached to @c body and added to the space.
    // CP_EXPORT void cpBodyEachShape(cpBody *body, cpBodyShapeIteratorFunc func, void *data);

    // /// Body/constraint iterator callback function type.
    // typedef void (*cpBodyConstraintIteratorFunc)(cpBody *body, cpConstraint *constraint, void *data);
    // /// Call @c func once for each constraint attached to @c body and added to the space.
    // CP_EXPORT void cpBodyEachConstraint(cpBody *body, cpBodyConstraintIteratorFunc func, void *data);

    // /// Body/arbiter iterator callback function type.
    // typedef void (*cpBodyArbiterIteratorFunc)(cpBody *body, cpArbiter *arbiter, void *data);
    // /// Call @c func once for each arbiter that is currently active on the body.
    // CP_EXPORT void cpBodyEachArbiter(cpBody *body, cpBodyArbiterIteratorFunc func, void *data);
}

impl Drop for Body {
    fn drop(&mut self) {
        if self.1 {
            unsafe { sys::cpBodyFree(self.0) }
        }
    }
}
