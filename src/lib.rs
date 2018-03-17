extern crate snap;

pub mod okof;

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use okof;
    use self::tempdir::TempDir;

    #[test]
    fn write_new() {
        let key = "key";
        let value = [0, 1, 2];
        let tmp_dir = TempDir::new("write_new").unwrap();

        okof::write(&tmp_dir.path(), &key, &value).unwrap();
        assert_eq!(okof::read(&tmp_dir.path(), &key).unwrap(), &value);
    }

    #[test]
    fn write_empty() {
        let key = "key";
        let value = [];
        let tmp_dir = TempDir::new("write_empty").unwrap();

        okof::write(&tmp_dir.path(), &key, &value).unwrap();
        assert_eq!(okof::read(&tmp_dir.path(), &key).unwrap(), &value);
    }

    #[test]
    fn write_replace() {
        let key = "key";
        let value_a = [0, 3, 2];
        let value_b = [5, 9, 7];
        let tmp_dir = TempDir::new("write_replace").unwrap();

        okof::write(&tmp_dir.path(), &key, &value_a).unwrap();
        assert_eq!(okof::read(&tmp_dir.path(), &key).unwrap(), &value_a);

        okof::write(&tmp_dir.path(), &key, &value_b).unwrap();
        assert_eq!(okof::read(&tmp_dir.path(), &key).unwrap(), &value_b);
    }

    #[test]
    fn write_big() {
        let key = "key";
        let mut value = vec![1; 100_000];
        value[0] = 7;
        let len = value.len();
        value[len - 1] = 7;
        let tmp_dir = TempDir::new("write_big").unwrap();

        okof::write(&tmp_dir.path(), &key, &value.as_slice()).unwrap();
        assert_eq!(okof::read(&tmp_dir.path(), &key).unwrap(), value);
    }

    #[test]
    fn write_many() {
        let size = 10_000;
        let tmp_dir = TempDir::new("write_many").unwrap();
        let path = tmp_dir.path();

        for i in 0..size {
            let key = i.to_string();
            okof::write(&path, &key, key.as_bytes()).unwrap();
        }
        for i in 0..size {
            let key = i.to_string();
            assert_eq!(okof::read(&path, &key).unwrap(), key.as_bytes());
        }
    }

    #[test]
    fn write_compressed() {
        let key = "key";
        let mut value = vec![1; 100_000];
        value[0] = 7;
        let len = value.len();
        value[len - 1] = 7;
        let tmp_dir = TempDir::new("write_compressed").unwrap();

        okof::write(&tmp_dir.path(), &key, value.as_slice()).unwrap();
        assert_eq!(okof::read(&tmp_dir.path(), &key).unwrap(), value);

        let file = okof::get_raw_file(&tmp_dir.path(), &key).unwrap();
        let expected_max_size = (value.len() / 20) as u64;
        assert!(file.metadata().unwrap().len() <= expected_max_size);

        let small_value = b"Small string";
        okof::write(&tmp_dir.path(), &key, small_value).unwrap();
        let file = okof::get_raw_file(&tmp_dir.path(), &key).unwrap();
        let small_expected_max_size = (small_value.len() as u64) + 1;
        assert!(file.metadata().unwrap().len() <= small_expected_max_size);
    }

    #[test]
    fn delete() {
        let key = "key";
        let value = [0, 1, 2];
        let tmp_dir = TempDir::new("delete").unwrap();

        okof::write(&tmp_dir.path(), &key, &value).unwrap();
        assert_eq!(okof::read(&tmp_dir.path(), &key).unwrap(), &value);

        assert!(okof::delete(&tmp_dir.path(), &key).is_ok());

        match okof::read(&tmp_dir.path(), &key) {
            Err(okof::Error::NotFound) => (),
            _ => { assert!(false); },
        }
    }

    #[test]
    fn read_empty() {
        let key = "key";
        let tmp_dir = TempDir::new("write_append").unwrap();

        // this is retarted
        match okof::read(&tmp_dir.path(), &key) {
            Err(okof::Error::NotFound) => (),
            _ => { assert!(false); },
        }
    }

    //#[test]
    //fn read_concurrent() {
    //    assert_eq!(2 + 2, 4);
    //}

    #[test]
    fn invalid_use() {
        let key = "key";
        let value = [3, 3, 2, 5, 8];
        let tmp_dir = TempDir::new("invalid_use").unwrap();
        let file_path = tmp_dir.path().join("test_file");

        match okof::write(&file_path, &key, &value) {
            Err(okof::Error::NotDir) => (),
            _ => { assert!(false); },
        }

        match okof::read(&file_path, &key) {
            Err(okof::Error::NotDir) => (),
            _ => { assert!(false); },
        }

        match okof::delete(&file_path, &key) {
            Err(okof::Error::NotDir) => (),
            _ => { assert!(false); },
        }

        let mut non_empty_buffer = vec![1, 2, 3];
        let path = tmp_dir.path();
        match okof::read_into(&path, &key, &mut non_empty_buffer) {
            Err(okof::Error::NotEmpty) => (),
            _ => { assert!(false); },
        }
    }
}
