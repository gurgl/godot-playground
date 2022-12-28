let
tmp = import ./nix/thing.nix {};
custPkgs = tmp.pkgs;
nixgl = tmp.nixgl;

rust-toolchain = custPkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [custPkgs.rustc custPkgs.cargo custPkgs.rustPlatform.rustcSrc custPkgs.rust-analyzer];
    };
in
    
    
    # Configure the dependency of your shell
    # Add support for clang for bindgen in godot-rust
    custPkgs.mkShell.override { stdenv = custPkgs.clangStdenv; } {
        buildInputs = [
            # Rust related dependencies
            rust-toolchain
            custPkgs.rustc
            #custPkgs.cargo
            #custPkgs.rustfmt
            custPkgs.libclang
            custPkgs.alsaPlugins
            # custPkgs.alsaPluginWrapper
            # custPkgs.alsaLib
            # custPkgs.alsaUtils
            # custPkgs.alsaTools
            # # custPkgs.apulse
            # custPkgs.libpulseaudio

            # custPkgs.rust.packages.stable.gdnative
            # Godot Engine Editor
            custPkgs.godot
            # The support for OpenGL in Nix
            nixgl.auto.nixGLDefault
        ];
        nativeBuildInputs = [ custPkgs.pkgconfig ];


        ALSA_PLUGIN_DIR = "${custPkgs.alsaPlugins}/lib/alsa-lib";
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

        RUST_TOOLCHAIN = "${rust-toolchain}";

        # Alias the godot engine to use nixGL
        shellHook = ''
            alias godot="nixGL godot -e"
        '';
    }

