{
  lib,
  rustPlatform,
  fetchFromGitHub,
  pkg-config,
  openssl,
  makeWrapper,
  bat,
  fzf,
}:
rustPlatform.buildRustPackage rec {
  pname = "noogle-search";
  version = "0.2.0";

  src = fetchFromGitHub {
    owner = "argosnothing";
    repo = "noogle-search";
    rev = "v${version}";
    hash = "sha256-dZQ8OEOYN4lBpP7e9f/+kaEOsV+G4t37KGdGoJ4IEAc=";
  };

  cargoHash = "sha256-Pb+/zgDCMNbt8N0BNE1F6+/TMzu3pfgUPrcnwJUEcEw=";

  nativeBuildInputs = [
    makeWrapper
    pkg-config
  ];

  buildInputs = [
    openssl.dev
  ];

  postInstall = ''
    wrapProgram $out/bin/noogle-search \
      --prefix PATH : ${lib.makeBinPath [bat fzf]}
  '';

  meta = {
    description = "Search Noogle functions with fzf";
    homepage = "https://github.com/argosnothing/noogle-search";
    license = lib.licenses.mit; # Update with actual license
    mainProgram = "noogle-search";
    maintainers = with lib.maintainers; []; # Add maintainer(s)
  };
}
