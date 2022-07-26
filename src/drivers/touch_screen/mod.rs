// SPDX-License-Identifier: GPL-3.0-or-later

#[cfg(feature="saturn")]
mod saturn;
#[cfg(feature="saturn")]
pub use saturn::*;

#[cfg(feature="mono4k")]
mod mono4k;
#[cfg(feature="mono4k")]
pub use mono4k::*;

#[cfg(feature="mono")]
mod mono;
#[cfg(feature="mono")]
pub use mono::*;

// TODO Merge the two implementation in one.
