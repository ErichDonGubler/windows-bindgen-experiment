# `windows-bindgen-experiment`

This is an experiment @ErichDonGubler did to determine if it's possible (circa
2022-11) to generate only a subset of the Rust code that's being published into
the `windows` and/or `windows-sys` crates available at the time of writing.

Short answer: yes, and with minimal changes from upstream! You will need to know
about the structure of `crates` directory tree:

* `libs` is composed of two kinds of crates: Crates like `metadata` are library
	crates for performing reading of Windows metadata files and emitting them as
	Rust source code.
	* Crates like `windows` and `sys` are crates composed of _both_ manually
		authored and generated code.
* `tools` is composed of code generation binary crates that are only run
	within a workspace's source tree. It uses some `libs` crates as
	dependencies, and a given `tools_*` crate emits code into a single specific
	output crate in `libs`.
