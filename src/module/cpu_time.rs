use std::{time::Instant, *};

// measure CPU time
pub fn time(start: Instant) {
    let end = start.elapsed();
    println!(
        "All completed in {}.{:03}s.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
