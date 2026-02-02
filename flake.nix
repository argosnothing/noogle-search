{
  description = "Noogle search for television - Rust development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "noogle-search-tv";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [pkgs.makeWrapper pkgs.pkg-config];
          buildInputs = [pkgs.openssl.dev];

          postInstall = ''
            wrapProgram $out/bin/noogle-search-tv \
              --prefix PATH : ${pkgs.lib.makeBinPath [pkgs.bat pkgs.fzf]}

            makeWrapper $out/bin/noogle-search-tv $out/bin/noogle-search \
              --add-flags search \
              --prefix PATH : ${pkgs.lib.makeBinPath [pkgs.bat pkgs.fzf]}
          '';

          meta = {
            description = "Search Noogle functions with fzf";
            mainProgram = "noogle-search";
          };
        };

        packages.noogle-search = pkgs.writeShellScriptBin "noogle-search" ''
          exec ${self.packages.${system}.default}/bin/noogle-search-tv search
        '';

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer

            pkg-config
            openssl

            bat
            fzf
          ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
