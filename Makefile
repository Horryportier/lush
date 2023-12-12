default:
	@cargo build; ./target/debug/lush
b:
	@cargo build
r:
	@cargo run
lib:
	@mkdir src/$(name); touch src/$(name)/mod.rs src/$(name)/$(name).rs
