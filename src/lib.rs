//! Simple crate for CFDI use in Mexico
//!
//! Biblioteca para el uso de CFDI en México

#![allow(deprecated)]

pub mod v3;
pub mod v4;

#[cfg(feature = "serde")]
pub mod serde_feature;

pub(crate) const CFDI_USE_G01_TEXT: &str = "Adquisición de mercancías";
pub(crate) const CFDI_USE_G02_TEXT: &str = "Devoluciones, descuentos o bonificaciones";
pub(crate) const CFDI_USE_G03_TEXT: &str = "Gastos en general";
pub(crate) const CFDI_USE_I01_TEXT: &str = "Construcciones";
pub(crate) const CFDI_USE_I02_TEXT: &str = "Mobiliario y equipo de oficina por inversiones";
pub(crate) const CFDI_USE_I03_TEXT: &str = "Equipo de transporte";
pub(crate) const CFDI_USE_I04_TEXT: &str = "Equipo de computo y accesorios";
pub(crate) const CFDI_USE_I05_TEXT: &str = "Dados, troqueles, moldes, matrices y herramental";
pub(crate) const CFDI_USE_I06_TEXT: &str = "Comunicaciones telefónicas";
pub(crate) const CFDI_USE_I07_TEXT: &str = "Comunicaciones satelitales";
pub(crate) const CFDI_USE_I08_TEXT: &str = "Otra maquinaria y equipo";
pub(crate) const CFDI_USE_D01_TEXT: &str = "Horarios médicos, dentales y gastos hospitalarios";
pub(crate) const CFDI_USE_D02_TEXT: &str = "Gastos médicos por incapacidad o discapacidad";
pub(crate) const CFDI_USE_D03_TEXT: &str = "Gastos funerales";
pub(crate) const CFDI_USE_D04_TEXT: &str = "Donativos";
pub(crate) const CFDI_USE_D05_TEXT: &str =
    "Intereses reales efectivamente pagados por créditos hipotecarios (casa habitación)";
pub(crate) const CFDI_USE_D06_TEXT: &str = "Aportaciones voluntarias al SAR";
pub(crate) const CFDI_USE_D07_TEXT: &str = "Primas por seguros de gastos médicos";
pub(crate) const CFDI_USE_D08_TEXT: &str = "Gastos de transportación escolar obligatoria";
pub(crate) const CFDI_USE_D09_TEXT: &str =
    "Depósitos en cuentas para el ahorro, primas que tengan como base planes de pensiones";
pub(crate) const CFDI_USE_D10_TEXT: &str = "Pagos por servicios educativos (colegiaturas)";

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn not_standard_cfdi() {
        let cfdi_use = v3::CFDIUse::from("Strange text");
        assert_eq!(cfdi_use, v3::CFDIUse::default());

        let cfdi_use = v4::CFDIUse::from("Strange text");
        assert_eq!(cfdi_use, v4::CFDIUse::default());
    }

    #[test]
    fn default() {
        let cfdi_use = v3::CFDIUse::from("G03");

        assert_eq!(cfdi_use, v3::CFDIUse::default());
        assert_eq!(cfdi_use, v3::CFDIUse::G03);
    }

    #[test]
    fn cfdi_from_description() {
        let cfdi_use = v4::CFDIUse::from_description("Pagos");
        assert_eq!(cfdi_use, v4::CFDIUse::CP01);
        assert_eq!("Pagos", v4::CFDIUse::CP01.description());
    }

    #[test]
    fn deref_works() {
        let cfdi_use = v4::CFDIUse::CP01;

        fn foo(str: &str) {
            println!("{str}")
        }

        foo(&cfdi_use)
    }
}
