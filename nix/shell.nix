{ pkgs }:
with pkgs;
mkShell {
  buildInputs = [ ]
    ++ lib.optionals stdenv.isDarwin [ ];

  nativeBuildInputs = [
    (rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" ];
    })
    cargo-criterion
    cargo-edit
    gnuplot
  ];
}
