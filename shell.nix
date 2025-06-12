{ pkgs ? import <nixpkgs> {} }:

let
  openssl = pkgs.openssl;
in
pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.pkg-config
    openssl.dev  # headers
    openssl.out  # libraries
  ];

  shellHook = ''
    export OPENSSL_DIR=${openssl.dev}
    export OPENSSL_LIB_DIR=${openssl.out}/lib
    export OPENSSL_INCLUDE_DIR=${openssl.dev}/include
    export PKG_CONFIG_PATH="${openssl.dev}/lib/pkgconfig"
  '';
}
