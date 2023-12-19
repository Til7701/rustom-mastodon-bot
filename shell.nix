{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      rustc
      cargo
      gcc
      rustfmt
      clippy
      openssl
      pkg-config
      llvmPackages_latest.bintools
    ];

    OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";

    RUST_BACKTRACE = 1;

  }