fn main() {
    // Projenin ana dizinini bul.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // system-integration/build/Release klasörünün yolunu oluştur.
    // .. -> core'dan ana dizine çık
    // system-integration -> C++ klasörüne gir
    let lib_path = std::path::Path::new(&manifest_dir)
        .join("..")
        .join("system-integration")
        .join("build")
        .join("Release"); // Windows'ta CMake bu alt klasörü oluşturur.

    // Linker'a bu yolu kütüphane arama yollarına eklemesini söyle.
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    
    // Linker'a system_integration kütüphanesini bağlamasını söyle.
    println!("cargo:rustc-link-lib=dylib=system_integration");
}