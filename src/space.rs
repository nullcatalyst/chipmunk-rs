extern crate chipmunk_sys as sys;

use crate::Body;
use std::ffi;

pub struct Space(*mut sys::cpSpace);

unsafe impl Send for Space {}

impl Space {
    pub fn new() -> Space {
        Space(unsafe { sys::cpSpaceNew() })
    }

    //MARK: Properties

    /// Number of iterations to use in the impulse solver to solve contacts and other constraints.
    pub fn iterations(&self) -> i32 {
        unsafe { sys::cpSpaceGetIterations(self.0) }
    }

    pub fn set_iterations(&mut self, iterations: i32) {
        unsafe { sys::cpSpaceSetIterations(self.0, iterations) }
    }

    /// Gravity to pass to rigid bodies when integrating velocity.
    pub fn gravity(&self) -> sys::cpVect {
        unsafe { sys::cpSpaceGetGravity(self.0) }
    }

    pub fn set_gravity(&mut self, gravity: sys::cpVect) {
        unsafe { sys::cpSpaceSetGravity(self.0, gravity) }
    }

    /// Damping rate expressed as the fraction of velocity bodies retain each second.
    /// A value of 0.9 would mean that each body's velocity will drop 10% per second.
    /// The default value is 1.0, meaning no damping is applied.
    /// @note This damping value is different than those of cpDampedSpring and cpDampedRotarySpring.
    pub fn damping(&self) -> f64 {
        unsafe { sys::cpSpaceGetDamping(self.0) }
    }

    pub fn set_damping(&mut self, damping: f64) {
        unsafe { sys::cpSpaceSetDamping(self.0, damping) }
    }

    /// Speed threshold for a body to be considered idle.
    /// The default value of 0 means to let the space guess a good threshold based on gravity.
    pub fn idle_speed_threshold(&self) -> f64 {
        unsafe { sys::cpSpaceGetIdleSpeedThreshold(self.0) }
    }

    pub fn set_idle_speed_threshold(&mut self, idle_speed_threshold: f64) {
        unsafe { sys::cpSpaceSetIdleSpeedThreshold(self.0, idle_speed_threshold) }
    }

    /// Time a group of bodies must remain idle in order to fall asleep.
    /// Enabling sleeping also implicitly enables the the contact graph.
    /// The default value of INFINITY disables the sleeping algorithm.
    pub fn sleep_time_threshold(&self) -> f64 {
        unsafe { sys::cpSpaceGetSleepTimeThreshold(self.0) }
    }

    pub fn set_sleep_time_threshold(&mut self, sleep_time_threshold: f64) {
        unsafe { sys::cpSpaceSetSleepTimeThreshold(self.0, sleep_time_threshold) }
    }

    /// Amount of encouraged penetration between colliding shapes.
    /// Used to reduce oscillating contacts and keep the collision cache warm.
    /// Defaults to 0.1. If you have poor simulation quality,
    /// increase this number as much as possible without allowing visible amounts of overlap.
    pub fn collision_slop(&self) -> f64 {
        unsafe { sys::cpSpaceGetCollisionSlop(self.0) }
    }

    pub fn set_collision_slop(&mut self, collision_slop: f64) {
        unsafe { sys::cpSpaceSetCollisionSlop(self.0, collision_slop) }
    }

    /// Determines how fast overlapping shapes are pushed apart.
    /// Expressed as a fraction of the error remaining after each second.
    /// Defaults to pow(1.0 - 0.1, 60.0) meaning that Chipmunk fixes 10% of overlap each frame at 60Hz.
    pub fn collision_bias(&self) -> f64 {
        unsafe { sys::cpSpaceGetCollisionSlop(self.0) }
    }

    pub fn set_collision_bias(&mut self, collision_bias: f64) {
        unsafe { sys::cpSpaceSetCollisionBias(self.0, collision_bias) }
    }

    /// Number of frames that contact information should persist.
    /// Defaults to 3. There is probably never a reason to change this value.
    pub fn collision_persistance(&self) -> u32 {
        unsafe { sys::cpSpaceGetCollisionPersistence(self.0) }
    }

    pub fn set_collision_persistance(&mut self, collision_persistance: u32) {
        unsafe { sys::cpSpaceSetCollisionPersistence(self.0, collision_persistance) }
    }

    /// User definable data pointer.
    /// Generally this points to your game's controller or game state
    /// class so you can access it when given a cpSpace reference in a callback.
    pub fn user_data(&self) -> *mut ffi::c_void {
        unsafe { sys::cpSpaceGetUserData(self.0) }
    }

    pub fn set_user_data(&mut self, user_data: *mut ffi::c_void) {
        unsafe { sys::cpSpaceSetUserData(self.0, user_data) }
    }

    /// The Space provided static body for a given cpSpace.
    /// This is merely provided for convenience and you are not required to use it.
    pub fn static_body(&self) -> Body {
        Body(unsafe { sys::cpSpaceGetStaticBody(self.0) })
    }

    /// Returns the current (or most recent) time step used with the given space.
    /// Useful from callbacks if your time step is not a compile-time global.
    pub fn current_time_step(&self) -> f64 {
        unsafe { sys::cpSpaceGetCurrentTimeStep(self.0) }
    }

    /// returns true from inside a callback when objects cannot be added/removed.
    pub fn is_locked(&self) -> bool {
        unsafe { sys::cpSpaceIsLocked(self.0) != 0 }
    }

    // //MARK: Collision Handlers

    // /// Create or return the existing collision handler that is called for all collisions that are not handled by a more specific collision handler.
    // CP_EXPORT cpCollisionHandler *cpSpaceAddDefaultCollisionHandler(cpSpace *space);
    // /// Create or return the existing collision handler for the specified pair of collision types.
    // /// If wildcard handlers are used with either of the collision types, it's the responibility of the custom handler to invoke the wildcard handlers.
    // CP_EXPORT cpCollisionHandler *cpSpaceAddCollisionHandler(cpSpace *space, cpCollisionType a, cpCollisionType b);
    // /// Create or return the existing wildcard collision handler for the specified type.
    // CP_EXPORT cpCollisionHandler *cpSpaceAddWildcardHandler(cpSpace *space, cpCollisionType type);

    //MARK: Add/Remove objects

    /// Add a collision shape to the simulation.
    /// If the shape is attached to a static body, it will be added as a static shape.
    pub fn add_shape(&mut self, shape: *mut sys::cpShape) {
        unsafe { sys::cpSpaceAddShape(self.0, shape) };
    }
    /// Add a rigid body to the simulation.
    pub fn add_body(&mut self, body: *mut sys::cpBody) {
        unsafe { sys::cpSpaceAddBody(self.0, body) };
    }

    /// Add a constraint to the simulation.
    pub fn add_constraint(&mut self, constraint: *mut sys::cpConstraint) {
        unsafe { sys::cpSpaceAddConstraint(self.0, constraint) };
    }

    /// Remove a collision shape from the simulation.
    pub fn remove_shape(&mut self, shape: *mut sys::cpShape) {
        unsafe { sys::cpSpaceRemoveShape(self.0, shape) };
    }

    /// Remove a rigid body from the simulation.
    pub fn remove_body(&mut self, body: *mut sys::cpBody) {
        unsafe { sys::cpSpaceRemoveBody(self.0, body) };
    }

    /// Remove a constraint from the simulation.
    pub fn remove_constraint(&mut self, constraint: *mut sys::cpConstraint) {
        unsafe { sys::cpSpaceRemoveConstraint(self.0, constraint) };
    }

    // /// Test if a collision shape has been added to the space.
    // CP_EXPORT cpBool cpSpaceContainsShape(cpSpace *space, cpShape *shape);
    // /// Test if a rigid body has been added to the space.
    // CP_EXPORT cpBool cpSpaceContainsBody(cpSpace *space, cpBody *body);
    // /// Test if a constraint has been added to the space.
    // CP_EXPORT cpBool cpSpaceContainsConstraint(cpSpace *space, cpConstraint *constraint);

    // //MARK: Post-Step Callbacks

    // /// Post Step callback function type.
    // typedef void (*cpPostStepFunc)(cpSpace *space, void *key, void *data);
    // /// Schedule a post-step callback to be called when cpSpaceStep() finishes.
    // /// You can only register one callback per unique value for @c key.
    // /// Returns true only if @c key has never been scheduled before.
    // /// It's possible to pass @c NULL for @c func if you only want to mark @c key as being used.
    // CP_EXPORT cpBool cpSpaceAddPostStepCallback(cpSpace *space, cpPostStepFunc func, void *key, void *data);

    // //MARK: Queries

    // // TODO: Queries and iterators should take a cpSpace parametery.
    // // TODO: They should also be abortable.

    // /// Nearest point query callback function type.
    // typedef void (*cpSpacePointQueryFunc)(cpShape *shape, cpVect point, cpFloat distance, cpVect gradient, void *data);
    // /// Query the space at a point and call @c func for each shape found.
    // CP_EXPORT void cpSpacePointQuery(cpSpace *space, cpVect point, cpFloat maxDistance, cpShapeFilter filter, cpSpacePointQueryFunc func, void *data);
    // /// Query the space at a point and return the nearest shape found. Returns NULL if no shapes were found.
    // CP_EXPORT cpShape *cpSpacePointQueryNearest(cpSpace *space, cpVect point, cpFloat maxDistance, cpShapeFilter filter, cpPointQueryInfo *out);

    // /// Segment query callback function type.
    // typedef void (*cpSpaceSegmentQueryFunc)(cpShape *shape, cpVect point, cpVect normal, cpFloat alpha, void *data);
    // /// Perform a directed line segment query (like a raycast) against the space calling @c func for each shape intersected.
    // CP_EXPORT void cpSpaceSegmentQuery(cpSpace *space, cpVect start, cpVect end, cpFloat radius, cpShapeFilter filter, cpSpaceSegmentQueryFunc func, void *data);
    // /// Perform a directed line segment query (like a raycast) against the space and return the first shape hit. Returns NULL if no shapes were hit.
    // CP_EXPORT cpShape *cpSpaceSegmentQueryFirst(cpSpace *space, cpVect start, cpVect end, cpFloat radius, cpShapeFilter filter, cpSegmentQueryInfo *out);

    // /// Rectangle Query callback function type.
    // typedef void (*cpSpaceBBQueryFunc)(cpShape *shape, void *data);
    // /// Perform a fast rectangle query on the space calling @c func for each shape found.
    // /// Only the shape's bounding boxes are checked for overlap, not their full shape.
    // CP_EXPORT void cpSpaceBBQuery(cpSpace *space, cpBB bb, cpShapeFilter filter, cpSpaceBBQueryFunc func, void *data);

    // /// Shape query callback function type.
    // typedef void (*cpSpaceShapeQueryFunc)(cpShape *shape, cpContactPointSet *points, void *data);
    // /// Query a space for any shapes overlapping the given shape and call @c func for each shape found.
    // CP_EXPORT cpBool cpSpaceShapeQuery(cpSpace *space, cpShape *shape, cpSpaceShapeQueryFunc func, void *data);

    // //MARK: Iteration

    // /// Space/body iterator callback function type.
    // typedef void (*cpSpaceBodyIteratorFunc)(cpBody *body, void *data);
    // /// Call @c func for each body in the space.
    // CP_EXPORT void cpSpaceEachBody(cpSpace *space, cpSpaceBodyIteratorFunc func, void *data);

    // /// Space/body iterator callback function type.
    // typedef void (*cpSpaceShapeIteratorFunc)(cpShape *shape, void *data);
    // /// Call @c func for each shape in the space.
    // CP_EXPORT void cpSpaceEachShape(cpSpace *space, cpSpaceShapeIteratorFunc func, void *data);

    // /// Space/constraint iterator callback function type.
    // typedef void (*cpSpaceConstraintIteratorFunc)(cpConstraint *constraint, void *data);
    // /// Call @c func for each shape in the space.
    // CP_EXPORT void cpSpaceEachConstraint(cpSpace *space, cpSpaceConstraintIteratorFunc func, void *data);

    // //MARK: Indexing

    // /// Update the collision detection info for the static shapes in the space.
    // CP_EXPORT void cpSpaceReindexStatic(cpSpace *space);
    // /// Update the collision detection data for a specific shape in the space.
    // CP_EXPORT void cpSpaceReindexShape(cpSpace *space, cpShape *shape);
    // /// Update the collision detection data for all shapes attached to a body.
    // CP_EXPORT void cpSpaceReindexShapesForBody(cpSpace *space, cpBody *body);

    // /// Switch the space to use a spatial has as it's spatial index.
    // CP_EXPORT void cpSpaceUseSpatialHash(cpSpace *space, cpFloat dim, int count);

    //MARK: Time Stepping

    /// Step the space forward in time by @c dt.
    pub fn step(&mut self, dt: f64) {
        unsafe { sys::cpSpaceStep(self.0, dt) }
    }
}

impl Drop for Space {
    fn drop(&mut self) {
        unsafe {
            sys::cpSpaceFree(self.0);
        }
    }
}
