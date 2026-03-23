{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Environnement Rust
    cargo
    rustc
    clippy
    rustfmt

    # Dépendances système pour compiler certains paquets Rust (reqwest, sqlx...)
    pkg-config
    openssl

    # Environnement Frontend (Vue.js)
    nodejs

    # Outils Base de données
    postgresql
    sqlx-cli

    # Command Runner
    just

    # DevSecOps
    pre-commit
    gitleaks
    eslint
    cargo-audit
    cargo-deny
  ];

  shellHook = ''
    # Permet au compilateur Rust de trouver OpenSSL via pkg-config
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
    # Permet à l'exécutable compilé de trouver libssl.so au runtime
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.openssl ]}:$LD_LIBRARY_PATH"

    echo "==========================================="
    echo "🚀 Bienvenue dans le Shell Nix de MyStats !"
    echo "==========================================="
    echo "Outils chargés et prêts à l'emploi :"
    echo " - Rust    : $(cargo --version)"
    echo " - Node.js : $(node --version)"
    echo " - SQLx    : install"
    echo " - Runner  : $(just --version | head -n 1)"
    echo "==========================================="

    # Installe automatiquement les hooks pre-commit pour le repository local
    pre-commit install > /dev/null 2>&1

    just --list
  '';
}
