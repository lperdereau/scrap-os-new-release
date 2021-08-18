use chrono::prelude::*;

pub trait OSScrapper {
    fn get_os_new_release(&mut self) -> Option<OSRelease>;
    fn scrap(&mut self) -> Vec<OSRelease>;
}

#[derive(Debug, Clone)]
pub enum OSFamily {
    Ubuntu,
    Debian,
}

#[derive(Debug, Clone)]
pub struct OSRelease {
    version: String,
    name: String,
    family: OSFamily,
    arch: String,
    release_date: DateTime<Utc>,
}

impl OSRelease {
    pub fn new(
        version: String,
        name: String,
        family: OSFamily,
        arch: String,
        release_date: DateTime<Utc>,
    ) -> OSRelease {
        OSRelease {
            version,
            name,
            family,
            arch,
            release_date,
        }
    }
    #[allow(dead_code)]
    fn older(newer_os_release: OSRelease) -> bool {
        unimplemented!("Should return true if os_release in arg is newer than self")
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {} {} {}", &self.version, &self.name, &self.family, &self.arch, &self.release_date)
    }
}
