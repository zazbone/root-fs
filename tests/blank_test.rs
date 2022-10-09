extern crate root_fs;

#[cfg(test)]
mod blank_test {
    use std::path::PathBuf;
    use root_fs::core::{Header, Source};
    use std::fs::File;
    #[test]
    fn detect_root_tag() {
        let mut path = PathBuf::from(file!());
        path.pop();
        path.push("datasets");
        path.push("dataset.root");
        let file = File::open(path).unwrap();
        let source = Source::Local(file);
        let header = Header::from_source(source).unwrap();
        assert!(header.had_root_tag());
    }
}