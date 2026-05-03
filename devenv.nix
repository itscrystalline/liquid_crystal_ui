{
  pkgs,
  lib,
  inputs,
  ...
}: {
  languages.rust = {
    enable = true;
    channel = "nightly";
    version = "2026-04-16";
  };
  # https://devenv.sh/packages/
  packages = with pkgs; [
    cargo-generate # generate rust projects from github templates
    cargo-udeps # find unused dependencies in Cargo.toml
    bacon
  ];
}
