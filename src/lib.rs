#[allow(unused)]
pub(crate) mod cint;
#[allow(unused)]
pub(crate) mod io;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn test_rawdata() {
        use crate::cint::{cdata::CintDate, libccint::int1e_ovlp_cart};

        //
        let basis_path =
            "/root/temp/cint/basis_set_exchange/basis_set_exchange/data/sto/STO-3G.1.json";
        let xyz_path = "/root/temp/cint/test.xyz";
        let cint_data = CintDate::fron_xyz(xyz_path, basis_path);

        //
        let intor_all = cint_data.gen_intor_all();
        let out = unsafe { intor_all.int_cart([0, 2], Some(int1e_ovlp_cart)) };
        println!("{:?}", out);

        //
        let intor_atm = cint_data.gen_intor(vec![0, 2]);
        let out = unsafe { intor_atm.int_cart([0, 1], Some(int1e_ovlp_cart)) };
        println!("{:?}", out);

        //
        let mut some_bas = BTreeMap::new();
        some_bas.insert(0, vec![0]);
        some_bas.insert(2, vec![0]);
        let intor_bas = cint_data.gen_intor_select(some_bas);
        let out = unsafe { intor_bas.int_cart([0, 1], Some(int1e_ovlp_cart)) };
        println!("{:?}", out);
    }
}
