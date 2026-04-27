{
  pkgs,
  lib,
  inputs,
  ...
}: {
  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.95.0";
  };
  # https://devenv.sh/packages/
  packages = with pkgs; [
    cargo-generate # generate rust projects from github templates
    cargo-udeps # find unused dependencies in Cargo.toml
    bacon
  ];
}
