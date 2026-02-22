{
  description = "Rust + Python Flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs, ... }@inputs:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      python = pkgs.python312;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          # Rust deps
          rustc
          cargo
          clippy
          rust-analyzer
          rustfmt

          # Python + Deps
          python
          python.pkgs.typer
          python.pkgs.browser-cookie3
          python.pkgs.natsort
          python.pkgs.black
          python.pkgs.isort
        ];
      };
    };
}
