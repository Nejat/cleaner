use std::sync::Once;

use crate::Platform;
use crate::utils::list_output;

pub fn supported_platforms(platforms: &[Platform]) {
    let mut separator = false;
    let skip_first = Once::new();


    for platform in platforms {
        if separator { println!(); }

        skip_first.call_once(|| separator = true);

        println!("Platform: {}", platform.name);
        println!("  Build Artifacts: {}", list_output(&platform.folders));
        println!("  Matched On: {}", list_output(&platform.associated));
    }
}