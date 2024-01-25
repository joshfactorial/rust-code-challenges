use std::{fs, path};

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn exists(&self) -> bool {
        self.exists()
    }

    fn is_writeable(&self) -> bool {
        fs::metadata(self)
            .map(|x| {
                !x.permissions().readonly()
            })
            .unwrap_or(false)
    }

    fn is_readable(&self) -> bool {
        fs::File::open(self).is_ok()
    }
}

fn main() {
    // 
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    let _ = fs::remove_file(f.path());
}
