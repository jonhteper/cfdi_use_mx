use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Serialize,
};

use crate::v4::CFDIUse;

impl Serialize for CFDIUse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

pub struct CfdiUseVisitor;

impl<'de> Visitor<'de> for CfdiUseVisitor {
    type Value = CFDIUse;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("código de uso de CFDI")
    }

    fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(CFDIUse::from(str))
    }

    fn visit_string<E>(self, str: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(&str)
    }
}

impl<'de> Deserialize<'de> for CFDIUse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(CfdiUseVisitor)
    }
}

#[cfg(test)]
mod serde_tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::v4::CFDIUse;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Costumer {
        rfc: String,
        cfdi_use: CFDIUse,
    }

    #[test]
    fn serialize_works() {
        let costumer = Costumer {
            rfc: "RFC11DDDF".to_string(),
            cfdi_use: CFDIUse::D04,
        };

        let result = serde_json::to_string(&costumer);
        assert!(result.is_ok());
        println!("{:?}", result.unwrap());
    }

    #[test]
    fn deserialize_works() {
        let costumer_json = json!({
            "rfc": "RFC11DDDF",
            "cfdi_use": "D03"
        });

        let costumer = Costumer {
            rfc: "RFC11DDDF".to_string(),
            cfdi_use: CFDIUse::D03,
        };

        let deserialize_costumer = serde_json::from_value::<Costumer>(costumer_json).unwrap();
        assert_eq!(&deserialize_costumer, &costumer);
        println!("{:?}", &deserialize_costumer);
    }
    #[test]
    fn deserialize_default_works() {
        let costumer_json = json!({
            "rfc": "RFC11DDDF",
            "cfdi_use": ""
        });

        let costumer = Costumer {
            rfc: "RFC11DDDF".to_string(),
            cfdi_use: CFDIUse::default(),
        };

        let deserialize_costumer = serde_json::from_value::<Costumer>(costumer_json).unwrap();
        assert_eq!(&deserialize_costumer, &costumer);
        println!("{:?}", &deserialize_costumer);
    }
}
