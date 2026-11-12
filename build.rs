// Embeds assets/icon.ico in the .exe (Explorer, taskbar pins, Alt+Tab).
fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        // Do not break the build if the resource compiler is missing.
        if let Err(e) = res.compile() {
            println!("cargo:warning=icon not embedded in the exe: {e}");
        }
    }
}
