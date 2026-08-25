# crabtree 

A reimplementation of the Linux `tree` command, in Rust. First full undirected Rust project. 

- - - 

#### Basic usage

```bash
# -i for ignoring unwanted directories
~/crabtree cargo run -q -- -i target
crabtree
├── README.md
├── Cargo.toml
├── LICENSE
├── target
├── .gitignore
├── tests
│   └── fixtures
│       └── tree
│           ├── hello.rs
│           ├── .im_hiding
│           └── subdir
│               └── hello2.rs
├── Cargo.lock
├── src
│   ├── viz.rs
│   ├── cli.rs
│   ├── main.rs
│   └── tree.rs
└── .git
```

With no args passed to it, it will print everything in the current directory. 

To print out the tree of a selective directory, you can use the `-d` flag
```bash 
# -i is still an option here
~/crabtree   cargo run -q -- -d src
src
├─viz.rs
├─cli.rs
├─main.rs
└─tree.rs
```

- - - 

#### Future Goals (in order):
 - Package as a crate
 - Refactor with better practices
 - Handling symlinks
 - Implement CLI natively
 - Adding more args for max & min depth
 - Better formatting and visualization

- - - 