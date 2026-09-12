fn main() {
	println!("cargo::rustc-link-lib=Xext");
	println!("cargo::rustc-link-lib=X11");
}
