use parse_display_derive::{Display, FromStr};

use crate::{
    CFDI_USE_D01_TEXT, CFDI_USE_D02_TEXT, CFDI_USE_D03_TEXT, CFDI_USE_D04_TEXT, CFDI_USE_D05_TEXT,
    CFDI_USE_D06_TEXT, CFDI_USE_D07_TEXT, CFDI_USE_D08_TEXT, CFDI_USE_D09_TEXT, CFDI_USE_D10_TEXT,
    CFDI_USE_G01_TEXT, CFDI_USE_G02_TEXT, CFDI_USE_G03_TEXT, CFDI_USE_I01_TEXT, CFDI_USE_I02_TEXT,
    CFDI_USE_I03_TEXT, CFDI_USE_I04_TEXT, CFDI_USE_I05_TEXT, CFDI_USE_I06_TEXT, CFDI_USE_I07_TEXT,
    CFDI_USE_I08_TEXT,
};
use std::{ops::Deref, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display, FromStr, Default)]
pub enum CFDIUse {
    G01,
    G02,
    #[default]
    G03,
    I01,
    I02,
    I03,
    I04,
    I05,
    I06,
    I07,
    I08,
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    CP01,
    CN01,
    S01,
}

impl From<&str> for CFDIUse {
    fn from(str: &str) -> Self {
        CFDIUse::from_str(str).unwrap_or_default()
    }
}

impl CFDIUse {
    pub fn description(&self) -> &'static str {
        match self {
            CFDIUse::G01 => CFDI_USE_G01_TEXT,
            CFDIUse::G02 => CFDI_USE_G02_TEXT,
            CFDIUse::G03 => CFDI_USE_G03_TEXT,
            CFDIUse::I01 => CFDI_USE_I01_TEXT,
            CFDIUse::I02 => CFDI_USE_I02_TEXT,
            CFDIUse::I03 => CFDI_USE_I03_TEXT,
            CFDIUse::I04 => CFDI_USE_I04_TEXT,
            CFDIUse::I05 => CFDI_USE_I05_TEXT,
            CFDIUse::I06 => CFDI_USE_I06_TEXT,
            CFDIUse::I07 => CFDI_USE_I07_TEXT,
            CFDIUse::I08 => CFDI_USE_I08_TEXT,
            CFDIUse::D01 => CFDI_USE_D01_TEXT,
            CFDIUse::D02 => CFDI_USE_D02_TEXT,
            CFDIUse::D03 => CFDI_USE_D03_TEXT,
            CFDIUse::D04 => CFDI_USE_D04_TEXT,
            CFDIUse::D05 => CFDI_USE_D05_TEXT,
            CFDIUse::D06 => CFDI_USE_D06_TEXT,
            CFDIUse::D07 => CFDI_USE_D07_TEXT,
            CFDIUse::D08 => CFDI_USE_D08_TEXT,
            CFDIUse::D09 => CFDI_USE_D09_TEXT,
            CFDIUse::D10 => CFDI_USE_D10_TEXT,
            CFDIUse::CP01 => "Pagos",
            CFDIUse::CN01 => "Nómina",
            CFDIUse::S01 => "Sin efectos fiscales",
        }
    }

    #[deprecated(since = "1.1.0", note = "Use CFDIUse::to_string instead")]
    pub fn as_str(&self) -> &'static str {
        match self {
            CFDIUse::G02 => "G02",
            CFDIUse::G01 => "G01",
            CFDIUse::G03 => "G03",
            CFDIUse::I01 => "I01",
            CFDIUse::I02 => "I02",
            CFDIUse::I03 => "I03",
            CFDIUse::I04 => "I04",
            CFDIUse::I05 => "I05",
            CFDIUse::I06 => "I06",
            CFDIUse::I07 => "I07",
            CFDIUse::I08 => "I08",
            CFDIUse::D01 => "D01",
            CFDIUse::D02 => "D02",
            CFDIUse::D03 => "D03",
            CFDIUse::D04 => "D04",
            CFDIUse::D05 => "D05",
            CFDIUse::D06 => "D06",
            CFDIUse::D07 => "D07",
            CFDIUse::D08 => "D08",
            CFDIUse::D09 => "D09",
            CFDIUse::D10 => "D10",
            CFDIUse::CP01 => "CP01",
            CFDIUse::CN01 => "CN01",
            CFDIUse::S01 => "S01",
        }
    }

    pub fn from_description(str: &str) -> Self {
        match str {
            CFDI_USE_G01_TEXT => Self::G01,
            CFDI_USE_G02_TEXT => Self::G02,
            CFDI_USE_G03_TEXT => Self::G03,
            CFDI_USE_I01_TEXT => Self::I01,
            CFDI_USE_I02_TEXT => Self::I02,
            CFDI_USE_I03_TEXT => Self::I03,
            CFDI_USE_I04_TEXT => Self::I04,
            CFDI_USE_I05_TEXT => Self::I05,
            CFDI_USE_I06_TEXT => Self::I06,
            CFDI_USE_I07_TEXT => Self::I07,
            CFDI_USE_I08_TEXT => Self::I08,
            CFDI_USE_D01_TEXT => Self::D01,
            CFDI_USE_D02_TEXT => Self::D02,
            CFDI_USE_D03_TEXT => Self::D03,
            CFDI_USE_D04_TEXT => Self::D04,
            CFDI_USE_D05_TEXT => Self::D05,
            CFDI_USE_D06_TEXT => Self::D06,
            CFDI_USE_D07_TEXT => Self::D07,
            CFDI_USE_D08_TEXT => Self::D08,
            CFDI_USE_D09_TEXT => Self::D09,
            CFDI_USE_D10_TEXT => Self::D10,
            "Pagos" => Self::CP01,
            "Nómina" => Self::CN01,
            "Sin efectos fiscales" => Self::S01,
            _ => Self::default(),
        }
    }
}

impl Deref for CFDIUse {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_works() {
        let from_str = "G03".parse();
        assert_eq!(from_str, Ok(CFDIUse::default()));

        let as_string = CFDIUse::D06.to_string();
        assert_eq!(as_string.as_str(), "D06");
    }
}
