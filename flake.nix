{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
		naersk = {
			url = "github:nix-community/naersk";
			inputs.nixpkgs.follows = "nixpkgs";
		};
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { self, nixpkgs, naersk, fenix }:
	let
		system = "x86_64-linux";
		pkgs = import nixpkgs {
			inherit system;
			config.allowUnfree = true;
			config.microsoftVisualStudioLicenseAccepted = true;
			# config.allowUnsupportedSystem = true;
			# config.allowBroken = true;
		};
		# targetGenericLinux = "x86_64-unknown-linux-gnu";
		# targetPkgsLinux = pkgs.pkgsCross.gnu64;
		# targetPkgsWindows = pkgs.pkgsCross.mingwW64;
		# targets = [ ];
		# buildExe = targetPkgs: { };
		naersk' = pkgs.callPackage naersk {};
	in {
		# packages.${system} = {};

		packages.${system}.windows =
		let
			target = "x86_64-pc-windows-gnu";
			targetPkgs = pkgs.pkgsCross.mingwW64;
			toolchain = with fenix.packages.${system}; combine [
				latest.cargo
				latest.rustc
				targets.${target}.latest.rust-std
			];
			# toolchain = fenix.packages.${system}.fromManifestFile ./rust-toolchain.toml;
			# toolchain = with fenix.packages.${system}.fromManifestFile ./rust-toolchain.toml; combine [
			# 	minimal.cargo
			# 	minimal.rustc
			# 	targets.${target}.latest.rust-std
			# ];
		in
		(naersk.lib.${system}.override {
			cargo = toolchain;
			rustc = toolchain;
		}).buildPackage {
			src = ./.;
			CARGO_BUILD_TARGET = target;
			CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER =
				let
					inherit (targetPkgs.stdenv) cc;
				in
					"${cc}/bin/${cc.targetPrefix}cc";
			nativeBuildInputs = with pkgs; [
				pkg-config
			];
			buildInputs = with targetPkgs; [
				sdl3
				windows.mingw_w64
				# windows.sdk
				windows.mingw_w64-ucrt-x86_64-toolchain
				windows.mingw_w64-ucrt-x86_64-nasm
				# llvmPackages.bintools
			];
			RUSTFLAGS = [
				"-L${targetPkgs.sdl3}/lib"
				"-L${targetPkgs.windows.mingw_w64}/lib"
			];
		};

		# packages.${system}.default =
		# let
		# 	target = "aarch64-unknown-linux-gnu";
		# 	targetPkgs = pkgs.pkgsCross.aarch64-multiplatform;
		# 	toolchain = with fenix.packages.${system}; combine [
		# 		latest.cargo
		# 		latest.rustc
		# 		targets.${target}.latest.rust-std
		# 	];
		# 	# toolchain = fenix.packages.${system}.fromManifestFile ./rust-toolchain.toml;
		# 	# toolchain = with fenix.packages.${system}.fromManifestFile ./rust-toolchain.toml; combine [
		# 	# 	minimal.cargo
		# 	# 	minimal.rustc
		# 	# 	targets.${target}.latest.rust-std
		# 	# ];
		# in
		# (naersk.lib.${system}.override {
		# 	cargo = toolchain;
		# 	rustc = toolchain;
		# }).buildPackage {
		# 	src = ./.;
		# 	CARGO_BUILD_TARGET = target;
		# 	CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
		# 		let
		# 			inherit (targetPkgs.stdenv) cc;
		# 		in
		# 			"${cc}/bin/${cc.targetPrefix}cc";
		# 	nativeBuildInputs = with pkgs; [
		# 		pkg-config
		# 	];
		# 	buildInputs = with targetPkgs; [
		# 		sdl3
		# 	];
		# 	RUSTFLAGS = [
		# 		"-L${targetPkgs.sdl3}/lib"
		# 	];
		# };
		#
		# packages.${system}.generic-linux = ;

		devShells.${system}.default = pkgs.mkShell rec {
			packages = with pkgs; [
				# pkg-config
				# libxkbcommon
				# libxkbcommon # for minifb
				# clang llvmPackages.libclang cmake # for llama-cpp-2
				sdl3
				# cmake
				# crossSystem
				# pkgsCross.mingwW64.SDL
				# pkgsCross.mingwW64.pkg-config
			];
			# Environment variables:

			# RUST_BACKTRACE = "full";

			nativeBuildInputs = with pkgs; [
				pkg-config
			];

			buildInputs = with pkgs; [
				# pkgsCross.mingwW64.sdl3
				# pkgsCross.mingwW64.pkg-config
				# targetPkgsLinux.sdl3
				# pkg-config
				# libxkbcommon
				# cmake
			];

			# RUSTFLAGS = [
			# 	"-L${targetPkgsLinux.sdl3}/lib"
			# ];
		};
	};
}
