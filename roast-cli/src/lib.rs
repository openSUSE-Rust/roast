pub mod cli;
pub mod raw;
pub mod recomprizz;
pub mod roast;
pub mod roast_scm;

pub use libroast::operations::{
    raw::raw_opts,
    recomprizz::recomprizz_opts,
    roast::roast_opts,
    roast_scm::roast_scm_opts,
};
