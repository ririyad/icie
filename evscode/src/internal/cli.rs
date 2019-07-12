use crate::meta::Package;
use std::path::Path;

pub fn run_main(pkg: &Package, manifest_dir: &str) -> std::io::Result<()> {
	let exec_path = std::env::current_exe().expect("evscode::run_main current_exe() errored");
	let toolchain = crate::internal::compile::Toolchain::new(pkg, Path::new(manifest_dir), &exec_path);
	let subcommand = std::env::args().nth(1);
	let subcommand = subcommand.as_ref().map(|s| s.as_str());
	if subcommand == Some("--extension") {
		crate::internal::executor::execute(pkg);
	} else if subcommand == Some("--compile") {
		toolchain.compile(false)?;
	} else if subcommand == Some("--launch") || subcommand == None {
		toolchain.compile(false)?;
		toolchain.launch()?;
	} else if subcommand == Some("--package") {
		toolchain.compile(true)?;
		toolchain.package()?;
	} else if subcommand == Some("--publish") {
		toolchain.compile(true)?;
		toolchain.publish()?;
	} else {
		panic!("unrecognized subcommand");
	}
	Ok(())
}