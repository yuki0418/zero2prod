## Init Project
Run the following command
```
./scripts/init_db.sh
```
or if Postgres Docker container is already running, run the following command
```
SKIP_DOCKER=true ./scripts/init_db.sh

```

### Code Coverage
Install tarpaulin
```
cargo install cargo-tarpaulin
```
Run tarpaulin
```
cargo tarpaulin --ignore-tests
```

### Linting
Install clippy to your rustup
```
rustup component add clippy
```
Run clippy
```
cargo clippy
```
In CI pipeline, run clippy with the following command
```
cargo clippy -- -D warnings
```

### Formatting
Install rustfmt to your rustup
```
rustup component add rustfmt
```
Run rustfmt
```
cargo fmt
```
In CI pipeline, run rustfmt with the following command
```
cargo fmt -- --check
```

### Security Vulnerabilities
Install cargo-audit
```
cargo install cargo-audit
```
Run cargo-audit
```
cargo audit
```
It will scan dependency tree for security vulnerabilities

## Run App
```
cargo watch -x check -x test -x run
```