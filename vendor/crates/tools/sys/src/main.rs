use rayon::prelude::*;
use std::collections::*;
use std::io::prelude::*;

/// Namespaces to exclude from code generation for the `windows-sys` crate.
const EXCLUDE_NAMESPACES: &[&str] = &["Windows.Win32.System.WinRT.Xaml", "Windows.Win32.Interop", "Windows.Win32.System.Diagnostics.Debug.WebApp", "Windows.Win32.Web"];

// TODO: Force output path spec?
fn main() {
    let mut rustfmt = true;
    let mut expect_namespace = false;
    let mut namespace = String::new();
    for arg in std::env::args() {
        match arg.as_str() {
            "-p" => rustfmt = false,
            "-n" => expect_namespace = true,
            _ => {
                if expect_namespace {
                    namespace = arg;
                }
            }
        }
    }
    let mut output = std::path::PathBuf::from("crates/libs/sys/src/Windows");
    if namespace.is_empty() {
        let _ = std::fs::remove_dir_all(&output);
    }
    output.pop();
    let files = vec![metadata::reader::File::new("crates/libs/metadata/default/Windows.winmd").unwrap(), metadata::reader::File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap(), metadata::reader::File::new("crates/libs/metadata/default/Windows.Win32.Interop.winmd").unwrap()];
    let reader = &metadata::reader::Reader::new(&files);
    if !namespace.is_empty() {
        let tree = reader.tree(&namespace, &[]).expect("Namespace not found");
        gen_tree(reader, &output, &tree, rustfmt);
        return;
    }
    // TODO: Make this a function, this is hecka tedious!
    let root = metadata::reader::Tree {
        namespace: "Windows",
        nested: vec![(
            "Win32",
            metadata::reader::Tree {
                namespace: "Windows.Win32",
                nested: BTreeMap::from([("Graphics", reader.tree("Windows.Win32.Graphics", EXCLUDE_NAMESPACES).unwrap())]).into_iter().collect(),
            },
        )]
        .into_iter()
        .collect(),
    };
    let trees = root.flatten();
    trees.par_iter().for_each(|tree| gen_tree(reader, &output, tree, rustfmt));
    output.pop();
    output.push("Cargo.toml");
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"
[package]
name = "windows-sys"
version = "0.42.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
readme = "../../../docs/readme.md"
rust-version = "1.49"

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []
all-features = true

[target.i686-pc-windows-msvc.dependencies]
windows_i686_msvc = { version = "0.42.0" }

[target.i686-uwp-windows-msvc.dependencies]
windows_i686_msvc = { version = "0.42.0" }

[target.x86_64-pc-windows-msvc.dependencies]
windows_x86_64_msvc = { version = "0.42.0" }

[target.x86_64-uwp-windows-msvc.dependencies]
windows_x86_64_msvc = { version = "0.42.0" }

[target.aarch64-pc-windows-msvc.dependencies]
windows_aarch64_msvc = { version = "0.42.0" }

[target.aarch64-uwp-windows-msvc.dependencies]
windows_aarch64_msvc = { version = "0.42.0" }

[target.aarch64-pc-windows-gnullvm.dependencies]
windows_aarch64_gnullvm = { version = "0.42.0" }

[target.i686-pc-windows-gnu.dependencies]
windows_i686_gnu = { version = "0.42.0" }

[target.i686-uwp-windows-gnu.dependencies]
windows_i686_gnu = { version = "0.42.0" }

[target.x86_64-pc-windows-gnu.dependencies]
windows_x86_64_gnu = { version = "0.42.0" }

[target.x86_64-uwp-windows-gnu.dependencies]
windows_x86_64_gnu = { version = "0.42.0" }

[target.x86_64-pc-windows-gnullvm.dependencies]
windows_x86_64_gnullvm = { version = "0.42.0" }

[features]
default = []
deprecated = []
"#
        .as_bytes(),
    )
    .unwrap();

    // Skip the root Windows tree while writing features
    for tree in trees.iter().skip(1) {
        let feature = tree.namespace[root.namespace.len() + 1..].replace('.', "_");

        if let Some(pos) = feature.rfind('_') {
            let dependency = &feature[..pos];

            file.write_all(format!("{} = [\"{}\"]\n", feature, dependency).as_bytes()).unwrap();
        } else {
            file.write_all(format!("{} = []\n", feature).as_bytes()).unwrap();
        }
    }

    std::fs::copy("license-mit", "crates/libs/sys/license-mit").unwrap();
    std::fs::copy("license-apache-2.0", "crates/libs/sys/license-apache-2.0").unwrap();
}

fn gen_tree(reader: &metadata::reader::Reader, output: &std::path::Path, tree: &metadata::reader::Tree, rustfmt: bool) {
    println!("{}", tree.namespace);
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();

    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.sys = true;
    gen.cfg = true;
    gen.doc = true;
    let mut tokens = bindgen::namespace(&gen, tree);
    lib::format(tree.namespace, &mut tokens, rustfmt);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();
}