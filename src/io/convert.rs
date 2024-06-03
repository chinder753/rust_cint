pub(crate) const AUTOAA: f64 = 0.52917726;
pub(crate) const AATOAU: f64 = 1.0 / AUTOAA;

pub(crate) fn autoaa(x: f64) -> f64 {
    x * (0.52917726)
}

pub(crate) fn aatoau(x: f64) -> f64 {
    x * (1.0 / 0.52917726)
}

pub(crate) const AMUTOKG: f64 = 1.660539040e-27;
pub(crate) const KGTOAMU: f64 = 1.0 / AMUTOKG;
pub(crate) const METOKG: f64 = 9.10938356e-31;
pub(crate) const KGTOME: f64 = 1.0 / METOKG;
pub(crate) const AMUTOAU: f64 = AMUTOKG * KGTOME;
