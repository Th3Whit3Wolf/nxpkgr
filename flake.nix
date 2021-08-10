{

  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.url = "nixpkgs/release-21.05";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };
  outputs = { self, utils, rust-overlay, nixpkgs, naersk
    , flake-compat, ... }@inputs:
    {

      overlay = final: prev: {
        nxpkgr =
        let
        system = prev.system;
          overlays = [ rust-overlay.overlay ];
          pkgs = import nixpkgs { inherit system overlays; };

        rust-stable = pkgs.rust-bin.stable."1.54.0".default.override {
            extensions = [
              "cargo"
              "clippy"
              "rust-docs"
              "rust-src"
              "rust-std"
              "rustc"
              "rustfmt"
            ];
          };

          naersk-lib = (naersk.lib."${system}".override {
            cargo = rust-stable;
            rustc = rust-stable;
          }); 
        
        in naersk-lib.buildPackage {
          pname = "nxpkgr";
          nativeBuildInputs = with pkgs;
            [ pkg-config  ];
            buildInputs = [ ];
          root = ./.;
        };
      };

    } // utils.lib.eachDefaultSystem (system:
      let
        rust-stable = pkgs.rust-bin.stable."1.54.0".default.override {
          extensions = [
            "cargo"
            "clippy"
            "rust-docs"
            "rust-src"
            "rust-std"
            "rustc"
            "rustfmt"
          ];
        };
        pkgs = import nixpkgs {
          overlays = [ self.overlay rust-overlay.overlay ];
          inherit system;
        };

        # `nix develop`
        extensions = (with pkgs.vscode-extensions; [
          bbenoist.Nix
          matklad.rust-analyzer
          tamasfe.even-better-toml
          pkief.material-icon-theme
        ]) ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [
          {
            name = "spacemacs";
            publisher = "cometeer";
            version = "1.1.1";
            sha256 =
              "da54d2a40b72bb814b2e4af6b03eff6b3982162ae6f4492e6ceccad8f70cc7d3";
          }
          {
            name = "search-crates-io";
            publisher = "belfz";
            version = "1.2.1";
            sha256 =
              "2b61f83871fabe042f86170e15d3f7443d1f3e0840c716e0babbfe37cda914db";
          }
        ];
        vscodium-with-extensions = pkgs.vscode-with-extensions.override {
          vscode = pkgs.vscodium;
          vscodeExtensions = extensions;
        };

      in {
        defaultPackage = pkgs.nxpkgr;

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs;
            [
              rust-stable
              rust-analyzer
              clang
              pkg-config
              cargo-whatfeatures
            ] ++ [ vscodium-with-extensions ];

          shellHook = ''
            alias code="${vscodium-with-extensions}/bin/codium"
          '';
          RA_PATH = "${pkgs.rust-analyzer}/bin/rust-analyzer";
          RUST_SRC_PATH =
            "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
        };
      });
}