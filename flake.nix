{
	# description = ""; # TODO

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
		rust-overlay.url = "github:oxalica/rust-overlay"; # for nightly
	};

	outputs = { self, nixpkgs, rust-overlay }:
	let
		system = "x86_64-linux";
		pkgs = import nixpkgs {
			inherit system;
			overlays = [ (import rust-overlay) ];
		};
		targetPkgsLinux = pkgs.pkgsCross.gnu64;
		targetPkgsWindows = pkgs.pkgsCross.mingwW64;
	in {
		devShells.${system}.default = pkgs.mkShell rec {
			packages = with pkgs; [
				#clang llvmPackages.libclang cmake # for llama-cpp-2
				# sdl3
				wayland libxkbcommon # for winit
				vulkan-loader # for wgpu
			];
			# RUSTFLAGS = [
			# 	"-L${targetPkgsLinux.sdl3}/lib"
			# 	"-L${targetPkgsWindows.sdl3}/lib"
			# ];
			LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages; # for wgpu
			WGPU_BACKEND = "vulkan"; # options: vulkan, metal, dx12, gl
		};

		packages.${system}.default =
		let
			cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
			pname = cargoToml.package.name;
			version = cargoToml.package.version;
		in
			pkgs.rustPlatform.buildRustPackage {
				inherit pname version;
				src = self;
				cargoLock.lockFile = ./Cargo.lock;
				nativeBuildInputs = with pkgs; [
					rust-bin.nightly.latest.default # nightly toolchain from the overlay
					# sdl3
					wayland libxkbcommon # for winit
					vulkan-loader # for wgpu
				];
			};
		apps.${system}.default = {
			type = "app";
			program = "${self.packages.${system}.default}/bin/${self.packages.${system}.default.pname}";
		};
	};
}
