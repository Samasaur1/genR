{ lib, rustPlatform, stdenv, darwin }:

let cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;

in rustPlatform.buildRustPackage rec {
  pname = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.name;
  version =
    (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

  src = ./.;

  useFetchCargoVendor = true;
  cargoHash = "sha256-FDVvZYJmYtSU8UAhGm02QgpVBDkKplA70TohoWXCtZ0=";

  buildInputs = [ ] ++ lib.optionals stdenv.isDarwin [ ];

  env = { VERGEN_IDEMPOTENT = true; };

  meta = with lib; {
    description = "Generate new Factor! puzzles";
    homepage = "https://factor.samasaur.com";
    license = with licenses; [ ];
    maintainers = with maintainers; [ ];
  };
}
