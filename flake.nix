{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs =
    {
      self,
      nixpkgs,
    }:

    let
      pkgsForSystem =
        system:
        import nixpkgs {
          inherit system;
          config = {
            allowUnfree = true;
          };
        };
    in
    {
      devShells = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
        system:
        let
          pkgs = pkgsForSystem system;
        in
        with pkgs;
        {
          default = mkShell rec {
            nativeBuildInputs = [ ];
            buildInputs = [
              cargo
              rustc
              cmake
              clang
              gcc
              ninja
              libcxx
              SDL2
              alsa-lib.dev
              code-cursor
              pkg-config
              doxygen
              graphviz
              systemdLibs.dev
              udev
              vulkan-loader
              wayland
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              libxkbcommon
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

            shellHook = ''
              export PKG_CONFIG_PATH=${pkgs.alsa-lib.dev}/lib/pkgconfig:${pkgs.systemdLibs.dev}/lib/pkgconfig:$PKG_CONFIG_PATH
              export CC=${pkgs.gcc}/bin/gcc
            '';
          };
        }
      );
    };
}
