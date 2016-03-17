fn main () {
    use std::env;

    let target = env::var("TARGET").unwrap();
    if target == "x86_64-unknown-linux-gnu" {
        println!("cargo:rustc-link-lib=static=pcap-1.7.4-glibc");
    } else if target == "x86_64-unknown-linux-musl" {
        println!("cargo:rustc-link-lib=static=pcap-1.7.4-musl-1.1.12");
    } else {
        panic!("unsupported target");
    }
    //println!("cargo:rustc-link-search=/tmp/pcap-static/lib/");
    println!("cargo:rustc-link-search={}", env::current_dir().unwrap().display());
}
