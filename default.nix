{ lib, rustPlatform, ... }:

let cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;

in rustPlatform.buildRustPackage {
  pname = cargoToml.name;
  version = cargoToml.version;

  src = ./.;

  cargoHash = "sha256-FDVvZYJmYtSU8UAhGm02QgpVBDkKplA70TohoWXCtZ0=";

  env = { VERGEN_IDEMPOTENT = true; };

  meta = {
    description = "Generate new Factor! puzzles";
    homepage = "https://factor.samasaur.com";
  };
}
