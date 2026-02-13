//! Simple crate for CFDI use in Mexico
//!
//! Biblioteca para el uso de CFDI en México

pub mod v4;

#[cfg(feature = "serde")]
pub mod serde_feature;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn not_standard_cfdi() {
        let cfdi_use = CFDIUse::from("Strange text");
        assert_eq!(cfdi_use, CFDIUse::default());
    }

    #[test]
    fn cfdi_from_description() {
        let cfdi_use = CFDIUse::from_description("Pagos");
        assert_eq!(cfdi_use, CFDIUse::CP01);
        assert_eq!("Pagos", CFDIUse::CP01.description());
    }
}
