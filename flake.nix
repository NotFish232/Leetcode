{
  description = "Rust + Python Flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-darwin" ];
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          python = pkgs.python312;
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              # Rust deps
              rustc
              cargo
              clippy
              rust-analyzer
              rustfmt

              # Python + Deps
              python
              python.pkgs.sortedcontainers
              python.pkgs.typer
              python.pkgs.browser-cookie3
              python.pkgs.natsort
              python.pkgs.black
              python.pkgs.isort
            ];
          };
        });
    };
}
