fn main() {
    println!("cargo:rerun-if-changed=protocols/hyprland-toplevel-export-v1.xml");
    println!("cargo:rerun-if-changed=config.example.toml");
}
