use std::fmt::{Display, Formatter};
use std::ops::Deref;
use crate::{
    CFDI_USE_G01_TEXT,
    CFDI_USE_G02_TEXT,
    CFDI_USE_G03_TEXT,
    CFDI_USE_I01_TEXT,
    CFDI_USE_I02_TEXT,
    CFDI_USE_I03_TEXT,
    CFDI_USE_I04_TEXT,
    CFDI_USE_I05_TEXT,
    CFDI_USE_I06_TEXT,
    CFDI_USE_I07_TEXT,
    CFDI_USE_I08_TEXT,
    CFDI_USE_D01_TEXT,
    CFDI_USE_D02_TEXT,
    CFDI_USE_D03_TEXT,
    CFDI_USE_D04_TEXT,
    CFDI_USE_D05_TEXT,
    CFDI_USE_D06_TEXT,
    CFDI_USE_D07_TEXT,
    CFDI_USE_D08_TEXT,
    CFDI_USE_D09_TEXT,
    CFDI_USE_D10_TEXT,
};

#[derive(Debug,Copy,  Clone, PartialEq, Eq)]
pub enum CFDIUse {
    G01,
    G02,
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

impl Display for CFDIUse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CFDIUse::G01 => write!(f, "{}", CFDI_USE_G01_TEXT),
            CFDIUse::G02 => write!(f, "{}", CFDI_USE_G02_TEXT),
            CFDIUse::G03 => write!(f, "{}", CFDI_USE_G03_TEXT),
            CFDIUse::I01 => write!(f, "{}", CFDI_USE_I01_TEXT),
            CFDIUse::I02 => write!(f, "{}", CFDI_USE_I02_TEXT),
            CFDIUse::I03 => write!(f, "{}", CFDI_USE_I03_TEXT),
            CFDIUse::I04 => write!(f, "{}", CFDI_USE_I04_TEXT),
            CFDIUse::I05 => write!(f, "{}", CFDI_USE_I05_TEXT),
            CFDIUse::I06 => write!(f, "{}", CFDI_USE_I06_TEXT),
            CFDIUse::I07 => write!(f, "{}", CFDI_USE_I07_TEXT),
            CFDIUse::I08 => write!(f, "{}", CFDI_USE_I08_TEXT),
            CFDIUse::D01 => write!(f, "{}", CFDI_USE_D01_TEXT),
            CFDIUse::D02 => write!(f, "{}", CFDI_USE_D02_TEXT),
            CFDIUse::D03 => write!(f, "{}", CFDI_USE_D03_TEXT),
            CFDIUse::D04 => write!(f, "{}", CFDI_USE_D04_TEXT),
            CFDIUse::D05 => write!(f, "{}", CFDI_USE_D05_TEXT),
            CFDIUse::D06 => write!(f, "{}", CFDI_USE_D06_TEXT),
            CFDIUse::D07 => write!(f, "{}", CFDI_USE_D07_TEXT),
            CFDIUse::D08 => write!(f, "{}", CFDI_USE_D08_TEXT),
            CFDIUse::D09 => write!(f, "{}", CFDI_USE_D09_TEXT),
            CFDIUse::D10 => write!(f, "{}", CFDI_USE_D10_TEXT),
            CFDIUse::CP01 => write!(f, "Pagos"),
            CFDIUse::CN01 => write!(f, "Nómina"),
            CFDIUse::S01 => write!(f, "Sin efectos fiscales"),
        }
    }
}

impl From<&str> for CFDIUse {
    fn from(str: &str) -> Self {
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

impl CFDIUse {
    pub fn as_str(&self) -> &'static str {
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
}

impl Default for CFDIUse {
    fn default() -> Self {
        Self::G03
    }
}

impl Deref for CFDIUse {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}