use std::path::Path;
use std::path::PathBuf;
use std::io;

use glob::glob;
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
};

/// Taken from firstyear's code in obs-service-cargo
/// for libroast adoption/migration
pub fn process_globs(src: &Path) -> io::Result<PathBuf> {
    let glob_iter = match glob(&src.as_os_str().to_string_lossy()) {
        Ok(gi) => {
            trace!(?gi);
            gi
        }
        Err(e) => {
            error!(err = ?e, "Invalid glob input");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid glob input",
            ));
        }
    };

    let mut globs = glob_iter
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            error!(?e, "glob error");
            io::Error::new(io::ErrorKind::InvalidInput, "Glob error")
        })?;

    // There can legitimately be multiple matching files. Generally this happens with
    // tar_scm where you have name-v1.tar and the service reruns and creates
    // name-v2.tar. In this case, we would error if we demand a single match, when what
    // we really need is to take the *latest*. Thankfully for us, versions in rpm
    // tar names tend to sort lexicographically, so we can just sort this list and
    // the last element is the newest. (ie v2 sorts after v1).

    globs.sort_unstable();

    if globs.len() > 1 {
        warn!("⚠️  Multiple files matched glob");
        for item in &globs {
            warn!("- {}", item.display());
        }
    }

    // Take the last item.
    globs
        .pop()
        .inspect(|item| info!("✅ Matched an item: {}", item.display()))
        .ok_or_else(|| {
            error!("No files/directories matched src glob input");
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "No files/directories matched src glob input",
            )
        })
}
