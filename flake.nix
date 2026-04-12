{
	inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

	outputs = { self, nixpkgs }:
	let
		system = "x86_64-linux";
		pkgs = import nixpkgs {
			inherit system;
		};
		targetPkgsLinux = pkgs.pkgsCross.gnu64;
		targetPkgsWindows = pkgs.pkgsCross.mingwW64;
	in {
		devShells.${system}.default = pkgs.mkShell rec {
			packages = with pkgs; [
				#clang llvmPackages.libclang cmake # for llama-cpp-2
				sdl3
			];

			RUSTFLAGS = [
				"-L${targetPkgsLinux.sdl3}/lib"
				"-L${targetPkgsWindows.sdl3}/lib"
			];
		};
	};
}
