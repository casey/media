use super::*;

#[derive(Default)]
pub struct Library {
  packages: HashMap<Hash, Package>,
  handlers: HashMap<Target, Hash>,
}

impl Library {
  pub fn add(&mut self, package: Package) {
    if let Manifest::App { target, .. } = &package.manifest {
      self.handlers.insert(*target, package.hash);
    }

    self.packages.insert(package.hash, package);
  }

  pub fn package(&self, hash: Hash) -> Option<&Package> {
    self.packages.get(&hash)
  }

  pub fn handler(&self, target: Target) -> Option<&Package> {
    self
      .handlers
      .get(&target)
      .map(|hash| self.packages.get(hash).unwrap())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add() {
    let mut library = Library::default();

    let package = Package::load(&packages().join("app.package")).unwrap();

    library.add(package.clone());

    assert_eq!(library.package(package.hash), Some(&package));
    assert_eq!(library.handler(Target::Comic), Some(&package));
  }
}