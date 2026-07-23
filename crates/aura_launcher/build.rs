fn main() {
    #[cfg(target_os = "windows")]
    {
        let icon_path = "../../assets/icon/aura.ico";
        println!("cargo:rerun-if-changed={icon_path}");
        if std::path::Path::new(icon_path).exists() {
            let mut resource = winres::WindowsResource::new();
            resource.set_icon(icon_path);
            if let Err(error) = resource.compile() {
                println!("cargo:warning=Could not embed AURA icon resource: {error}");
            }
        } else {
            println!("cargo:warning=AURA icon resource missing at {icon_path}");
        }
    }
}
