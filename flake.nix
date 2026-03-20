{
	inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

	outputs = { self, nixpkgs }:
	let
		system = "x86_64-linux";
		pkgs = import nixpkgs { inherit system; };
	in {
		devShells.${system}.default = pkgs.mkShell rec {
			packages = with pkgs; [
				libxkbcommon # for minifb
				clang
				llvmPackages.libclang
				cmake
			];
			# Environment variables:
			# RUST_BACKTRACE = "full";
			LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
		};
	};
}
