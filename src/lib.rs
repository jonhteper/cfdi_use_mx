//! Simple crate for CFDI use in Mexico
//!
//! Crate sencillo para el uso de CFDI en México

#[derive(Debug, Clone, PartialEq, Eq)]
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
    D010,
    P01,
}

impl ToString for CFDIUse {
    fn to_string(&self) -> String {
        match self {
            CFDIUse::G01 => "Adquisición de mercancías".to_string(),
            CFDIUse::G02 => "Devoluciones, descuentos o bonificaciones".to_string(),
            CFDIUse::G03 => "Gastos en general".to_string(),
            CFDIUse::I01 => "Construcciones".to_string(),
            CFDIUse::I02 => "Mobilario y equipo de oficina por inversiones".to_string(),
            CFDIUse::I03 => "Equipo de transporte".to_string(),
            CFDIUse::I04 => "Equipo de computo y accesorios".to_string(),
            CFDIUse::I05 => "Dados, troqueles, moldes, matrices y herramental".to_string(),
            CFDIUse::I06 => "Comunicaciones telefónicas".to_string(),
            CFDIUse::I07 => "Comunicaciones satelitales".to_string(),
            CFDIUse::I08 => "Otra maquinaria y equipo".to_string(),
            CFDIUse::D01 => "Horarios médicos, dentales y gastos hospitalarios".to_string(),
            CFDIUse::D02 => "Gastos médicos por incapacidad o discapacidad".to_string(),
            CFDIUse::D03 => "Gastos funerales".to_string(),
            CFDIUse::D04 => "Donativos".to_string(),
            CFDIUse::D05 => "Intereses reales efectivamente pagados por créditos hipotecarios (casa habitación)".to_string(),
            CFDIUse::D06 => "Aportaciones voluntarias al SAR".to_string(),
            CFDIUse::D07 => "Primas por seguros de gastos médicos".to_string(),
            CFDIUse::D08 => "Gastos de transportación escolar obligatoria".to_string(),
            CFDIUse::D09 => "Depósitos en cuentas para el ahorro, primas que tengan como base planes de pensiones".to_string(),
            CFDIUse::D010 => "Pagos por servicios educativos (colegiaturas)".to_string(),
            CFDIUse::P01 => "Por definir".to_string(),
        }
    }
}

impl From<&str> for CFDIUse {
    fn from(str: &str) -> Self {
        match str {
            "Adquisición de mercancías" => Self::G01,
            "Devoluciones, descuentos o bonificaciones" => Self::G02,
            "Gastos en general" => Self::G03,
            "Construcciones" => Self::I01,
            "Mobilario y equipo de oficina por inversiones" => Self::I02,
            "Equipo de transporte" => Self::I03,
            "Equipo de computo y accesorios" => Self::I04,
            "Dados, troqueles, moldes, matrices y herramental" => Self::I05,
            "Comunicaciones telefónicas" => Self::I06,
            "Comunicaciones satelitales" => Self::I07,
            "Otra maquinaria y equipo" => Self::I08,
            "Horarios médicos, dentales y gastos hospitalarios" => Self::D01,
            "Gastos médicos por incapacidad o discapacidad" => Self::D02,
            "Gastos funerales" => Self::D03,
            "Donativos" => Self::D04,
            "Intereses reales efectivamente pagados por créditos hipotecarios (casa habitación)" => Self::D05,
            "Aportaciones voluntarias al SAR" => Self::D06,
            "Primas por seguros de gastos médicos" => Self::D07,
            "Gastos de transportación escolar obligatoria" => Self::D08,
            "Depósitos en cuentas para el ahorro, primas que tengan como base planes de pensiones" => Self::D09,
            "Pagos por servicios educativos (colegiaturas)" => Self::D010,
            "Por definir" => Self::P01,
            _ => Self::default(),
        }
    }
}

impl Default for CFDIUse {
    fn default() -> Self {
        Self::G03
    }
}

#[cfg(test)]
mod tests {
    use crate::CFDIUse;

    #[test]
    fn not_standard_cfdi() {
        let cfdi_use = CFDIUse::from("Strange text");

        assert_eq!(cfdi_use, CFDIUse::default());
    }

    #[test]
    fn default() {
        let cfdi_use = CFDIUse::from("Gastos en general");

        assert_eq!(&cfdi_use, &CFDIUse::default());
        assert_eq!(&cfdi_use, &CFDIUse::G03);
    }
}
