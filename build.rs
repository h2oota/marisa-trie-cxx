use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::Write;

use ureq;
use cargo_metadata;
use cmake::Config;

trait FunctionBlocker
{
    fn disable_functions(self, functions: Vec<&str>) -> bindgen::Builder;
}

impl FunctionBlocker for bindgen::Builder
{
    fn disable_functions(self, functions: Vec<&str>) -> Self {
	let mut this = self;
	for func in functions.iter() {
	    this = this.blocklist_function(func);
	}
	this
    }
}

fn build_marisa(out_dir: &Path) -> PathBuf
{
    // get version from package.metadata in Cargo.toml.
    let metadata = cargo_metadata::MetadataCommand::new()
	.exec() .expect("Failed to read Cargo metadata");
    let pkg = metadata
	.packages .iter()
	.find(|p| p.name == metadata.root_package().unwrap().name)
	.unwrap();
    let version = pkg
	.metadata
	.get("marisa_version")
	.expect("package.metadata.marisa_version not found")
	.as_str()
	.expect("marisa_version must be a string");

    // GitHub archive URL
    let url = format!( "https://github.com/s-yata/marisa-trie/archive/refs/tags/{}.tar.gz", version);

    let download_path = out_dir.join("marisa-src.tar.gz");
    let extract_dir = out_dir.join("marisa-src");

    // skip if extracted
    if !extract_dir.exists() {
	// --- 1.  downloads ---
	let bytes = ureq::get(&url)
	    .call()
	    .expect("Download failed")
	    .body_mut()
	    .with_config()
	    .limit(50 * 1024 * 1024) // 50MB
	    .read_to_vec()
	    .expect("Download failed");

	let mut file = std::fs::File::create(&download_path).expect("Failed to create file");
	file.write_all(&bytes).expect("Failed to write file");
	// --- 2. extract ---
	std::fs::create_dir_all(&extract_dir).unwrap();
	let status = Command::new("tar")
	    .args(&[ "xf", download_path.to_str().unwrap(), "-C", extract_dir.to_str().unwrap(), "--strip-components=1", ])
	    .status()
	    .expect("Failed to extract archive");
	assert!(status.success());
    }

    // --- configure + build + install ---
    let dst = Config::new(extract_dir)
	.profile("Release") // -DCMAKE_BUILD_TYPE=Release
	.define("ENABLE_NATIVE_CODE", "ON")
	.define("BUILD_TESTING", "OFF")
	.build();

    dst.join("lib")
}

fn check_compiler() -> (/* cc */ String, /* c++ */ String, /* rutimelib */ Option<String>)
{
    let target = std::env::var("TARGET").unwrap();

    let cc = {
	if let Ok(cc) = std::env::var("CC") {
	    cc
	} else 	if target.contains("windows-msvc") {
	    String::from("cl")
	} else if target.contains("windows-gnu") {
	    String::from("gcc")
	} else if target.contains("apple-darwin") {
	    String::from("clang")
	} else {
	    String::from("cc")
	}
    };

    let cxx = {
	if let Ok(cxx) = std::env::var("CXX") {
	    cxx
	} else 	if target.contains("windows-msvc") {
	    String::from("cl")
	} else if target.contains("windows-gnu") {
	    String::from("g++")
	} else if target.contains("apple-darwin") {
	    String::from("clang++")
	} else {
	    String::from("c++")
	}
    };

    let lib = {
	if target.contains("windows-msvc") || cxx == "cl" || cxx == "clang-cl" {
	    None
	} else if target.contains("apple-darwin") {
	    Some(String::from("c++"))
	} else {
	    Some(String::from("stdc++"))
	}
    };

    (cc, cxx, lib)
}


fn main() {
    fn combine_paths<'a>(dir: &'a str, files: Vec<&'a str>) -> Vec<String> {
	files
            .into_iter()
            .map(|file| std::path::Path::new(dir).join(file).to_str().unwrap().to_string())
            .collect()
    }

    fn call_c(lib: &libloading::Library, name: &[u8]) -> usize {
	unsafe {
            let func: libloading::Symbol<unsafe extern "C" fn() -> usize> =
		lib
		.get(name)
		.expect("Failed to find function 'add'");

            // 関数を呼び出して値を取得
	    func()
	}
    }

    fn get_number_size(i: usize) -> usize {
	match i {
	    1 ..= 8 => 8,
	    9 ..= 16 => 16,
	    17 ..= 32 => 32,
	    31 ..= 64 => 64,
	    65 ..= 128 => 128,
	    _ => {
		panic!("no suitable integer for {}", i);
	    }
	}
    }

    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "macos" {
        let sdk_path = Command::new("xcrun")
            .arg("--show-sdk-path")
            .output()
            .expect("Failed to get SDK path")
            .stdout;

        let sdk_path = String::from_utf8(sdk_path).expect("Invalid UTF-8 output").trim().to_string();

	unsafe {std::env::set_var("SDKROOT", sdk_path)}
    }

    let out_dir_str = env::var("OUT_DIR").expect("Failed to get OUT_DIR");
    let out_dir = Path::new(&out_dir_str);

    let (libmarisa_config, cxx, libcxx) = {
	let old_var = std::env::var_os("PKG_CONFIG_PATH");
	let use_system_marisa = std::env::var("CARGO_FEATURE_USE_SYSTEM_MARISA").is_ok();
	let (_cc, cxx, libcxx) = check_compiler();

	// download and compile marisa-trie from github.
	if ! use_system_marisa {
	    let marisa_dir = build_marisa(&out_dir); // download & compile marisa-trie
	    std::env::set_var("PKG_CONFIG_PATH", marisa_dir.join("pkgconfig"));
	}

	let libmarisa_config = match pkg_config::Config::new()
	    .statik(true)
	    .probe("marisa") {
		Ok(result) => result,
		Err(err) => panic!("{:?}", err)
	    };
	if old_var.is_none() {
	    std::env::remove_var("PKG_CONFIG_PATH");
	} else {
	    std::env::set_var("PKG_CONFIG_PATH", old_var.unwrap());
	}
	if use_system_marisa {
	    (libmarisa_config, cxx, None)
	} else {
	    (libmarisa_config, cxx, libcxx)
	}
    };

    let lib_name = format!(
	"libc_number_width.{}",
	match env::var("CARGO_CFG_TARGET_OS")
            .expect("Failed to get target OS")
            .as_str() {
		"windows" => "dll",
		"macos" => "dylib",
		_       => "so",
            });
    let lib_path = Path::new(&out_dir).join(&lib_name);



    // sizeof(int)とsizeof(void*)を取得するライブラリ
    let status = std::process::Command::new(cxx)
        .arg("-shared") // 共有ライブラリを生成するフラグ
        .arg("-fPIC")   // ポジション独立コードの有効化
        .arg("-o")      // 出力ファイル指定
        .arg(lib_path.to_str().unwrap()) // 出力ファイルのパス
        .arg("src/c_number_width.cpp")    // 入力ファイル (Cコード)
        .status()
        .expect("Failed to execute clang");

    if !status.success() {
        panic!("Clang failed to compile the shared library.");
    }

    // 動的ライブラリをロード
    let libc_number_width = unsafe {
	libloading::Library::new(&lib_path)
	    .expect("Failed to load dynamic library")
    };

    // ライブラリから取得
    let char_bits = call_c(&libc_number_width, b"char_bits");
    let pointer_width = call_c(&libc_number_width, b"pointer_width");
    let int_width = call_c(&libc_number_width, b"int_width");
    let float_width = call_c(&libc_number_width, b"float_width");
    let double_width = call_c(&libc_number_width, b"double_width");

    if char_bits > 8 {
	panic!("unsupported char size({})", char_bits);
    }

    // bindgenで自動生成したソースはusize_tがuint64_t/uint32_tに
    // usizeとu64/u32は異なり直接渡せない(引数、戻り値)
    // usize と u64/u32を相互変換するrustで書かれた小さなコード断片を生成してインクルードする

    let size_converter_path = Path::new(&out_dir).join("size_converter.rs");


    let generated_code = format!(
            r#"
// rustのstrをCに渡すときはUTF-8のバイトの配列として
// C:sizeof(char) >= 8
// rust:
// c -> rust
#[allow(dead_code)]
pub type usize_int = u{target_size};
// rust -> c
#[allow(dead_code)]
pub type cint = i{target_int};
#[allow(dead_code)]
pub type cuint = u{target_int};
#[allow(dead_code)]
pub type cchar = i{target_char};
#[allow(dead_code)]
pub type cuchar = u{target_char};
#[allow(dead_code)]
pub type cfloat = f{target_float};
#[allow(dead_code)]
pub type cdouble = f{target_double};
#[allow(dead_code)]
#[inline(always)]
pub fn usize_to_csize(value: usize) -> u{target_size} {{
    value as u{target_size}
}}

#[allow(dead_code)]
#[inline(always)]
pub fn csize_to_usize(value: u{target_size}) -> usize {{
    value as usize
}}

const _ : (usize_int, cint, cuint, cchar, cuchar, cfloat, cdouble) = (0, 0, 0, 0, 0, 0.0, 0.0);

"#,
	target_size = get_number_size(pointer_width),
	target_int = get_number_size(int_width),
	target_char = get_number_size(char_bits),
	target_float = get_number_size(float_width),
	target_double = get_number_size(double_width),
    );

    fs::write(&size_converter_path, generated_code).expect("Failed to write generated file");


    let src_files = combine_paths(
	"src/marisa",
	vec!["agent.cpp", "key.cpp", "keyset.cpp", "query.cpp", "trie.cpp", "except.cpp"]);

    cc::Build::new()
        .cpp(true)                  // C++を有効にする
	.files(&src_files)
	.includes(libmarisa_config.include_paths.clone())
	.flag("-fno-inline-functions")
	.flag("-std=c++17")
	.pic(true)
        .compile("marisa_wrapper");         // libmarisa_wrapper.a


    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let disable_in_release = if env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string()) == "release" {
	vec![
	    "marisa_wrapper::key_set_(?:str|id|weight)|ptr|length|weight",
	    "marisa_wrapper::query_set_(?:str|id)|ptr|length",
	    "marisa_wrapper::keyset_size",
	]
    } else {
	Vec::new()
    };


    let bindings = bindgen::Builder::default()
        .impl_debug(true)
        .size_t_is_usize(true)
        .generate_cstr(true)
        .clang_arg("-std=c++17")
        .clang_arg("--language=c++")
	.clang_args(libmarisa_config
		   .include_paths
		    .clone()
		   .iter()
		   .map(|path| format!("-I{}", path.display()))
		   .chain(libmarisa_config
			  .defines
			  .clone()
			  .iter()
			  .map(|(key, value)|
			       match value {
				   Some(value) => format!("-D{}={}", key, value),
				   None => format!("-D{}", key),
			       }))
		   .collect::<Vec<String>>())
        .header("src/marisa/marisa-wrapper.hpp")
        .allowlist_function("(?:key|query|keyset|agent|trie)_[A-Za-z0-9_]+")
        .allowlist_function("exception_(?:name|message)")
        .allowlist_type("marisa_num_tries|marisa_cache_level|marisa_tail_mode|marisa_node_order|marisa_config_mask")
	.opaque_type("marisa::.*")
        .disable_functions(disable_in_release)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
//        .no_copy(true)
        .generate()
        .expect("Unable to generate  bindings");

    bindings
        .write_to_file(out_path.join("marisa_bindings.rs"))
        .expect("Couldn't write bindings!");

    for dir in libmarisa_config.link_paths.clone() {
	println!("cargo:rustc-link-search=native={}", dir.display());
    }

    if let Some(lib) = libcxx { // C++ runtime for marisa static library
	println!("cargo:rustc-link-lib={}", lib);
    }

    println!("cargo:rustc-link-lib=marisa");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/marisa/marisa-wrapper.hpp");

    for file in src_files.into_iter() {
	println!("cargo:rerun-if-changed={}", file);
    }
}
