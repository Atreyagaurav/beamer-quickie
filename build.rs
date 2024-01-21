fn main() {
    println!("cargo:rerun-if-changed=resources/resources.gresource.xml");
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "beamer-quickie.gresource",
    );
}
