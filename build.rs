use git_version::git_version;
use std::{env, fs, path::Path, process::Command};

fn render_schema(gschema: &str, out_dir: &Path) {
    let gettext_package = env::var("GETTEXT_PACKAGE").unwrap();
    let app_id = env::var("APP_ID").unwrap();

    let contents = fs::read_to_string(gschema).unwrap();
    let new = contents.replace("@gettext-package@", &gettext_package).replace("@app-id@", &app_id);

    let filename = Path::new(gschema).file_name().unwrap();
    let outfile = Path::new(filename).file_stem().unwrap();
    let outpath = out_dir.join(outfile);

    fs::write(outpath.clone(), new).unwrap();

    let outpath_file = outpath.to_string_lossy();
    println!("cargo:rerun-if-changed={gschema}");
    println!("cargo:rerun-if-changed={outpath_file}");
}

pub fn compile_schemas<P: AsRef<Path>>(gschemas: &[P]) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    for gschema in gschemas {
        render_schema(&gschema.as_ref().to_string_lossy(), out_dir);
    }

    let schema_dir = env::var("GSETTINGS_SCHEMA_DIR").unwrap();
    let output = Command::new("glib-compile-schemas")
        .arg("--strict")
        .arg("--targetdir")
        .arg(schema_dir)
        .arg(out_dir)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "glib-compile-schemas failed with exit status {} and stderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn quote(input: &str) -> String {
    format!(r#""{}""#, input)
}

fn render_config() {
    let app_id = env::var("APP_ID").unwrap();
    let gettext_package = env::var("GETTEXT_PACKAGE").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = Path::new(&manifest_dir).join("src");
    let locale_dir = Path::new(&manifest_dir).join("po");
    let data_dir = Path::new(&manifest_dir).join("data");
    let out_dir = env::var("OUT_DIR").unwrap();

    let config_in_path = src_dir.join("config.rs.in");
    let config_out_path = src_dir.join("config.rs");

    let contents = fs::read_to_string(&config_in_path).unwrap();
    let new = contents
        .replace("@APP_ID@", &quote(&app_id))
        .replace("@GETTEXT_PACKAGE@", &quote(&gettext_package))
        .replace("@LOCALEDIR@", &quote(&locale_dir.to_string_lossy()))
        .replace("@PKGDATADIR@", &quote(&data_dir.to_string_lossy()))
        .replace("@RESOURCEDIR@", &quote(&out_dir))
        .replace("@PROFILE@", "Devel")
        .replace("@VERSION@", &quote(git_version!()));

    fs::write(config_out_path, new).unwrap();

    let config_in = config_in_path.to_string_lossy();
    println!("cargo:rerun-if-changed={config_in}");
}

fn compile_glib() {
    glib_build_tools::compile_resources(&["data/resources/"], "data/resources/resources.gresource.xml", "resources.gresource");

    compile_schemas(&["data/io.github.vhdirk.Terms.gschema.xml.in"]);
}

fn main() {
    if !env::var("BUILD_IS_MESON").is_ok_and(|val| val == "true") {
        compile_glib();
        render_config();
    }
}
