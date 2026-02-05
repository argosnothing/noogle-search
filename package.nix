{
  lib,
  rustPlatform,
  fetchFromGitHub,
  pkg-config,
  openssl,
  makeWrapper,
  bat,
  fzf,
  xdg-utils,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "noogle-search";
  version = "0.2.0";

  src = ./.;
  cargoHash = "sha256-axqFE5ZEiVP8PzFTtW5mbyyYcR4q9g3LX/0i6y+cgy8=";

  nativeBuildInputs = [
    makeWrapper
    pkg-config
  ];

  buildInputs = [
    openssl.dev
  ];

  postInstall = ''
    wrapProgram $out/bin/noogle-search \
      --prefix PATH : ${lib.makeBinPath [bat fzf xdg-utils]}
  '';

  meta = {
    description = "Search Noogle functions with fzf";
    homepage = "https://github.com/argosnothing/noogle-search";
    license = lib.licenses.gpl3Plus;
    mainProgram = "noogle-search";
    maintainers = with lib.maintainers; [argos_nothing];
  };
})
