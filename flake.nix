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
      venv_dir = "./venv/";
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          clippy
          rust-analyzer
          rustfmt
          python.pkgs.venvShellHook
        ];
        venvDir = venv_dir;
        postShellHook = ''
          SENTINEL="${venv_dir}/.installed"
          REQUIREMENTS="requirements.txt"

          if [ ! -f "$SENTINEL" ] || [ "$REQUIREMENTS" -nt "$SENTINEL" ]; then
            pip install --upgrade pip
            pip install -r "$REQUIREMENTS"
            touch "$SENTINEL"
          fi
        '';
      };
    };
}
