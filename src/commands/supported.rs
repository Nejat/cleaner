use std::sync::Once;

use crate::Platform;
use crate::utils::SEPARATOR;

pub fn supported_platforms(platforms: &[Platform]) {
    let mut separator = false;
    let skip_first = Once::new();


    for platform in platforms {
        if separator { println!(); }

        skip_first.call_once(|| separator = true);

        println!("Platform: {}", platform.name);
        println!(
            "  Build Artifacts: {}",
            platform.folders.iter().map(ToString::to_string).collect::<Vec<_>>().join(SEPARATOR),
        );
        println!(
            "  Matched On: {}",
            platform.associated.iter().map(ToString::to_string).collect::<Vec<_>>().join(SEPARATOR),
        );
    }
}