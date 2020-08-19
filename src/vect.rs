extern crate chipmunk_sys as sys;

#[derive(Copy, Clone)]
pub struct Vect(pub sys::cpVect);

impl Vect {
    pub fn new(x: f64, y: f64) -> Vect {
        Vect(sys::cpVect { x, y })
    }

    pub fn zero() -> Vect {
        Vect(sys::cpVect { x: 0.0, y: 0.0 })
    }

    pub fn x(&self) -> f64 {
        self.0.x
    }

    pub fn set_x(&mut self, x: f64) {
        self.0.x = x;
    }

    pub fn y(&self) -> f64 {
        self.0.y
    }

    pub fn set_y(&mut self, y: f64) {
        self.0.y = y;
    }
}

impl From<sys::cpVect> for Vect {
    fn from(v: sys::cpVect) -> Self {
        Vect(v)
    }
}

impl From<Vect> for sys::cpVect {
    fn from(v: Vect) -> Self {
        v.0
    }
}
