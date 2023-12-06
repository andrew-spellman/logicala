use std::io::Read;
use vfs::{MemoryFS, VfsPath};

pub fn file_from_str(content: &str) -> Box<dyn Read> {
    let root: VfsPath = VfsPath::new(MemoryFS::new());
    let path = root.join("test.txt").unwrap();

    path.create_file()
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();

    Box::new(path.open_file().unwrap())
}
