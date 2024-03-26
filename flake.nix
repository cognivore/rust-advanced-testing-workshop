{
    inputs = {
        fenix = {
            url = "github:nix-community/fenix";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        nixpkgs.url = "github:NixOS/nixpkgs";
    };

    outputs = {self, fenix, nixpkgs}:
        let pkgs = nixpkgs.legacyPackages.x86_64-linux;
            rust_complete = (fenix.outputs.packages.x86_64-linux.complete.withComponents [
                        "cargo"
                        "clippy"
                        "rustc"
                        "rustfmt"
                        "rust-src"
                    ]);
            npkgs = pkgs.nodePackages;

        in {
            defaultPackage.x86_64-linux = pkgs.hello;

            devShell.x86_64-linux =
                pkgs.mkShell {
                    buildInputs = [
                        rust_complete

                        pkgs.git

                        pkgs.httpie
                        pkgs.openssl
                        pkgs.jq
                        pkgs.yq
                        pkgs.dig
                        pkgs.shellcheck

                        pkgs.nodejs-18_x
                        pkgs.expect
                        npkgs.typescript
                        npkgs.node-gyp
                        npkgs.serve
                    ];
                    shellHook = ''
                      echo "Entering devShell";
                      export LD_LIBRARY_PATH=${pkgs.lib.strings.makeLibraryPath [ pkgs.openssl ]};
                      export OPENSSL_DIR=${pkgs.lib.getDev pkgs.openssl};
                      export OPENSSL_LIB_DIR=${pkgs.lib.getLib pkgs.openssl}/lib;
                    '';
                };
        };
}
