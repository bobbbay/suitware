{
  inputs = {
    cargo.url = "github:yusdacra/nix-cargo-integration";
  };
  outputs = inputs: inputs.cargo.lib.makeOutputs { root = ./.; };
}
