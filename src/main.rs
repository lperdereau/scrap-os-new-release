mod os_scrapper;
mod distribution;

use self::os_scrapper::{OSFamily, OSRelease};
use self::distribution::ubuntu::OSScarpperUbuntu;

use chrono::prelude::*;

fn main() {
    let os_release = OSRelease::new(
        String::from("1.1.1"),
        String::from("Test"),
        OSFamily::Ubuntu,
        String::from("Test"),
        Utc::now(),
    );

    println!("{:?}", os_release);

    let os_new_release = OSScarpperUbuntu{ latest_os_release_known: os_release };
    os_new_release.get_os_new_release();
    println!("{:?}", os_new_release);
}
