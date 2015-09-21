extern crate rand;

mod vec3;

use vec3::Vec3;
use atoms::{Atom, Atoms};

fn main() {
    let mut a1 = Atom::new(0., 0., 0., 1., 1.);
    let mut a2 = Atom::new(5., 5., 5., 1., 1.);
    let sys = Atoms::new(vec![a1, a2]);
    println!("{}", sys.atoms[0].position.x);
//    for (i, atomi) in b.atoms {
//        for (j, atomj) in b.atoms {
//            if i == j {continue};
//            let dr = atomj.position - atomi.position;
//            let r = dr.len();
//            println!("{}", r);
//        }
//    }
}
