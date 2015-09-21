extern crate rand;


mod vec3;
mod atoms;

use vec3::Vec3;
use atoms::{Atom, Atoms};

fn main() {
    let mut a1 = Atom::new(0., 0., 0., 1., 1.);
    let mut a2 = Atom::new(5., 5., 5., 1., 1.);
    let mut sys = a1 + a2;
    sys = sys + Atom::new(1., 1., 1., 1., 1.,);
    for (i, atomi) in sys.enumerate() {
        for (j, atomj) in sys.enumerate() {
            if i == j { continue };
            let dr = atomj.position - atomi.position;
            let r = dr.len();
            println!("{}", r);
        }
    }
}
