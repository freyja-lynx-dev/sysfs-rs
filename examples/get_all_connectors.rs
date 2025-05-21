#[path = "shared/common.rs"]
mod common;

use common::print_object;
use sysfs::api::drm::get_all_connectors;

fn main() {
    for connector in get_all_connectors().unwrap() {
        print_object!(
            in sysfs::api::drm::drm
                ["{}", connector.to_str().unwrap()] {
                    status,
                    connector_id,
                    enabled,
                    modes,
                    uevent
                }
        )
    }
}
