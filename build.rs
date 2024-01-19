fn main() {
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
		.compile("asm-lib");
}