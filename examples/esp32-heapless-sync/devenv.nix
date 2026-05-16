{
  pkgs,
  lib,
  inputs,
  ...
}: let
  inherit (pkgs.stdenv.hostPlatform) system;
in {
  # https://devenv.sh/basics/
  env = rec {
    LD_LIBRARY_PATH = lib.makeLibraryPath [
      pkgs.stdenv.cc.cc
      pkgs.libxml2
      pkgs.libz
    ];
    NIX_LD_LIBRARY_PATH = LD_LIBRARY_PATH;
    NIX_LD = pkgs.runCommand "ld.so" {} ''
      ln -s "$(cat '${pkgs.stdenv.cc}/nix-support/dynamic-linker')" $out
    '';
  };

  # https://devenv.sh/packages/
  packages = with pkgs; [
    cargo-generate # generate rust projects from github templates
    inputs.nixpkgs-upstream.legacyPackages.${system}.esp-generate
    inputs.my-nur.packages.${system}.esp-config
    cargo-udeps # find unused dependencies in Cargo.toml
    cargo-bloat
    ldproxy

    # required for esp development
    espup # tool for installing esp-rs toolchain
    rustup # rust installer, required by espup
    rust-analyzer
    bacon
    espflash # flash binary to esp
  ];

  enterShell = ''
    echo -e "\e[1mInstalling toolchains for esp"
    echo -e "-----------------------------\e[0m"
    espup install -b 1.90.0 --export-file $DEVENV_ROOT/esp-export.sh
    export PATH="~/.rustup/toolchains/esp/bin:$PATH"
    source $DEVENV_ROOT/esp-export.sh
  '';
}
