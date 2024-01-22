fn main() {
    println!("cargo:rerun-if-changed=resources/resources.gresource.xml");
    println!("cargo:rerun-if-changed=resources/slide.ui");
    println!("cargo:rerun-if-changed=resources/window.ui");
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "beamer-quickie.gresource",
    );
}
