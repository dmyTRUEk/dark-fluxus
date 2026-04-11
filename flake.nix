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
		targetPkgs = pkgs.pkgsCross.gnu64;
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

			nativeBuildInputs = with pkgs; [
				pkg-config
			];

			buildInputs = with pkgs; [
				# pkgsCross.mingwW64.SDL
				# pkgsCross.mingwW64.pkg-config
				targetPkgs.sdl3
				# pkg-config
				# libxkbcommon
				# cmake
			];

			PKG_CONFIG_ALLOW_CROSS = "1";
			PKG_CONFIG_PATH = "${targetPkgs.sdl3.dev}/lib/pkgconfig";

			RUSTFLAGS = [
				"-L${targetPkgs.sdl3}/lib"
			];
		};
	};
}
