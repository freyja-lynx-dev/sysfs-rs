//! <https://www.kernel.org/doc/html/latest/gpu/drm-kms-helpers.html#edid-helper-functions-reference>

use crate::lib::sysfs_attrs;
use std::{fs::DirEntry, path::PathBuf};

pub fn get_all_connectors() -> crate::Result<Vec<PathBuf>> {
    /* we find valid connectors by searching for devices with edid data
     * and then checking if the connector is valid
     * there's likely a cleaner way to do this
     */
    let is_card_obj = |inode: &DirEntry| {
        let name = inode.file_name();
        let name = name.to_string_lossy();
        name.starts_with("card") && name["card".len()..].chars().all(|ch| ch.is_ascii_digit())
    };
    let cards: Vec<DirEntry> = std::fs::read_dir("/sys/class/drm/")?
        .filter_map(|res| res.ok())
        .filter(|inode| is_card_obj(inode))
        .collect();

    let is_connector_obj = |inode: &DirEntry| {
        let name = inode.file_name();
        let name = name.to_string_lossy();
        // this works but it feels bad
        name.starts_with("card")
    };
    let connectors: Vec<PathBuf> = cards
        .iter()
        .flat_map(|inode| std::fs::read_dir(inode.path()).unwrap())
        .filter_map(|res| res.ok())
        .filter(|inode| is_connector_obj(inode))
        .map(|inode| inode.path())
        .collect();
    Ok(connectors)
}

pub fn get_active_connectors() -> crate::Result<Vec<PathBuf>> {
    let connectors = get_all_connectors()?;

    let active_connectors = connectors
        .into_iter()
        .filter_map(|connector| {
            let path_str = connector.to_str()?;
            match drm::status(path_str) {
                Ok(drm::Status::Connected) => Some(connector),
                _ => None,
            }
        })
        .collect();
    Ok(active_connectors)
}

#[sysfs_attrs(in "{card}")]
pub mod drm {
    use crate::lib::sysfs;
    use strum::{EnumString, IntoStaticStr};

    #[derive(Clone, Copy, Debug, IntoStaticStr, EnumString)]
    pub enum Status {
        #[strum(serialize = "disconnected")]
        Disconnected,
        #[strum(serialize = "connected")]
        Connected,
        #[strum(serialize = "unknown")]
        Unknown,
    }

    #[sysfs]
    pub fn status(card: &str) -> Status {
        let read = |text: &str| text.parse().unwrap();
        ..
    }
    #[sysfs]
    pub fn connector_id(card: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }
    #[sysfs]
    pub fn uevent(card: &str) -> String {
        let read = str::to_owned;
        ..
    }
    #[derive(Clone, Copy, Debug, IntoStaticStr, EnumString)]
    pub enum Enabled {
        #[strum(serialize = "disabled")]
        Disabled,
        #[strum(serialize = "enabled")]
        Enabled,
    }
    #[sysfs]
    pub fn enabled(card: &str) -> Enabled {
        let read = |text: &str| text.parse().unwrap();
        ..
    }
    #[sysfs]
    pub fn modes(card: &str) -> Vec<String> {
        let read = |text: &str| text.split_whitespace().map(|s| s.to_owned()).collect();
        ..
    }
    #[sysfs]
    pub fn edid(card: &str) -> Vec<u8> {
        let read = |text: &str| text.as_bytes().to_owned();
        ..
    }
}

// pub fn get_edid(mut connector: PathBuf) -> crate::Result<String> {
//     use std::fs::File;
//     use std::io::Read;
//     connector.push("edid");
//     let mut file = File::open(connector)?;
//     let mut buf = String::new();
//     file.read_to_string(&mut buf)?;
//     Ok(buf)
// }

// pub fn get_edid_hex(connector: usize) -> crate::Result<String> {
//     use std::fs::File;
//     let path = format!("/sys/class/drm/card{}/edid_hex", connector);
//     let mut file = File
//     let connectors = std::fs::read_dir("/sys/class/drm/")
// }
