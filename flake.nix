{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };
  outputs = { self, nixpkgs, ... }:
    {
     devShells.aarch64-darwin.default =
        let
          pkgs = import nixpkgs {
            system = "aarch64-darwin";
          };
        in
        pkgs.mkShell {
          buildInputs = with pkgs; [
         ];
          nativeBuildInputs = with pkgs; [
            nixpkgs-fmt
            rustfmt
            cargo-edit
            cargo
            rustc
          ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
    };
}
