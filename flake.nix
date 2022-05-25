{
  description = "Badger2040 project";

  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils = {
      url = "github:numtide/flake-utils";
    };
    mozillapkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, utils, naersk, mozillapkgs }:
    utils.lib.eachDefaultSystem (system:
      let
        mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") { };
        rust = (mozilla.rustChannelOf {
          date = "2022-05-25"; # get the current date with `date -I`
          channel = "nightly";
          sha256 = "sha256-zjx9Ogl5ZyJOWq/1byndSStGQiIzmw0NamzmVGmUZbY==";
        }).rust.override {
          targets = [ "thumbv6m-none-eabi" ];
        };

        # Override the version used in naersk
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      rec {
        # `nix build`
        packages.flip-link = naersk-lib.buildPackage rec {
          pname = "flip-link";
          version = "0.1.5";
          src = pkgs.fetchFromGitHub {
            owner = "knurling-rs";
            repo = pname;
            rev = "v${version}";
            sha256 = "sha256-7o4B8vKia1b6Joo6k2PLG8DCclkdEd15PvX9aMPYbDE=";
          };
        };
        packages.elf2uf2-rs = naersk-lib.buildPackage rec {
          pname = "elf2uf2-rs";
          version = "1.3.7";
          src = pkgs.fetchFromGitHub {
            owner = "JoNil";
            repo = pname;
            rev = "b861f6b3c9540bcb27e88ec496e09763e590dc76";
            sha256 = "sha256-rjh2B1fMwyixkTIeci6/zc66PlguEzpLnCAM8CVZ3ug=";
          };
          nativeBuildInputs = with pkgs; [
            pkg-config
            udev
          ];
        };
        # defaultPackage = packages.my-project;

        # `nix run`
        # apps.my-project = utils.lib.mkApp {
        #   drv = packages.my-project;
        # };
        # defaultApp = apps.my-project;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.pre-commit
            rust
            packages.flip-link
            packages.elf2uf2-rs
          ];
        };
      });
}
