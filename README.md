# cfdi_use_mx
[![Crates.io](https://shields.io/crates/v/cfdi_use_mx.svg)](https://crates.io/crates/cfdi_use_mx)

Bliblioteca para el uso de CFDI en México, según las especificaciones 3[^*]  y 4. 

[^*]: Actualmente deprecada
## Ejemplo de uso

Conversión por código

```rust
use cfdi_use_mx::*;

fn main() {
    let cfdi_use = v4::CFDIUse::from("G03");
    assert_eq!(cfdi_use, v4::CFDIUse::G03);
}
```

Conversión por descripción

```rust
use cfdi_use_mx::*;

fn main() {
   let cfdi_use = v4::CFDIUse::from_description("Pagos");
    assert_eq!(cfdi_use, v4::CFDIUse::CP01);
    assert_eq!("Pagos", v4::CFDIUse::CP01.description());
}
```

Valor por default

```rust
use cfdi_use_mx::*;

fn main() {
    let cfdi_use = v4::CFDIUse::from("Strange text");
    assert_eq!(cfdi_use, v4::CFDIUse::default());
}
```



## Serde

*IMPORTANTE: El soporte de `serde` está activo por default. Si no lo necesita, desactive las [`default-features`](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features)*

### Ejemplo

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;

use cfdi_use_mx::v4::CFDIUse;

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
```

