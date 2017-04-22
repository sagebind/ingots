use ingots::dynamic::DynamicIngot;
use std::path::PathBuf;


pub struct IngotEngine {
    containers: Vec<IngotContainer>,
}

struct IngotContainer {
    prefix: String,
    instance: DynamicIngot,
}

impl IngotEngine {
    pub fn new() -> Self {
        Self { containers: Vec::new() }
    }

    pub fn register<S, P>(&mut self, prefix: S, path: P)
        where S: Into<String>,
              P: Into<PathBuf>
    {
        let prefix = prefix.into();
        let path = path.into();

        info!("Loading ingot {:?} under prefix {}", path, prefix);

        let container = IngotContainer {
            prefix: prefix,
            instance: DynamicIngot::open(path).unwrap(),
        };

        self.containers.push(container);
    }

    pub fn find_ingot_for_url(&self, url: &str) -> Option<&DynamicIngot> {
        for container in self.containers.iter() {
            if url.starts_with(&container.prefix) {
                return Some(&container.instance);
            }
        }

        None
    }
}
