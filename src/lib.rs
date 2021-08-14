use chrono::prelude::*;

trait OSScrapper {
    fn new(latest_os_version_known: OSRelease) -> Self;

    fn get_os_new_release(&mut self) -> Option<OSRelease>;

    fn scrap(&mut self) -> ();
}

enum Familly {
    Ubuntu,
    Debian,
}

struct OSRelease {
    version: String,
    name: String,
    family: Familly,
    arch: String,
    release_date: Utc,
}

impl OSRelease {
    fn new(
        version: String,
        name: String,
        family: Familly,
        arch: String,
        release_date: Utc,
    ) -> OSRelease {
        OSRelease {
            version,
            name,
            family,
            arch,
            release_date,
        }
    }

    fn older(newer_os_release: OSRelease) -> bool {
        unimplemented!("Should return true if os_release in arg is newer than self")
    }
}
