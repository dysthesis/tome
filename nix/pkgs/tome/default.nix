{
  craneLib,
  cargoArtifacts,
  commonArgs,
  ...
}:
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;
    pname = "tome";
    CARGO_PROFILE = "release";
  }
)
