{ pkgs ? import <nixpkgs> { }, lib ? pkgs.lib }:
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    pkg-config
    openssl
    pango
    xorg.libX11
    xorg.libX11.dev
    xorg.libXi.dev
    xorg.libXi
    mesa
    alsa-lib
    libxkbcommon
    zlib
  ];

  LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
    "${pkgs.libxkbcommon}/lib"
    "${pkgs.xorg.libX11}/lib"
    "${pkgs.xorg.libXi}/lib"
    "${pkgs.libGL}/lib"
  ];
}
