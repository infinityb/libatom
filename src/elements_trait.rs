#![feature(associated_consts)]

macro_rules! impl_element(
    ($name:ident, $anum:expr, $sym:expr, $mass:expr) => (
        impl Element for $name {
            fn atomic_number(&self) -> u16 { $anum }
            fn symbol(&self) -> &'static str { $sym }
            #[inline(never)]
            fn atomic_mass(&self) -> f64 { $mass }
        }
    )
);

pub trait Element {
    fn atomic_number(&self) -> u16;

    fn symbol(&self) -> &'static str;

    fn atomic_mass(&self) -> f64;
}

#[inline]
fn print_element<E>(elem: E) where E: Element {
    println!("{} has atomic number {} and an atomic mass of {:?}",
        elem.symbol(),
        elem.atomic_number(),
        elem.atomic_mass());
}

struct H;
impl_element!(H, 1, "H", 1.00782503207);

struct He;
impl_element!(He, 2, "He", 4.00260325415);

struct Li;
impl_element!(Li, 3, "Li", 7.01600455);

struct Be;
impl_element!(Be, 4, "Be", 9.0121822);

struct B;
impl_element!(B, 5, "B", 11.0093054);

struct C;
impl_element!(C, 6, "C", 12.0);

struct N;
impl_element!(N, 7, "N", 14.0030740048);

struct O;
impl_element!(O, 8, "O", 15.9949146196);

struct F;
impl_element!(F, 9, "F", 18.99840322);

struct Ne;
impl_element!(Ne, 10, "Ne", 19.9924401754);

struct Na;
impl_element!(Na, 11, "Na", 22.9897692809);

struct Mg;
impl_element!(Mg, 12, "Mg", 23.9850417);

struct Al;
impl_element!(Al, 13, "Al", 26.98153863);

struct Si;
impl_element!(Si, 14, "Si", 27.9769265325);

struct P;
impl_element!(P, 15, "P", 30.97376163);

struct S;
impl_element!(S, 16, "S", 31.972071);

struct Cl;
impl_element!(Cl, 17, "Cl", 34.96885268);

struct Ar;
impl_element!(Ar, 18, "Ar", 39.9623831225);

struct K;
impl_element!(K, 19, "K", 38.96370668);

struct Ca;
impl_element!(Ca, 20, "Ca", 39.96259098);

struct Sc;
impl_element!(Sc, 21, "Sc", 44.9559119);

struct Ti;
impl_element!(Ti, 22, "Ti", 47.9479463);

struct V;
impl_element!(V, 23, "V", 50.9439595);

struct Cr;
impl_element!(Cr, 24, "Cr", 51.9405075);

struct Mn;
impl_element!(Mn, 25, "Mn", 54.9380451);

struct Fe;
impl_element!(Fe, 26, "Fe", 55.9349375);

struct Co;
impl_element!(Co, 27, "Co", 58.933195);

struct Ni;
impl_element!(Ni, 28, "Ni", 57.9353429);

struct Cu;
impl_element!(Cu, 29, "Cu", 62.9295975);

struct Zn;
impl_element!(Zn, 30, "Zn", 63.9291422);

struct Ga;
impl_element!(Ga, 31, "Ga", 68.9255736);

struct Ge;
impl_element!(Ge, 32, "Ge", 73.9211778);

struct As;
impl_element!(As, 33, "As", 74.9215965);

struct Se;
impl_element!(Se, 34, "Se", 79.9165213);

struct Br;
impl_element!(Br, 35, "Br", 78.9183371);

struct Kr;
impl_element!(Kr, 36, "Kr", 83.911507);

struct Rb;
impl_element!(Rb, 37, "Rb", 84.911789738);

struct Sr;
impl_element!(Sr, 38, "Sr", 87.9056121);

struct Y;
impl_element!(Y, 39, "Y", 88.9058483);

struct Zr;
impl_element!(Zr, 40, "Zr", 89.9047044);

struct Nb;
impl_element!(Nb, 41, "Nb", 92.9063781);

struct Mo;
impl_element!(Mo, 42, "Mo", 97.9054082);

struct Tc;
impl_element!(Tc, 43, "Tc", 97.907216);

struct Ru;
impl_element!(Ru, 44, "Ru", 101.9043493);

struct Rh;
impl_element!(Rh, 45, "Rh", 102.905504);

struct Pd;
impl_element!(Pd, 46, "Pd", 105.903486);

struct Ag;
impl_element!(Ag, 47, "Ag", 106.905097);

struct Cd;
impl_element!(Cd, 48, "Cd", 113.9033585);

struct In;
impl_element!(In, 49, "In", 114.903878);

struct Sn;
impl_element!(Sn, 50, "Sn", 119.9021947);

struct Sb;
impl_element!(Sb, 51, "Sb", 120.9038157);

struct Te;
impl_element!(Te, 52, "Te", 129.9062244);

struct I;
impl_element!(I, 53, "I", 126.904473);

struct Xe;
impl_element!(Xe, 54, "Xe", 131.9041535);

struct Cs;
impl_element!(Cs, 55, "Cs", 132.905451933);

struct Ba;
impl_element!(Ba, 56, "Ba", 137.9052472);

struct La;
impl_element!(La, 57, "La", 138.9063533);

struct Ce;
impl_element!(Ce, 58, "Ce", 139.9054387);

struct Pr;
impl_element!(Pr, 59, "Pr", 140.9076528);

struct Nd;
impl_element!(Nd, 60, "Nd", 141.9077233);

struct Pm;
impl_element!(Pm, 61, "Pm", 144.912749);

struct Sm;
impl_element!(Sm, 62, "Sm", 151.9197324);

struct Eu;
impl_element!(Eu, 63, "Eu", 152.9212303);

struct Gd;
impl_element!(Gd, 64, "Gd", 157.9241039);

struct Tb;
impl_element!(Tb, 65, "Tb", 158.9253468);

struct Dy;
impl_element!(Dy, 66, "Dy", 163.9291748);

struct Ho;
impl_element!(Ho, 67, "Ho", 164.9303221);

struct Er;
impl_element!(Er, 68, "Er", 165.9302931);

struct Tm;
impl_element!(Tm, 69, "Tm", 168.9342133);

struct Yb;
impl_element!(Yb, 70, "Yb", 173.9388621);

struct Lu;
impl_element!(Lu, 71, "Lu", 174.9407718);

struct Hf;
impl_element!(Hf, 72, "Hf", 179.94655);

struct Ta;
impl_element!(Ta, 73, "Ta", 180.9479958);

struct W;
impl_element!(W, 74, "W", 183.9509312);

struct Re;
impl_element!(Re, 75, "Re", 186.9557531);

struct Os;
impl_element!(Os, 76, "Os", 191.9614807);

struct Ir;
impl_element!(Ir, 77, "Ir", 192.9629264);

struct Pt;
impl_element!(Pt, 78, "Pt", 194.9647911);

struct Au;
impl_element!(Au, 79, "Au", 196.9665687);

struct Hg;
impl_element!(Hg, 80, "Hg", 201.970643);

struct Tl;
impl_element!(Tl, 81, "Tl", 204.9744275);

struct Pb;
impl_element!(Pb, 82, "Pb", 207.9766521);

struct Bi;
impl_element!(Bi, 83, "Bi", 208.9803987);

struct Po;
impl_element!(Po, 84, "Po", 208.9824304);

struct At;
impl_element!(At, 85, "At", 209.987148);

struct Rn;
impl_element!(Rn, 86, "Rn", 222.0175777);

struct Fr;
impl_element!(Fr, 87, "Fr", 223.0197359);

struct Ra;
impl_element!(Ra, 88, "Ra", 226.0254098);

struct Ac;
impl_element!(Ac, 89, "Ac", 227.0277521);

struct Th;
impl_element!(Th, 90, "Th", 232.0380553);

struct Pa;
impl_element!(Pa, 91, "Pa", 231.035884);

struct U;
impl_element!(U, 92, "U", 238.0507882);

struct Np;
impl_element!(Np, 93, "Np", 237.0481734);

struct Pu;
impl_element!(Pu, 94, "Pu", 244.064204);

struct Am;
impl_element!(Am, 95, "Am", 243.0613811);

struct Cm;
impl_element!(Cm, 96, "Cm", 247.070354);

struct Bk;
impl_element!(Bk, 97, "Bk", 247.070307);

struct Cf;
impl_element!(Cf, 98, "Cf", 251.079587);

struct Es;
impl_element!(Es, 99, "Es", 252.08298);

struct Fm;
impl_element!(Fm, 100, "Fm", 257.095105);

struct Md;
impl_element!(Md, 101, "Md", 258.098431);

struct No;
impl_element!(No, 102, "No", 259.10103);

struct Lr;
impl_element!(Lr, 103, "Lr", 262.10963);

struct Rf;
impl_element!(Rf, 104, "Rf", 261.10877);

struct Db;
impl_element!(Db, 105, "Db", 262.11408);

struct Sg;
impl_element!(Sg, 106, "Sg", 263.11832);

struct Bh;
impl_element!(Bh, 107, "Bh", 264.1246);

struct Hs;
impl_element!(Hs, 108, "Hs", 265.13009);

struct Mt;
impl_element!(Mt, 109, "Mt", 268.13873);

struct Ds;
impl_element!(Ds, 110, "Ds", 271.14606);

struct Rg;
impl_element!(Rg, 111, "Rg", 272.15362);

#[bench]
fn bench_sum(b: &mut ::test::Bencher) {
    // C6H12O6
    let glucose: [&Element; 24] = [
        &C,
        &H,
        &H,
        &H,
        &H,
        &C,
        &H,
        &H,
        &H,
        &O,
        &O,
        &H,
        &H,
        &C,
        &O,
        &H,
        &C,
        &O,
        &H,
        &C,
        &H,
        &O,
        &C,
        &O,
    ];
    let glucose = ::test::black_box(&glucose);
    b.iter(|| {
        let n = ::test::black_box(1000);
        let mut acc = 0.0;
        for i in 0..n {
            acc += glucose[(i % 24)].atomic_mass();
        }
        acc
    })
}