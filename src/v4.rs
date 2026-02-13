use parse_display_derive::{Display, FromStr};

use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display, FromStr, Default, Hash)]
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
        use CFDIUse::*;
        match self {
            G01 => "Adquisición de mercancías",
            G02 => "Devoluciones, descuentos o bonificaciones",
            G03 => "Gastos en general",
            I01 => "Construcciones",
            I02 => "Mobiliario y equipo de oficina por inversiones",
            I03 => "Equipo de transporte",
            I04 => "Equipo de computo y accesorios",
            I05 => "Dados, troqueles, moldes, matrices y herramental",
            I06 => "Comunicaciones telefónicas",
            I07 => "Comunicaciones satelitales",
            I08 => "Otra maquinaria y equipo",
            D01 => "Honorarios médicos, dentales y gastos hospitalarios",
            D02 => "Gastos médicos por incapacidad o discapacidad",
            D03 => "Gastos funerales",
            D04 => "Donativos",
            D05 => "Intereses reales efectivamente pagados por créditos hipotecarios (casa habitación)",
            D06 => "Aportaciones voluntarias al SAR",
            D07 => "Primas por seguros de gastos médicos",
            D08 => "Gastos de transportación escolar obligatoria",
            D09 => "Depósitos en cuentas para el ahorro, primas que tengan como base planes de pensiones",
            D10 => "Pagos por servicios educativos (colegiaturas)",
            S01 => "Sin efectos fiscales",
            CP01 => "Pagos",
            CN01 => "Nómina",
        }
    }

    pub fn from_description(str: &str) -> Self {
        use CFDIUse::*;
        match str {
            "Adquisición de mercancías" => G01,
            "Devoluciones, descuentos o bonificaciones" => G02,
            "Gastos en general" => G03,
            "Construcciones" => I01,
            "Mobiliario y equipo de oficina por inversiones" => I02,
            "Equipo de transporte" => I03,
            "Equipo de computo y accesorios" => I04,
            "Dados, troqueles, moldes, matrices y herramental" => I05,
            "Comunicaciones telefónicas" => I06,
            "Comunicaciones satelitales" => I07,
            "Otra maquinaria y equipo" => I08,
            "Honorarios médicos, dentales y gastos hospitalarios" => D01,
            "Gastos médicos por incapacidad o discapacidad" => D02,
            "Gastos funerales" => D03,
            "Donativos" => D04,
            "Intereses reales efectivamente pagados por créditos hipotecarios (casa habitación)" => D05,
            "Aportaciones voluntarias al SAR" => D06,
            "Primas por seguros de gastos médicos" => D07,
            "Gastos de transportación escolar obligatoria" => D08,
            "Depósitos en cuentas para el ahorro, primas que tengan como base planes de pensiones" => D09,
            "Pagos por servicios educativos (colegiaturas)" => D10,
            "Sin efectos fiscales" => S01,
            "Pagos" => CP01,
            "Nómina" => CN01,
            _ => Self::default(),
        }
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
