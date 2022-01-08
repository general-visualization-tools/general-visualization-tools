use std::collections::HashSet;
use once_cell::sync::Lazy;

pub static PARTS_NAMES: Lazy<HashSet<&str>> = Lazy::new(||
    ["camera", "clear", "new", "update", "chart", "circle", "rect", "path"]
        .iter().fold(HashSet::new(), |mut s, &name| { s.insert(name); s })
);
