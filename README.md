# GNSS

[![Rust](https://github.com/nav-solutions/gnss/actions/workflows/rust.yml/badge.svg)](https://github.com/nav-solutions/gnss/actions/workflows/rust.yml)
[![Rust](https://github.com/nav-solutions/gnss/actions/workflows/daily.yml/badge.svg)](https://github.com/nav-solutions/gnss/actions/workflows/daily.yml)
[![crates.io](https://img.shields.io/crates/v/gnss-rs.svg)](https://crates.io/crates/gnss-rs)
[![crates.io](https://docs.rs/gnss-rs/badge.svg)](https://docs.rs/gnss-rs)

[![License](https://img.shields.io/badge/license-MPL_2.0-orange?style=for-the-badge&logo=mozilla)](https://github.com/nav-solutions/qc-traits/blob/main/LICENSE)

High level definitions to work with GNSS in Rust

+ Space Vehicles definitions: `SV`
+ GNSS Constellations: `Constellation`
+ GNSS Timescales: `Constellation.timescale()`

## Getting started

Add "gnss" to your Cargo.toml

```toml
gnss-rs = "2"
```

## Constellation database

This library defines both constellation and satellites from these constellations, in a single enum.

```rust
use std::str::FromStr;
use gnss_rs::prelude::*;
use hifitime::{TimeScale, Epoch};

// It is possible to define satellites that do not exist
let sv = SV::new(Constellation::GPS, 1);

assert_eq!(sv.prn, 1);
assert_eq!(sv.constellation, Constellation::GPS);
assert_eq!(sv.timescale(), Some(TimeScale::GPST)); // convenient method
assert_eq!(sv.constellation.country_code(), Some("US".to_string())); // convenient method

assert_eq!(Constellation::GPS.to_string(), "GPS (US)"); // readable format
assert_eq!(format!("{:E}", Constellation::GPS), "GPS"); // standard accronym
assert_eq!(format!("{:x}", Constellation::GPS), "G"); // RINEX like format

assert_eq!(Constellation::from_str("G"), Ok(Constellation::GPS)); // reciprocal
assert_eq!(Constellation::from_str("GPS (US)"), Ok(Constellation::GPS)); // reciprocal

// this type of information is only defined
// for SBAS vehicles
assert_eq!(sv.launch_datetime(), None); 

```

## SBAS (Geostationary services)

We offer convenient methods to handle SBAS (geostationary services).
For example, we integrate a builtin database to define most known SBAS services.
    
```rust
use std::str::FromStr;
use gnss_rs::prelude::*;
use hifitime::{TimeScale, Epoch, MonthName};

let egnos = Constellation::EGNOS;

assert!(egnos.is_sbas(), "obvious");
assert_eq!(egnos.timescale(), Some(TimeScale::GPST)); // we refer GEO services to GPST

// convenient builder for RINEX and other similar applications.
// Must be known to our database for this to work.
// The database is defined in data/sbas.json
let geo23 = SV::new_sbas(23).unwrap();
assert_eq!(geo23.constellation, Constellation::EGNOS);

// convenient information using our builtin database
assert_eq!(geo23.launch_datetime().and_then(|e| Some(e.to_string())), Some("2021-11-01T00:00:00 UTC".to_string()));
```

All this information is provided by default. If you compiled the library with the "sbas" option,
we provide a selection helper that helps select the GEO service, for given user coordinates.

```rust
use geo::Point;

use gnss_rs::{
    sbas_selector, 
    prelude::Constellation,
};

let paris = Point::new(2.38268, 48.808378); //x=longitude°, y=latitude°
assert_eq!(sbas_selector(paris), Some(Constellation::EGNOS));
```

## COSPAR definition

When compiled with the "COSPAR" option, the library defines the `COSPAR` 
launch identifier (unique number).

```rust
use gnss_rs::prelude::COSPAR;
use std::str::FromStr;

assert!(COSPAR::from_str("2018-080A").is_ok());
```

## DOMES definition

When compiled with the "DOMES" option, the library provides the definition
of [DOMES (IGS) site identification number](https://itrf.ign.fr/en/network/domes/description).


- The SERDE features unlocks serialization/deserialization of the main structures defined here.

- The DOMES features unlocks the definition of DOMES GNSS/IGS reference station,
that are widely used in GNSS data processing. This number identifies a station uniquely.

- The COSPAR features unlocks the definition of the COSPAR (Launch) ID number.
This number identifies the launch of a vehicule uniquely. It is used in RINEX
and other files format.

- The SBAS feature will create a static database that defines each SBAS service areas,
projected on ground as WKT/GEO objects, with one method to select a SBAS service based
on Latitude and Longitude coordinates.

## Relevant Ecosystems

Many libraries exist nowadays to process GNSS data or perform typical GNSS processing tasks.  
Amongst them, be sure to checkout:

- [Nyx](https://github.com/nyx-space/nyx): Orbital navigation
- [ANISE](https://github.com/nyx-space/anise): Earth orientation modeling and Orbital navigation
- [GNSS-RTK](https://github.com/nav-solutions/gnss-rtk): Precise Point Positioning, related calculations and modeling
- [RINEX](https://github.com/nav-solutions/rinex): files processing and management
- [SP3](https://github.com/nav-solutions/sp3): files processing and management
- [Hifitime](https://github.com/nyx-space/hifitime): Timescale and related calculations
- [CGGTTS](https://github.com/nav-solutions/cggtts): files production and processing

## License

This library is part of the [NAV-Solutions framework](https://github.com/nav-solutions) which
is licensed under [Mozilla V2 Public](https://www.mozilla.org/en-US/MPL/2.0) license.
