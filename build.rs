fn main() {
	println!("cargo:rerun-if-changed=src/gdt/gdt.s");
	println!("cargo:rerun-if-changed=src/idt/idt.s");
	println!("cargo:rerun-if-changed=src/arch/x86/linker.ld");
	println!("cargo:rustc-link-arg=-Tsrc/arch/x86/linker.ld");	
	cc::Build::new()
		.compiler("gcc")
		.flag("-nostdlib")
		.flag("-ffreestanding")
		.flag("-fno-stack-protector")
		.flag("-mno-red-zone")
		.flag("-Wall")
		.flag("-m32")
		.flag("-Wextra")
		.file("src/gdt/gdt.s")
		.file("src/idt/idt.s")
		.compile("asm-lib");
}
