extern crate rand;

use std::{slice, iter};
use vec3::Vec3;

pub struct Atom {
    position: Vec3,
    epsilon: f64,
    sigma: f64,
}

impl Atom {
    fn new(x: f64, y: f64, z: f64, epsilon: f64, sigma: f64) -> Atom {
        Atom {
            position: Vec3{x: x, y: y, z: z},
            sigma: sigma,
            epsilon: epsilon,
        }
    }
}

pub struct Atoms {
    a: f64,
    b: f64,
    c: f64,
    atoms: Vec<Atom>,
}

impl Atoms {
    fn new(atoms: Vec<Atom>) -> Atoms {
        Atoms {
            a: 0.,
            b: 0.,
            c: 0.,
            atoms: atoms,
        }
    }

//    fn atoms(&self) -> iter::Enumerate<slice::Iter<'_, Atom>> {
//        self.atoms.iter().enumerate()
//    }

}
