# rust-ranges

Goal 1: recreate the functionality of IRanges/S4Vectors I need in triform2 to avoid using Rpy2

Goal 2: create Python-bindings

Goal 3: refactor and optimize the rust code

This is my first rust project and the first time I have touched a low-level lang since writing a compiler in college so expect it to take a long while (or never be completed at all :/ )

#### Develop

`cargo watch test` to run tests

#### Play around with it

```
cargo build
python test_ranges.py
```

#### Important differences

rust-ranges does not cycle arguments of different lengths.
