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

fn compile_glib() {
    glib_build_tools::compile_resources(&["data/resources/"], "data/resources/resources.gresource.xml", "resources.gresource");

    compile_schemas(&["data/io.github.vhdirk.Terms.gschema.xml.in"]);
}

fn main() {
    if !env::var("BUILD_IS_MESON").is_ok_and(|val| val == "true") {
        compile_glib();
    }
}
