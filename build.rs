fn main() {
	println!("cargo::rerun-if-changed=linuxdoom-1.10");
	println!("cargo::rustc-link-lib=Xext");
	println!("cargo::rustc-link-lib=X11");
	println!("cargo::rustc-link-lib=nsl");
	println!("cargo::rustc-link-lib=m");
}
