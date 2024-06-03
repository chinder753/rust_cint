use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::cint::rawdata::CGTO;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ElectronShells {
    function_type: String,
    region: String,
    angular_momentum: Vec<u8>,
    exponents: Vec<String>,
    coefficients: Vec<Vec<String>>,
}

impl ElectronShells {
    pub(crate) fn function_type(&self) -> &String {
        &self.function_type
    }

    pub(crate) fn region(&self) -> &String {
        &self.region
    }

    pub(crate) fn angular_momentum(&self) -> &Vec<u8> {
        &self.angular_momentum
    }

    pub(crate) fn exponents(&self) -> &Vec<String> {
        &self.exponents
    }

    pub(crate) fn coefficients(&self) -> &Vec<Vec<String>> {
        &self.coefficients
    }

    pub(crate) fn to_cgto(&self, kappa_of: i8, norm: bool) -> CGTO {
        let cgto = CGTO {
            kappa_of,
            angl: self.angular_momentum.clone(),
            exp: self.exponents.iter().map(|x| x.parse().unwrap()).collect(),
            coeff: self
                .coefficients
                .iter()
                .map(|y| y.iter().map(|x| x.parse().unwrap()).collect())
                .collect(),
        };
        if norm {
            cgto.norm()
        } else {
            cgto
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Elements {
    references: Vec<String>,
    electron_shells: Vec<ElectronShells>,
}

impl Elements {
    pub(crate) fn references(&self) -> &Vec<String> {
        &self.references
    }

    pub(crate) fn electron_shells(&self) -> &Vec<ElectronShells> {
        &self.electron_shells
    }

    pub(crate) fn to_cgto(&self, kappa_of: i8, norm: bool) -> Vec<CGTO> {
        self.electron_shells
            .iter()
            .map(|eshl| eshl.to_cgto(kappa_of, norm))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MolssiBseSchema {
    schema_type: String,
    schema_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JsonBasis {
    molssi_bse_schema: MolssiBseSchema,
    description: String,
    data_source: String,
    elements: HashMap<u8, Elements>,
}

impl JsonBasis {
    pub(crate) fn from_string(json_string: &str) -> Self {
        match serde_json::from_str(&json_string) {
            Ok(js) => js,
            Err(why) => panic!(""),
        }
    }

    pub(crate) fn schema_type(&self) -> &String {
        &self.molssi_bse_schema.schema_type
    }

    pub(crate) fn schema_version(&self) -> &String {
        &self.molssi_bse_schema.schema_version
    }

    pub(crate) fn description(&self) -> &String {
        &self.description
    }

    pub(crate) fn data_source(&self) -> &String {
        &self.data_source
    }

    pub(crate) fn get_elements(&self, nuc: u8) -> Option<&Elements> {
        self.elements.get(&nuc)
    }
}
