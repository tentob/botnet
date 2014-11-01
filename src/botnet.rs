use std::os;
use std::str;
use std::collections::TreeMap;
use std::dynamic_lib::DynamicLibrary;
    
pub struct PluginLoader {
    plugins: TreeMap<String, Box<DynamicLibrary>>
}

impl PluginLoader {
    pub fn new() -> PluginLoader {
        PluginLoader { plugins: TreeMap::new() }
    }

    pub fn load(mut self, path: String) {
        let path = Path::new(path);
        let path = os::make_absolute(&path);
        let filename: String = path.filestem_str().expect("Couldn't get filename").to_string();

        match DynamicLibrary::open(Some(&path)) {
            Ok(lib) => self.plugins.insert(filename, box lib),
            Err(error) => panic!("Couldnt load library: {}", error)
        };
    }
}
