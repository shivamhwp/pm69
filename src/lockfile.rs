use crate::spec::{self, Package};
use std::collections::HashMap;

struct Lockfile {
    packages: HashMap<String, Box<spec::Package>>,
}

// functions to be implemented readlocal(), path()
