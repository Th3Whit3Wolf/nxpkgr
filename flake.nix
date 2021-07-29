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
  outputs = { self, utils, rust-overlay, nixpkgs, naersk, flake-compat, ... }:
  utils.lib.eachDefaultSystem (system: let
      overlays = [ 
        rust-overlay.overlay
        (self: super: 
        let
          rust-stable = pkgs.rust-bin.stable.latest.default.override {
            extensions =
              [ "cargo" "clippy" "rust-docs" "rust-src" "rust-std" "rustc" "rustfmt" ];
          };
        in
        {
          rustc = rust-stable;
          cargo = rust-stable;
        }) 
      ];
      pkgs = import nixpkgs {
          inherit system overlays;
        };
      /*
      naersk-lib = (naersk.lib."${system}".override {
          cargo = rust-stable;
          rustc = rust-stable;
        });
        */
    in rec {
      # `nix build`
      packages.vscodeExtensionSettings = naersk.buildPackage {
        pname = "vscodeExtensionSettings";
        root = ./.;
      };
      defaultPackage = packages.vscodeExtensionSettings;

      # `nix run`
      apps.vscodeExtensionSettings = utils.lib.mkApp {
        drv = packages.vscodeExtensionSettings;
      };
      defaultApp = apps.vscodeExtensionSettings;

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
      sha256 = "da54d2a40b72bb814b2e4af6b03eff6b3982162ae6f4492e6ceccad8f70cc7d3";
    }
    {
      name = "search-crates-io";
      publisher = "belfz";
      version = "1.2.1";
      sha256 = "2b61f83871fabe042f86170e15d3f7443d1f3e0840c716e0babbfe37cda914db";
    }
  ];
      vscodium-with-extensions = pkgs.vscode-with-extensions.override {
        vscode = pkgs.vscodium;
	vscodeExtensions = extensions;
      };
  
      devShell = pkgs.mkShell {
        nativeBuildInputs =  with pkgs; [
          rustc
          cargo
          clippy
          rustfmt
          rust-analyzer
          gcc
          gtk3
          pkg-config
          cargo-whatfeatures
          gcc
	] ++ [
	  vscodium-with-extensions  
        ];

	shellHook = ''
	  alias code="${vscodium-with-extensions}/bin/codium"
	'';
        
        RA_PATH = "${pkgs.rust-analyzer}/bin/rust-analyzer";
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
      };
    });
}
