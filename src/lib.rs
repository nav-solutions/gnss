#![doc(
    html_logo_url = "https://raw.githubusercontent.com/nav-solutions/.github/master/logos/logo2.jpg"
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod macros;

// pub modules
pub mod constellation;
pub mod sv;

// private modules
#[cfg(all(feature = "sbas", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "sbas", feature = "std"))))]
mod sbas;

#[cfg(all(feature = "cospar", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "cospar", feature = "std"))))]
pub mod cospar;

#[cfg(feature = "domes")]
#[cfg_attr(docsrs, doc(cfg(feature = "domes")))]
pub mod domes;

#[cfg(feature = "sbas")]
#[cfg_attr(docsrs, doc(cfg(feature = "sbas")))]
pub use sbas::sbas_selector;

#[cfg(feature = "sbas")]
pub use geo::geometry::Point;

// prelude (pkg)
pub mod prelude {
    pub use crate::{constellation::Constellation, sv::SV};

    #[cfg(all(feature = "cospar", feature = "std"))]
    pub use crate::cospar::COSPAR;

    #[cfg(feature = "domes")]
    pub use crate::domes::{TrackingPoint as DOMESTrackingPoint, DOMES};
}
