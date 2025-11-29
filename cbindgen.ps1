& cargo rustc --crate-type=cdylib --release
& cbindgen --config cbindgen.toml --crate cg-rust --output autogen/cg_rust.h