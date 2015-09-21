import csv
import json

masses = {
    'H':  1.00782503207,
    'He': 4.00260325415,
    'Li': 7.01600455,
    'Be': 9.0121822,
    'B':  11.0093054,
    'C':  12.0000000,
    'N':  14.0030740048,
    'O':  15.99491461956,
    'F':  18.99840322,
    'Ne': 19.9924401754,
    'Na': 22.9897692809,
    'Mg': 23.985041700,
    'Al': 26.98153863,
    'Si': 27.9769265325,
    'P':  30.97376163,
    'S':  31.97207100,
    'Cl': 34.96885268,
    'Ar': 39.9623831225,
    'K':  38.96370668,
    'Ca': 39.96259098,
    'Sc': 44.9559119,
    'Ti': 47.9479463,
    'V':  50.9439595,
    'Cr': 51.9405075,
    'Mn': 54.9380451,
    'Fe': 55.9349375,
    'Co': 58.9331950,
    'Ni': 57.9353429,
    'Cu': 62.9295975,
    'Zn': 63.9291422,
    'Ga': 68.9255736,
    'Ge': 73.9211778,
    'As': 74.9215965,
    'Se': 79.9165213,
    'Br': 78.9183371,
    'Kr': 83.911507,
    'Rb': 84.911789738,
    'Sr': 87.9056121,
    'Y':  88.9058483,
    'Zr': 89.9047044,
    'Nb': 92.9063781,
    'Mo': 97.9054082,
    'Tc': 97.907216,
    'Ru': 101.9043493,
    'Rh': 102.905504,
    'Pd': 105.903486,
    'Ag': 106.905097,
    'Cd': 113.9033585,
    'In': 114.903878,
    'Sn': 119.9021947,
    'Sb': 120.9038157,
    'Te': 129.9062244,
    'I':  126.904473,
    'Xe': 131.9041535,
    'Cs': 132.905451933,
    'Ba': 137.9052472,
    'La': 138.9063533,
    'Ce': 139.9054387,
    'Pr': 140.9076528,
    'Nd': 141.9077233,
    'Pm': 144.912749,
    'Sm': 151.9197324,
    'Eu': 152.9212303,
    'Gd': 157.9241039,
    'Tb': 158.9253468,
    'Dy': 163.9291748,
    'Ho': 164.9303221,
    'Er': 165.9302931,
    'Tm': 168.9342133,
    'Yb': 173.9388621,
    'Lu': 174.9407718,
    'Hf': 179.9465500,
    'Ta': 180.9479958,
    'W':  183.9509312,
    'Re': 186.9557531,
    'Os': 191.9614807,
    'Ir': 192.9629264,
    'Pt': 194.9647911,
    'Au': 196.9665687,
    'Hg': 201.9706430,
    'Tl': 204.9744275,
    'Pb': 207.9766521,
    'Bi': 208.9803987,
    'Po': 208.9824304,
    'At': 209.987148,
    'Rn': 222.0175777,
    'Fr': 223.0197359,
    'Ra': 226.0254098,
    'Ac': 227.0277521,
    'Th': 232.0380553,
    'Pa': 231.0358840,
    'U':  238.0507882,
    'Np': 237.0481734,
    'Pu': 244.064204,
    'Am': 243.0613811,
    'Cm': 247.070354,
    'Bk': 247.070307,
    'Cf': 251.079587,
    'Es': 252.082980,
    'Fm': 257.095105,
    'Md': 258.098431,
    'No': 259.10103,
    'Lr': 262.10963,
    'Rf': 261.10877,
    'Db': 262.11408,
    'Sg': 263.11832,
    'Bh': 264.1246,
    'Hs': 265.13009,
    'Mt': 268.13873,
    'Ds': 271.14606,
    'Rg': 272.15362,
    'Cn': None,
}

with open('pt-data1.csv') as fh:
    all_elems = [
        elem for elem in csv.reader(fh)
        if masses.get(elem[1].strip()) is not None]
    print("#[derive(Copy, Clone)]")
    print("pub enum Element {")
    for elem in all_elems:
        print("    {} = {},".format(elem[1].strip(), elem[0]))
    print("}")
    print("")

    print("impl Element {")
    print("    fn atomic_number(&self) -> u16 { *self as u16 }")
    print("")
    print("    fn symbol(&self) -> &'static str {")
    print("        use self::Element::*;")
    print("        match *self {")
    for elem in all_elems:
        sym = elem[1].strip()
        print("            {} => {},".format(sym, json.dumps(sym)))
    print("        }")
    print("    }")
    print("")
    print("    fn atomic_mass(&self) -> f64 {")
    print("        use self::Element::*;")
    print("        match *self {")
    for elem in all_elems:
        sym = elem[1].strip()
        print("            {} => {},".format(sym, masses.get(sym)))
    print("        }")
    print("    }")
    print("}")


    # for elem in all_elems:
    #     anum = elem[0]
    #     symbol = elem[1].strip()

    #     mass = masses.get(symbol)
    #     if mass is not None:
    #         print("struct {};".format(symbol))
    #         print("impl_element!({}, {}, {}, {});".format(
    #             symbol, anum, json.dumps(symbol), mass))
    #         print("")
    # print("}")


# pub enum Element {

# }

# impl Element {
#     fn atomic_number(&self) -> u16 {
#         *self as u16
#     }

#     
#         use self::Element::*;
#         match *self {
#             Hydrogen => "H",
#             Helium => "He",
#             Lithium => "Li",
#             Beryllium => "Be",
#         }
#     }

#     fn atomic_mass(&self) -> f64 {
#         use self::Element::*;
#         match *self {
#             Hydrogen => 1.007944,
#             Helium => 4.0026022,
#             Lithium => 6.9412,
#             Beryllium => 9.0121823,
#         }
#     }
# }