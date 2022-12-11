let
tmp = import ./nix/thing.nix {};
custPkgs = tmp.pkgs;
nixgl = tmp.nixgl;
in
    
    
    # Configure the dependency of your shell
    # Add support for clang for bindgen in godot-rust
    custPkgs.mkShell.override { stdenv = custPkgs.clangStdenv; } {
        buildInputs = [
            # Rust related dependencies
            custPkgs.rustc
            custPkgs.cargo
            custPkgs.rustfmt
            custPkgs.libclang

            custPkgs.rust.packages.stable.gdnative
            # Godot Engine Editor
            custPkgs.godot
            # The support for OpenGL in Nix
            nixgl.auto.nixGLDefault
        ];

        # Point bindgen to where the clang library would be
        LIBCLANG_PATH = "${custPkgs.libclang.lib}/lib";
        # Make clang aware of a few headers (stdbool.h, wchar.h)
        BINDGEN_EXTRA_CLANG_ARGS = with custPkgs; ''
          -isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.getVersion clang}/include
          -isystem ${llvmPackages.libclang.out}/lib/clang/${lib.getVersion clang}/include
          -isystem ${glibc.dev}/include
        '';

        # For Rust language server and rust-analyzer
        RUST_SRC_PATH = "${custPkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        # Alias the godot engine to use nixGL
        shellHook = ''
            alias godot="nixGL godot -e"
        '';
    }

