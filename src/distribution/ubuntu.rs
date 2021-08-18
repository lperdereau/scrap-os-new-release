use super::super::os_scrapper::{OSRelease, OSScrapper};

#[derive(Debug, Clone)]
pub struct OSScarpperUbuntu {
    latest_os_release_known: OSRelease,
}

impl OSScrapper for OSScarpperUbuntu {
    fn get_os_new_release(&mut self) -> Option<OSRelease> {
        Some(self.latest_os_release_known.to_owned())
    }

    fn scrap(&mut self) -> Vec<OSRelease> {
        return vec![];
    }
}

impl OSScarpperUbuntu {
    fn new(latest_os_release_known: OSRelease) -> Self {
        Self { latest_os_release_known, }
    }
}
