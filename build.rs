use std::io::Read;

fn add_cfg_options(cfg: &mut std::process::Command) {
    let cfg = cfg
        .stdout(std::process::Stdio::piped())
        .spawn().expect("Could not launch config");
    let mut stdout = cfg.stdout.expect("Could not get output");
    let mut pcre_options : String = String::new();
    stdout.read_to_string(&mut pcre_options).expect("Could not read output");

    for opt in pcre_options.split(" ") {
        println!("cargo:rustc-link-arg={opt}");
    }
}

fn main() {
//    pkg_config::Config::new()
//        .atleast_version("3.4").probe("libffi").unwrap();
//    pkg_config::Config::new().probe("zlib").unwrap();
//    pkg_config::Config::new().probe("libselinux").unwrap();
//    pkg_config::Config::new().probe("gmodule-2.0").unwrap();
//    pkg_config::Config::new().probe("mount").unwrap();
//    pkg_config::Config::new().probe("blkid").unwrap();
//    pkg_config::Config::new().probe("fribidi").unwrap();
//    pkg_config::Config::new().probe("epoxy").unwrap();
//    pkg_config::Config::new().probe("harfbuzz").unwrap();
//    pkg_config::Config::new().probe("graphite2").unwrap();
//    pkg_config::Config::new().probe("fontconfig").unwrap();
//    pkg_config::Config::new().probe("pixman-1").unwrap();
//
//    add_cfg_options(std::process::Command::new("pcre2-config")
//                    .arg("--libs8"));
//    add_cfg_options(std::process::Command::new("pcre-config")
//                    .arg("--libs"));
//    add_cfg_options(std::process::Command::new("freetype-config")
//                    .arg("--libs"));
//    println!("cargo:rustc-link-arg=-lGL");
//    println!("cargo:rustc-link-arg=-lGLX");
//    println!("cargo:rustc-link-arg=-lX11");
//    println!("cargo:rustc-link-arg=-lXinerama");
//    println!("cargo:rustc-link-arg=-lXi");
//    println!("cargo:rustc-link-arg=-lXrender");
//    println!("cargo:rustc-link-arg=-lXrandr");
//    println!("cargo:rustc-link-arg=-lxcb");
//    println!("cargo:rustc-link-arg=-lxkbcommon");
//    pkg_config::Config::new().probe("gio-2.0").unwrap();

    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "tunnelhound.gresource"
    );
}
