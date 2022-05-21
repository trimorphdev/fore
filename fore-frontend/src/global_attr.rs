use std::collections::HashMap;

use crate::AttributeMap;

/// Gets the global attributes from an attribute map and outputs them to another attribute map.
pub fn global_attr(src: &AttributeMap) -> AttributeMap {
    let mut dest = HashMap::new();

    for (name, value) in src {
        let new_name = name.replace("_", "-");
        dest.insert(new_name, value.clone());
    }

    dest
}