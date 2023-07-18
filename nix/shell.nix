{ pkgs }:
with pkgs;
mkShell {
  buildInputs = [ ]
    ++ lib.optionals stdenv.isDarwin [ ];

  nativeBuildInputs = [
    (rust-bin.stable.latest.default.override {
      extensions = [
        "rust-src"
        "rust-analysis"
        "clippy"
      ];
    })
    cargo-criterion
    cargo-edit
    cargo-watch
    gnuplot
  ];
}
