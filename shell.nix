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
  ];

  shellHook = ''
    # Permet au compilateur Rust de trouver OpenSSL via pkg-config
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
    
    echo "==========================================="
    echo "🚀 Bienvenue dans le Shell Nix de MyStats !"
    echo "==========================================="
    echo "Outils chargés et prêts à l'emploi :"
    echo " - Rust    : $(cargo --version)"
    echo " - Node.js : $(node --version)"
    echo " - SQLx    : install"
    echo " - Runner  : $(just --version | head -n 1)"
    echo "==========================================="
  '';
}
