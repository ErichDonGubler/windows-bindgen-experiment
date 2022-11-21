use rayon::prelude::*;
use std::collections::*;
use std::io::prelude::*;

const EXCLUDE_NAMESPACES: &[&str] = &["Windows.AI.MachineLearning.Preview", "Windows.ApplicationModel.SocialInfo", "Windows.Devices.AllJoyn", "Windows.Devices.Perception", "Windows.Security.Authentication.Identity.Provider", "Windows.Services.Cortana", "Windows.System.Power.Diagnostics", "Windows.System.Preview", "Windows.UI.Xaml", "Windows.Win32.Interop", "Windows.Win32.System.Diagnostics.Debug.WebApp", "Windows.Win32.System.WinRT.Xaml", "Windows.Win32.Web"];

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
    let mut output = std::path::PathBuf::from("crates/libs/windows/src/Windows");
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
    let root = metadata::reader::Tree {
        namespace: "Windows",
        nested: BTreeMap::from([(
            "Win32",
            metadata::reader::Tree {
                namespace: "Windows.Win32",
                nested: BTreeMap::from([(
                    "Graphics",
                    metadata::reader::Tree {
                        namespace: "Windows.Win32.Graphics",
                        nested: BTreeMap::from([
                            ("CompositionSwapchain", reader.tree("Windows.Win32.Graphics.CompositionSwapchain", EXCLUDE_NAMESPACES).unwrap()),
                            ("DXCore", reader.tree("Windows.Win32.Graphics.DXCore", EXCLUDE_NAMESPACES).unwrap()),
                            ("Direct3D", reader.tree("Windows.Win32.Graphics.Direct3D", EXCLUDE_NAMESPACES).unwrap()),
                            ("Direct3D11", reader.tree("Windows.Win32.Graphics.Direct3D11", EXCLUDE_NAMESPACES).unwrap()),
                            ("Direct3D12", reader.tree("Windows.Win32.Graphics.Direct3D12", EXCLUDE_NAMESPACES).unwrap()),
                            ("DirectComposition", reader.tree("Windows.Win32.Graphics.DirectComposition", EXCLUDE_NAMESPACES).unwrap()),
                            ("DirectManipulation", reader.tree("Windows.Win32.Graphics.DirectManipulation", EXCLUDE_NAMESPACES).unwrap()),
                            ("Dwm", reader.tree("Windows.Win32.Graphics.Dwm", EXCLUDE_NAMESPACES).unwrap()),
                            ("Dxgi", reader.tree("Windows.Win32.Graphics.Dxgi", EXCLUDE_NAMESPACES).unwrap()),
                            ("Gdi", reader.tree("Windows.Win32.Graphics.Gdi", EXCLUDE_NAMESPACES).unwrap()),
                            ("Hlsl", reader.tree("Windows.Win32.Graphics.Hlsl", EXCLUDE_NAMESPACES).unwrap()),
                        ]),
                    },
                )]),
            },
        )])
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
name = "windows"
version = "0.42.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
documentation = "https://microsoft.github.io/windows-docs-rs/"
readme = "../../../docs/readme.md"
rust-version = "1.59"
[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []
[target.i686-pc-windows-msvc.dependencies]
windows_i686_msvc = { path = "../../targets/i686_msvc", version = "0.42.0" }
[target.i686-uwp-windows-msvc.dependencies]
windows_i686_msvc = { path = "../../targets/i686_msvc", version = "0.42.0" }
[target.x86_64-pc-windows-msvc.dependencies]
windows_x86_64_msvc = { path = "../../targets/x86_64_msvc", version = "0.42.0" }
[target.x86_64-uwp-windows-msvc.dependencies]
windows_x86_64_msvc = { path = "../../targets/x86_64_msvc", version = "0.42.0" }
[target.aarch64-pc-windows-msvc.dependencies]
windows_aarch64_msvc = { path = "../../targets/aarch64_msvc", version = "0.42.0" }
[target.aarch64-uwp-windows-msvc.dependencies]
windows_aarch64_msvc = { path = "../../targets/aarch64_msvc", version = "0.42.0" }
[target.aarch64-pc-windows-gnullvm.dependencies]
windows_aarch64_gnullvm = { path = "../../targets/aarch64_gnullvm", version = "0.42.0" }
[target.i686-pc-windows-gnu.dependencies]
windows_i686_gnu = { path = "../../targets/i686_gnu", version = "0.42.0" }
[target.i686-uwp-windows-gnu.dependencies]
windows_i686_gnu = { path = "../../targets/i686_gnu", version = "0.42.0" }
[target.x86_64-pc-windows-gnu.dependencies]
windows_x86_64_gnu = { path = "../../targets/x86_64_gnu", version = "0.42.0" }
[target.x86_64-uwp-windows-gnu.dependencies]
windows_x86_64_gnu = { path = "../../targets/x86_64_gnu", version = "0.42.0" }
[target.x86_64-pc-windows-gnullvm.dependencies]
windows_x86_64_gnullvm = { path = "../../targets/x86_64_gnullvm", version = "0.42.0" }
[dependencies]
windows-implement = { path = "../implement",  version = "0.42.0", optional = true }
windows-interface = { path = "../interface",  version = "0.42.0", optional = true }
[features]
default = []
deprecated = []
implement = ["windows-implement"]
interface = ["windows-interface"]
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

    file.write_all(
        r#"
# UNSTABLE FEATURES (requires Rust nightly)
# Enable to use the #[debugger_visualizer] attribute.
debugger_visualizer = []

"#
        .as_bytes(),
    )
    .unwrap();

    std::fs::copy("license-mit", "crates/libs/windows/license-mit").unwrap();
    std::fs::copy("license-apache-2.0", "crates/libs/windows/license-apache-2.0").unwrap();
}

fn gen_tree(reader: &metadata::reader::Reader, output: &std::path::Path, tree: &metadata::reader::Tree, rustfmt: bool) {
    println!("{}", tree.namespace);
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();

    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.cfg = true;
    gen.doc = true;
    let mut tokens = bindgen::namespace(&gen, tree);
    tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
    lib::format(tree.namespace, &mut tokens, rustfmt);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();
    let mut tokens = bindgen::namespace_impl(&gen, tree);
    lib::format(tree.namespace, &mut tokens, rustfmt);
    std::fs::write(path.join("impl.rs"), tokens).unwrap();
}
