# GNSS

[![Rust](https://github.com/nav-solutions/gnss/actions/workflows/rust.yml/badge.svg)](https://github.com/nav-solutions/gnss/actions/workflows/rust.yml)
[![Rust](https://github.com/nav-solutions/gnss/actions/workflows/daily.yml/badge.svg)](https://github.com/nav-solutions/gnss/actions/workflows/daily.yml)
[![crates.io](https://img.shields.io/crates/v/gnss-rs.svg)](https://crates.io/crates/gnss-rs)
[![crates.io](https://docs.rs/gnss-rs/badge.svg)](https://docs.rs/gnss-rs)

[![License](https://img.shields.io/badge/license-MPL_2.0-orange?style=for-the-badge&logo=mozilla)](https://github.com/nav-solutions/qc-traits/blob/main/LICENSE)

High level definitions to work with GNSS in Rust or Python

+ Space Vehicles definitions: `SV`
+ GNSS Constellations: `Constellation`
+ GNSS Timescales: `Constellation.timescale()`
+ Python casts using `python` feature

## Getting started

Add "gnss" to your Cargo.toml

```toml
gnss-rs = "2"
```

Import "gnss-rs": 

```rust
extern crate gnss_rs as gnss;
```

## Space Vehicles

```rust
extern crate gnss_rs as gnss;

use gnss::sv;
use gnss::prelude::*;
use std::str::FromStr;
use hifitime::TimeScale;

let sv = SV::new(Constellation::GPS, 1);
assert_eq!(sv.constellation, Constellation::GPS);
assert_eq!(sv.prn, 1);
assert_eq!(sv.timescale(), Some(TimeScale::GPST));
assert_eq!(sv, sv!("G01"));

// undefined launch date
assert_eq!(sv.launch_date(), None);
```

## SBAS support

We support SBAS (geostationary augmentations) systems. 

```rust
extern crate gnss_rs as gnss;

use gnss::sv;
use gnss::prelude::*;
use std::str::FromStr;
use hifitime::{Epoch, TimeScale};

let sv = sv!("S23");
assert_eq!(sv.constellation, Constellation::EGNOS);

let launch_date = Epoch::from_str("2021-11-01T00:00:00 UTC")
    .unwrap();

assert_eq!(sv.launch_date(), Some(launch_date));
```

## Other definitions and features

Other definitions and features exist. Use compilation options (crate features) to unlock them.
The idea is to maintain a very minimal default library.

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
is delivered under the [Mozilla V2 Public](https://www.mozilla.org/en-US/MPL/2.0) license.
