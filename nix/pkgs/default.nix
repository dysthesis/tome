{
  perSystem = {
    craneLib,
    pkgs,
    commonArgs,
    cargoArtifacts,
    ...
  }: let
    inherit (pkgs) callPackage;
  in {
    packages = rec {
      tome = callPackage ./package {
        inherit
          craneLib
          pkgs
          commonArgs
          cargoArtifacts
          ;
      };

      default = tome;
    };
  };
}
