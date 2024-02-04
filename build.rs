use glib_build_tools::compile_resources;

fn main() {
    // actions
    compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "whiteboard.gresource",
    );
}