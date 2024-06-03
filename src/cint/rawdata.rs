use super::{cdata::CintBasis, libccint::CINTgto_norm};

#[derive(Debug, Clone)]
pub(crate) struct CGTO {
    pub(crate) kappa_of: i8,
    pub(crate) angl: Vec<u8>,
    pub(crate) exp: Vec<f64>,
    pub(crate) coeff: Vec<Vec<f64>>,
}

impl CGTO {
    pub(crate) fn norm(mut self) -> Self {
        // pub fn CINTgto_norm(n: ::std::os::raw::c_int, a: f64) -> f64;
        self.coeff = self
            .coeff
            .iter()
            .enumerate()
            .map(|(i, coeff)| {
                coeff
                    .iter()
                    .enumerate()
                    .map(|(ic, c)| c * unsafe { CINTgto_norm(self.angl[i].into(), self.exp[ic]) })
                    .collect()
            })
            .collect();
        self
    }

    pub(crate) fn gen_bas(&self, iangl: usize, ptr_exp: i32, ptr_coeff: i32) -> CintBasis {
        CintBasis {
            atom_of: -1,
            ang_of: self.angl[iangl].into(),
            nprim_of: self.exp.len() as i32,
            nctr_of: 1,
            kappa_of: self.kappa_of as i32,
            ptr_exp,
            ptr_coeff,
            reserve_baslot: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AtomGroup {
    pub(crate) basis: Option<Vec<CGTO>>,
    pub(crate) charge_of: u8,
    pub(crate) nuc_mod_of: u8,
    pub(crate) zeta: f64,
    pub(crate) frac_charge: f64,
    pub(crate) coordinates: Vec<[f64; 3]>,
}
