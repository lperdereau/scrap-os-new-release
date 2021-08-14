use super::*;

struct OSScrapperUbuntu {
    latest_os_version_known: OsReleaseUbuntu,
}

impl OSScrapper for OSScarpperUbuntu {}

impl OSRelease for OsReleaseUbuntu {
    fn new(version: String, name: String, arch: String, release_date: Utc) -> OsReleaseUbuntu {
        OsReleaseUbuntu {
            version,
            name,
            familly: Familly::Ubuntu,
            arch,
            release_date,
        }
    }
}
