use std::collections::BTreeMap;

use super::super::convert::aatoau;
use super::super::PERIODIC_TABLE;
use crate::cint::{
    rawdata::{AtomGroup, CGTO},
    NAtom,
};

impl AtomGroup {
    pub(crate) fn from_xyz(
        xyz_str: Vec<&str>,
        basis: Option<BTreeMap<u8, Vec<CGTO>>>,
    ) -> Vec<Self> {
        let mut natm: NAtom = 0;
        match xyz_str[0].parse() {
            Err(why) => panic!("{:?}", why),
            Ok(v) => natm = v,
        }
        assert_eq!(natm + 2, xyz_str.len());

        let mut atoms_map: BTreeMap<u8, Vec<[f64; 3]>> = BTreeMap::new();

        xyz_str[2..xyz_str.len()].iter().for_each(|line| {
            let mut split_s = line.trim().split_whitespace();
            let nuc = match split_s.next() {
                Some(symbol) => match PERIODIC_TABLE.iter().position(|ele| *ele == symbol) {
                    None => panic!(""),
                    Some(nuc) => (nuc + 1) as u8,
                },
                None => panic!(""),
            };

            let mut coor = [0.0; 3];
            for (i, x_str) in split_s.enumerate() {
                match x_str.parse() {
                    Ok(x) => coor[i] = aatoau(x),
                    Err(why) => panic!(""),
                }
            }

            match atoms_map.get_mut(&nuc) {
                Some(atom_coors) => atom_coors.push(coor),
                None => {
                    atoms_map.insert(nuc, vec![coor]);
                    ()
                }
            }
        });

        atoms_map
            .iter()
            .map(|(nuc, coors)| Self {
                basis: match &basis {
                    Some(bas) => match bas.get(&nuc) {
                        Some(b) => Some(b.to_vec()),
                        None => panic!(""),
                    },
                    None => None,
                },
                charge_of: *nuc,
                nuc_mod_of: 0,
                zeta: 0.0,
                frac_charge: 0.0,
                coordinates: coors.to_vec(),
            })
            .collect()
    }
}
