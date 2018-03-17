extern crate tempdir;

pub mod okof;

#[cfg(test)]
mod tests {
    use okof;
    use tempdir::TempDir;

    #[test]
    fn write_new() {
        let key = "key";
        let value = [0, 1, 2];
        let tmp_dir = TempDir::new("write_new").unwrap();

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
        let mut value = vec![1; 100000];
        value[0] = 7;
        let len = value.len();
        value[len - 1] = 7;
        let tmp_dir = TempDir::new("write_big").unwrap();

        okof::write(&tmp_dir.path(), &key, &value.as_slice()).unwrap();
        assert_eq!(okof::read(&tmp_dir.path(), &key).unwrap(), value);
    }

    #[test]
    fn write_many() {
        let size = 10000;
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

    /*#[test]
    fn write_compressed() {
        let key = "key";
        let mut value = vec![1; 100000];
        value[0] = 7;
        let len = value.len();
        value[len - 1] = 7;
        let tmp_dir = TempDir::new("write_compressed").unwrap();

        okof::write(&tmp_dir.path(), &key, &value.as_slice().unwrap();
        assert_eq!(okof::read(&tmp_dir.path(), &key).unwrap(), value);

        let file = okof::get_raw_file(&tmp_dir.path(), &key);
        assert!(file.size() < (value.len() / 2))
    }*/

    #[test]
    fn delete() {
        let key = "key";
        let value = [0, 1, 2];
        let tmp_dir = TempDir::new("delete").unwrap();

        okof::write(&tmp_dir.path(), &key, &value).unwrap();
        assert_eq!(okof::read(&tmp_dir.path(), &key).unwrap(), &value);

        assert!(okof::delete(&tmp_dir.path(), &key).is_ok());

        match okof::read(&tmp_dir.path(), &key).err().unwrap() {
            okof::Error::NotFound => (),
            _ => { assert!(false); },
        }
    }

    #[test]
    fn read_empty() {
        let key = "key";
        let tmp_dir = TempDir::new("write_append").unwrap();

        // this is retarted
        match okof::read(&tmp_dir.path(), &key).err().unwrap() {
            okof::Error::NotFound => (),
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

        match okof::write(&file_path, &key, &value).err().unwrap() {
            okof::Error::NotDir => (),
            _ => { assert!(false); },
        }

        match okof::read(&file_path, &key).err().unwrap() {
            okof::Error::NotDir => (),
            _ => { assert!(false); },
        }

        match okof::delete(&file_path, &key).err().unwrap() {
            okof::Error::NotDir => (),
            _ => { assert!(false); },
        }

        let mut non_empty_buffer = vec![1, 2, 3];
        let path = tmp_dir.path();
        match okof::read_into(&path, &key, &mut non_empty_buffer).err().unwrap() {
            okof::Error::NotEmpty => (),
            _ => { assert!(false); },
        }
    }
}
