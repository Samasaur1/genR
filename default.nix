{ lib, rustPlatform, stdenv, darwin }:

let cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;

in rustPlatform.buildRustPackage rec {
  pname = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.name;
  version =
    (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

  src = ./.;

  cargoHash = "sha256-kpOVzA1Poe+yMzcRLJvzbHurrg+E4osQwbOFkNbaY60=";

  buildInputs = [ ] ++ lib.optionals stdenv.isDarwin [ ];

  env = { VERGEN_IDEMPOTENT = true; };

  meta = with lib; {
    description = "Generate new Factor! puzzles";
    homepage = "https://factor.samasaur.com";
    license = with licenses; [ ];
    maintainers = with maintainers; [ ];
  };
}
