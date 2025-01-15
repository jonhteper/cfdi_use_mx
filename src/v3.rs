use crate::{
    CFDI_USE_D01_TEXT, CFDI_USE_D02_TEXT, CFDI_USE_D03_TEXT, CFDI_USE_D04_TEXT, CFDI_USE_D05_TEXT,
    CFDI_USE_D06_TEXT, CFDI_USE_D07_TEXT, CFDI_USE_D08_TEXT, CFDI_USE_D09_TEXT, CFDI_USE_D10_TEXT,
    CFDI_USE_G01_TEXT, CFDI_USE_G02_TEXT, CFDI_USE_G03_TEXT, CFDI_USE_I01_TEXT, CFDI_USE_I02_TEXT,
    CFDI_USE_I03_TEXT, CFDI_USE_I04_TEXT, CFDI_USE_I05_TEXT, CFDI_USE_I06_TEXT, CFDI_USE_I07_TEXT,
    CFDI_USE_I08_TEXT,
};
#[deprecated(since = "1.0.0", note = "please use `v4::CFDIUse` instead")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    P01,
}

impl CFDIUse {
    pub fn description(&self) -> String {
        match self {
            CFDIUse::G01 => CFDI_USE_G01_TEXT.to_string(),
            CFDIUse::G02 => CFDI_USE_G02_TEXT.to_string(),
            CFDIUse::G03 => CFDI_USE_G03_TEXT.to_string(),
            CFDIUse::I01 => CFDI_USE_I01_TEXT.to_string(),
            CFDIUse::I02 => CFDI_USE_I02_TEXT.to_string(),
            CFDIUse::I03 => CFDI_USE_I03_TEXT.to_string(),
            CFDIUse::I04 => CFDI_USE_I04_TEXT.to_string(),
            CFDIUse::I05 => CFDI_USE_I05_TEXT.to_string(),
            CFDIUse::I06 => CFDI_USE_I06_TEXT.to_string(),
            CFDIUse::I07 => CFDI_USE_I07_TEXT.to_string(),
            CFDIUse::I08 => CFDI_USE_I08_TEXT.to_string(),
            CFDIUse::D01 => CFDI_USE_D01_TEXT.to_string(),
            CFDIUse::D02 => CFDI_USE_D02_TEXT.to_string(),
            CFDIUse::D03 => CFDI_USE_D03_TEXT.to_string(),
            CFDIUse::D04 => CFDI_USE_D04_TEXT.to_string(),
            CFDIUse::D05 => CFDI_USE_D05_TEXT.to_string(),
            CFDIUse::D06 => CFDI_USE_D06_TEXT.to_string(),
            CFDIUse::D07 => CFDI_USE_D07_TEXT.to_string(),
            CFDIUse::D08 => CFDI_USE_D08_TEXT.to_string(),
            CFDIUse::D09 => CFDI_USE_D09_TEXT.to_string(),
            CFDIUse::D10 => CFDI_USE_D10_TEXT.to_string(),
            CFDIUse::P01 => "Por definir".to_string(),
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
            "Por definir" => Self::P01,
            _ => Self::default(),
        }
    }
}

impl ToString for CFDIUse {
    fn to_string(&self) -> String {
        match self {
            CFDIUse::G01 => "G01".to_string(),
            CFDIUse::G02 => "G02".to_string(),
            CFDIUse::G03 => "G03".to_string(),
            CFDIUse::I01 => "I01".to_string(),
            CFDIUse::I02 => "I02".to_string(),
            CFDIUse::I03 => "I03".to_string(),
            CFDIUse::I04 => "I04".to_string(),
            CFDIUse::I05 => "I05".to_string(),
            CFDIUse::I06 => "I06".to_string(),
            CFDIUse::I07 => "I07".to_string(),
            CFDIUse::I08 => "I08".to_string(),
            CFDIUse::D01 => "D01".to_string(),
            CFDIUse::D02 => "D02".to_string(),
            CFDIUse::D03 => "D03".to_string(),
            CFDIUse::D04 => "D04".to_string(),
            CFDIUse::D05 => "D05".to_string(),
            CFDIUse::D06 => "D06".to_string(),
            CFDIUse::D07 => "D07".to_string(),
            CFDIUse::D08 => "D08".to_string(),
            CFDIUse::D09 => "D09".to_string(),
            CFDIUse::D10 => "D10".to_string(),
            CFDIUse::P01 => "P01".to_string(),
        }
    }
}

impl From<&str> for CFDIUse {
    fn from(str: &str) -> Self {
        match str {
            "G01" => Self::G01,
            "G02" => Self::G02,
            "G03" => Self::G03,
            "I01" => Self::I01,
            "I02" => Self::I02,
            "I03" => Self::I03,
            "I04" => Self::I04,
            "I05" => Self::I05,
            "I06" => Self::I06,
            "I07" => Self::I07,
            "I08" => Self::I08,
            "D01" => Self::D01,
            "D02" => Self::D02,
            "D03" => Self::D03,
            "D04" => Self::D04,
            "D05" => Self::D05,
            "D06" => Self::D06,
            "D07" => Self::D07,
            "D08" => Self::D08,
            "D09" => Self::D09,
            "D10" => Self::D10,
            "P01" => Self::P01,
            _ => Self::default(),
        }
    }
}

impl Default for CFDIUse {
    fn default() -> Self {
        Self::G03
    }
}
