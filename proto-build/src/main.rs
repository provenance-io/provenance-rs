//! Build ProvenanceSDK proto files. This build script clones the ProvenanceSDK version
//! specified in the PROVENANCE_SDK_REV constant and then uses that to build the required
//! proto files for further compilation. This is based on the proto-build code
//! in https://github.com/cosmos/cosmos-rust

use regex::Regex;
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process,
    sync::atomic::{self, AtomicBool},
};
use walkdir::WalkDir;

/// Suppress log messages
static QUIET: AtomicBool = AtomicBool::new(false);

/// The Provenance SDK commit or tag to be cloned and used to build the proto files
const PROVENANCE_SDK_REV: &str = "v1.14.1";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const PROVENANCE_SDK_PROTO_DIR: &str = "../provenance-sdk-proto/src/prost/";
/// Directory where the cosmos-sdk submodule is located
const PROVENANCE_SDK_DIR: &str = "../provenance-sdk-go";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

// Patch strings used by `copy_and_patch`

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &[
    "cosmos",
    "cosmos_proto",
    "ibc",
    "gogoproto",
    "google",
    "tendermint",
];

/// Log info to the console (if `QUIET` is disabled)
macro_rules! info {
    ($msg:expr) => {
        if !is_quiet() {
            println!("[info] {}", $msg)
        }
    };
    ($fmt:expr, $($arg:tt)+) => {
        info!(&format!($fmt, $($arg)+))
    };
}

fn main() {
    if is_github() {
        set_quiet();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = PROVENANCE_SDK_PROTO_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    let temp_sdk_dir = tmp_build_dir.join("provenance-sdk");

    fs::create_dir_all(&temp_sdk_dir).unwrap();

    update_submodules();
    output_sdk_version(&temp_sdk_dir);
    compile_sdk_protos_and_services(&temp_sdk_dir);

    copy_generated_files(&temp_sdk_dir, &proto_dir.join("provenance-sdk"));

    info!("Running rustfmt on prost/tonic-generated code");
    run_rustfmt(&proto_dir);

    if is_github() {
        println!("Rebuild protos with proto-build (cosmos-sdk rev: {PROVENANCE_SDK_REV})");
    }
}

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn set_quiet() {
    QUIET.store(true, atomic::Ordering::Relaxed);
}

/// Parse `--github` flag passed to `proto-build` on the eponymous GitHub Actions job.
/// Disables `info`-level log messages, instead outputting only a commit message.
fn is_github() -> bool {
    env::args().any(|arg| arg == "--github")
}

fn run_cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new(&cmd)
        .args(args)
        .stdout(stdout)
        .status()
        .expect("exit status missing");

    if !exit_status.success() {
        panic!(
            "{:?} exited with error code: {:?}",
            cmd.as_ref(),
            exit_status.code()
        );
    }
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    run_cmd("git", args)
}

fn run_rustfmt(dir: &Path) {
    let mut args = ["--edition", "2021"]
        .iter()
        .map(Into::into)
        .collect::<Vec<OsString>>();

    args.extend(
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension() == Some(OsStr::new("rs")))
            .map(|e| e.into_path())
            .map(Into::into),
    );

    run_cmd("rustfmt", args);
}

fn update_submodules() {
    info!("Updating provenance/provenance-sdk submodule...");
    run_git(["submodule", "update", "--init"]);
    run_git(["-C", PROVENANCE_SDK_DIR, "fetch"]);
    run_git([
        "-C",
        PROVENANCE_SDK_DIR,
        "reset",
        "--hard",
        PROVENANCE_SDK_REV,
    ]);
}

fn output_sdk_version(out_dir: &Path) {
    let path = out_dir.join("PROVENANCE_SDK_COMMIT");
    fs::write(path, PROVENANCE_SDK_REV).unwrap();
}

fn compile_sdk_protos_and_services(out_dir: &Path) {
    info!(
        "Compiling cosmos-sdk .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let sdk_dir = Path::new(PROVENANCE_SDK_DIR);

    let proto_includes_paths = [
        format!("{}/proto", sdk_dir.display()),
        format!("{}/third_party/proto", sdk_dir.display()),
    ];

    // Paths
    let proto_paths = [
        format!("{}/proto/provenance/attribute", sdk_dir.display()),
        format!("{}/proto/provenance/marker", sdk_dir.display()),
        format!("{}/proto/provenance/metadata", sdk_dir.display()),
        format!("{}/proto/provenance/msgfees", sdk_dir.display()),
        format!("{}/proto/provenance/name", sdk_dir.display()),
        format!("{}/proto/provenance/reward", sdk_dir.display()),
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all of the proto files, along with grpc service clients
    info!("Compiling proto definitions and clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir(out_dir)
        .extern_path(".tendermint", "::tendermint_proto")
        .compile(&protos, &includes)
        .unwrap();

    info!("=> Done!");
}

/// collect_protos walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(to_dir).unwrap_or_default();
    create_dir_all(to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    /// Regex substitutions to apply to the prost-generated output
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Use `tendermint-proto` proto definitions
        ("(super::)+cosmos", "cosmos_sdk_proto::cosmos"),
        ("(super::)+ibc", "cosmos_sdk_proto::ibc"),
        // Feature-gate gRPC client modules
        (
            "/// Generated client implementations.",
            "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
        // Feature-gate gRPC impls which use `tonic::transport`
        (
            "impl(.+)tonic::transport(.+)",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl${1}tonic::transport${2}",
        ),
        // Feature-gate gRPC server modules
        (
            "/// Generated server implementations.",
            "/// Generated server implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
    ];

    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let mut contents = fs::read_to_string(src)?;

    for &(regex, replacement) in REPLACEMENTS {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    fs::write(dest, &contents)
}
