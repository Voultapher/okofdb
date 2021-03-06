# One Key One File Database

`okofdb` is a [key value database](https://en.wikipedia.org/wiki/Key-value_database).

### Design Goals:

* Very efficient data storage
* Small memory footprint
* Minimal interface
* ACD | Atomicity Consistency Durability
* Isolation only for multiple readers, not multiple writers
* No application state

If you need multiple concurrent writers,
synchronization is your applications responsibility.
Depending on your use case, possibly a very stupid way of storing your data.

Above an implementation defined value size threshold it will use
[snap](https://crates.io/crates/snap) to compress the value.

## Example

```rust
use std::path::Path;
use okofdb::okof;

fn main() -> Result<(), okof::Error> {
    let path = Path::new("db-dir");
    let key = "plants";
    let value = b"Cucumber:Fluxroot";

    okof::write(&path, &key, value)?;
    assert_eq!(okof::read(&path, &key)?, value);

    let other_value = String::from("Ironwood");
    okof::write(&path, &key, other_value.as_bytes())?;
    assert_eq!(okof::read(&path, &key)?, other_value.as_bytes());

    okof::delete(&path, &key)?;
    match okof::read(&path, &key) {
        Err(okof::Error::NotFound) => Ok(()),
        _ => { panic!(); },
    }
}
```

## Getting Started

Add [this crate](https://crates.io/crates/okofdb) to your project.

### Prerequisites

Rust toolchain and cargo.

Note, some filesystems might struggle with a lot of files in a single directory,
which will happen if there are many keys.

One key == One File.

### Installing

[See cargo docs](https://doc.rust-lang.org/cargo/guide/).

## Running the tests

```
cargo test
```

## Deployment

TBD

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md)
for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available,
see the [tags on this repository](https://github.com/Voultapher/okofdb/tags).

## Authors

* **Lukas Bergdoll** - *Initial work* - [Voultapher](https://github.com/Voultapher)

See also the list of [contributors](https://github.com/Voultapher/okofdb/contributors)
who participated in this project.

## License

This project is licensed under the Apache License, Version 2.0 -
see the [LICENSE.md](LICENSE.md) file for details.
