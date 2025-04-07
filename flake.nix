{
  description = "terms";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }: {
    devShells = flake-utils.lib.eachDefaultSystemPassThrough (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            allowUnfree = true;
          };
        };
      in {

        "${system}".default = pkgs.mkShell {
          packages = with pkgs;
            [
              (rust-bin.beta.latest.default.override {
                extensions = ["rust-src" "rust-std" "rust-analyzer" "clippy" "rustfmt"];
              })
              pkg-config
              glib.dev
              pango.dev
              cairo.dev
              graphene.dev
              gtk4.dev
              libxml2.dev
              libadwaita.dev
              vte-gtk4.dev
          ];

        };
      }
    );
  };
}
