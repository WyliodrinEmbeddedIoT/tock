fn main() {
    println!("cargo:rustc-link-search=chips/esp32-c3/esp32-wifi-lib/esp32c3");
    // println!("cargo:rustc-link-lib=static=coexist");
    println!("cargo:rustc-link-lib=static=core");
    println!("cargo:rustc-link-lib=static=espnow");
    println!("cargo:rustc-link-lib=static=mesh");
    println!("cargo:rustc-link-lib=static=net80211");
    println!("cargo:rustc-link-lib=static=phy");
    println!("cargo:rustc-link-lib=static=pp");
    println!("cargo:rustc-link-lib=static=smartconfig");
    println!("cargo:rustc-link-lib=static=wapi");
}
