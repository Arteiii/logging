{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.clippy
    pkgs.protobuf
    pkgs.ghz # for testing
  ];

  # set environment
  shellHook = ''
  '';
}
