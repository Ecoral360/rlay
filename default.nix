{ pkgs ? import <nixpkgs> { }, lib ? pkgs.lib }:
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    cmake
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
    xorg.libXrandr
    xorg.libXcursor
    xorg.libXinerama
    xorg.libXext
    glfw
    raylib
    libGL
    wayland
  ];

  X11_X11_INCLUDE_PATH = "${pkgs.xorg.libX11}/lib";
  X11_X11_LIB = "${pkgs.xorg.libX11}/lib";

  # RUST_ANALYZER_CARGO_FEATURES = "raylib";

  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
    libGL
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXi
    libxkbcommon
  ];
  LIBCLANG_PATH = "${pkgs.llvmPackages_16.libclang.lib}/lib";

  # LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
  #   "${pkgs.libxkbcommon}/lib"
  #   "${pkgs.xorg.libX11}/lib"
  #   "${pkgs.xorg.libXi}/lib"
  #   "${pkgs.libGL}/lib"
  # ];

  shellHook = ''
    # export X11_X11_INCLUDE_PATH=${pkgs.xorg.libX11}/lib
    # export X11_X11_LIB=${pkgs.xorg.libX11}/lib
    # export CMAKE_PREFIX_PATH=${pkgs.raylib}:${pkgs.glfw}:${pkgs.xorg.libX11}:${pkgs.xorg.libXi}:${pkgs.xorg.libXrandr}:${pkgs.xorg.libXcursor}:${pkgs.xorg.libXinerama}:${pkgs.mesa}
    export RAYLIB_SYS_NO_BUILD=1
  '';
}
