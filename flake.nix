{
  inputs = {
    cargo.url = "github:yusdacra/nix-cargo-integration";
  };
  outputs = inputs:
    let
      appNativeBuildInputs = pkgs: with pkgs; [
        pkg-config
      ];
      appBuildInputs = pkgs: (appRuntimeInputs pkgs) ++ (with pkgs; [
        udev
        alsaLib
        x11
        vulkan-tools
        vulkan-headers
        vulkan-validation-layers
      ]);
      appRuntimeInputs = pkgs: with pkgs; [
        vulkan-loader
        xlibs.libXcursor
        xlibs.libXi
        xlibs.libXrandr
      ];
      shellInputs = pkgs: with pkgs; [ xorg.libX11 xorg.libX11.dev libGL glxinfo ] ++ appNativeBuildInputs pkgs ++ appBuildInputs pkgs ++ appRuntimeInputs pkgs;
    in
    inputs.cargo.lib.makeOutputs {
      root = ./.;

      overrides = {
        common = prev: {
          runtimeLibs = prev.runtimeLibs ++ (appRuntimeInputs prev.pkgs);
          buildInputs = prev.buildInputs ++ (appBuildInputs prev.pkgs);
          nativeBuildInputs = prev.nativeBuildInputs ++ (appNativeBuildInputs prev.pkgs);
        };

        shell = common: prev: {
          packages = prev.packages ++ (shellInputs common.pkgs);
          # env      = prev.env      ++ [ { name = "LD_LIBRARY_PATH"; eval = "${common.pkgs.lib.makeLibraryPath (shellInputs common.pkgs)}"; } ];
        };
      };
    };
}
