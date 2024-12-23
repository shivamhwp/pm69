use std::collections::HashMap;

//srtucts for specfile
pub struct Package {
    owner: String,
    name: String,
    version: String,
    homepage: String,
    repository: String,
    specfile: String,
    authors: String,
    license: Vec<String>,
    description: String,
    keywords: Vec<String>,
    symbols: Vec<String>,
    assets: Assets,
}

// An AssetPath describes a local file path or a remote URL.

struct AssetPath {
    value: String,
    is_remote: bool,
}

struct Assets {
    path: *mut AssetPath,
    pattern: String,
    files: HashMap<String, String>,
    checksums: HashMap<String, String>,
}

// functions to be implemented
// Read(), ReadLocal(), readremote(), dir(), path()
