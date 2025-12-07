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
      tome = callPackage ./tome {
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
