use std::{slice, iter};
use std::ops::Add;
use vec3::Vec3;

#[derive(Clone)]
pub struct Atom {
    pub position: Vec3,
    pub epsilon: f64,
    pub sigma: f64,
}

impl Atom {
    pub fn new(x: f64, y: f64, z: f64, epsilon: f64, sigma: f64) -> Atom {
        Atom {
            position: Vec3{x: x, y: y, z: z},
            sigma: sigma,
            epsilon: epsilon,
        }
    }
}

#[derive(Clone)]
pub struct Atoms {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub atoms: Vec<Atom>,
}

impl Atoms {
    pub fn new(atoms: Vec<Atom>) -> Atoms {
        Atoms {
            a: 0.,
            b: 0.,
            c: 0.,
            atoms: atoms,
        }
    }

    pub fn push(&mut self, item: Atom) {
        self.atoms.push(item)
    }

    pub fn pop(&mut self) -> Option<Atom> {
        self.atoms.pop()
    }

    pub fn enumerate(&self) -> iter::Enumerate<slice::Iter<Atom>> {
        self.atoms.iter().enumerate()
    }
}

impl Add<Atom> for Atom {
    type Output = Atoms;

    fn add(self, other: Atom) -> Atoms {
        Atoms::new(vec![self, other])
    }
}

impl Add<Atom> for Atoms {
    type Output = Atoms;

    fn add(self, other: Atom) -> Atoms {
        let mut new = self.clone();
        new.push(other.clone());
        new
    }
}

impl Add<Atoms> for Atom {
    type Output = Atoms;

    fn add(self, other: Atoms) -> Atoms {
        let mut new = other.clone();
        new.atoms = vec![self];
        for atom in other.atoms {
            new.push(atom.clone());
        }
        new
    }
}

impl Add<Atoms> for Atoms {
    type Output = Atoms;

    fn add(self, other: Atoms) -> Atoms {
        let mut new = self.clone();
        for atom in other.atoms {
            new.push(atom.clone());
        }
        new
    }
}

//impl IntoIterator for Atoms {
//    type Item = Atom;
//    type IntoIter = AtomsIntoIter;
//
//    fn into_iter(self) -> Self::IntoIter {
//        AtomsIntoIter { atoms: self, index: 0 }
//    }
//}
//
//struct AtomsIntoIter {
//    atoms: Atoms,
//    index: usize,
//}
//
//impl Iterator for AtomsIntoIter {
//    type Item = Atom;
//    fn next(&mut self) -> Option<Atom> {
//

// This adds support for ```
// for atom in &my_atoms {
//     ...
// }```
impl<'a> IntoIterator for &'a Atoms {
    type Item = &'a Atom;
    type IntoIter = slice::Iter<'a, Atom>;

    fn into_iter(self) -> slice::Iter<'a, Atom> {
        self.atoms.iter()
    }
}
