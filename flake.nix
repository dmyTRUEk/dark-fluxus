{
	inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

	outputs = { self, nixpkgs }:
	let
		system = "x86_64-linux";
		pkgs = import nixpkgs {
			inherit system;
			# config.allowUnsupportedSystem = true;
			# config.allowBroken = true;
		};
	in {
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

			# buildInputs = with pkgs; [
			# 	pkgsCross.mingwW64.SDL
			# 	pkgsCross.mingwW64.pkg-config
			# ];
			#
			# # This is the important part:
			# LIBRARY_PATH = "${pkgs.pkgsCross.mingwW64.SDL}/lib";
			# C_INCLUDE_PATH = "${pkgs.pkgsCross.mingwW64.SDL}/include";
			#
			# # Often needed for Rust build scripts:
			# PKG_CONFIG_PATH = "${pkgs.pkgsCross.mingwW64.SDL}/lib/pkgconfig";
		};
	};
}
