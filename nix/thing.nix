self :
let
  # Get an up-to-date package for enabling OpenGL support in Nix
  _nixgl = import (fetchTarball "https://github.com/guibou/nixGL/archive/master.tar.gz") {};

  # Pin the version of the nix package repository that has Godot 3.2.3 and compatible with godot-rust 0.9.3
  # You might want to update the commit hash into the one that have your desired version of Godot
  # You could search for the commit hash of a particular package by using this website https://lazamar.co.uk/nix-versions
  # OLD pkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/archive/5658fadedb748cb0bdbcb569a53bd6065a5704a9.tar.gz") {};

  # pkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/archive/b05d2077ebe219f6a47825767f8bab5c6211d200.tar.gz") {};



  pkgs = import ./nixpkgs.nix {
      overlays = [
                (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
                ];
    };

  libs = with pkgs; [
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXinerama
          xorg.libXrandr
          xorg.libXrender
          xorg.libXi
          xorg.libXext
          xorg.libXfixes
          udev
          systemd
          systemd.dev
          libpulseaudio
          freetype
          openssl
          alsa-lib
          libGLU
          zlib
          yasm
        ];

  # buildTools = with pkgs; [ scons pkg-config autoPatchelfHook bashInteractive patchelf gcc clang];
  buildTools = with pkgs; [ pkg-config autoPatchelfHook bashInteractive patchelf gcc clang];

  gdnative = pkgs.rustPlatform.buildRustPackage rec {
        pname = "gdnative";
        version = "0.10.1";

        src = pkgs.fetchFromGitHub {
          owner = "godot-rust";
          repo = pname;
          # hash = "sha256-wA6i8IUhY9pABuQze8T2CQSnNjp42xLWzCNqnHzNoi8=";
          # sha256 = "1a9inv2mx6arfh97x5z1ap82hqiszli5y7xdwrc594frkjwmc0ng";
          sha256 = "wA6i8IUhY9pABuQze8T2CQSnNjp42xLWzCNqnHzNoi8=";
          # sha256 = pkgs.lib.fakeSha256;
          rev = version;
          # rev = "ee1ceb28341e78e0bcb20b6969efaa49a254c40a";
        };
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        BINDGEN_EXTRA_CLANG_ARGS = with pkgs; ''
                  -isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.getVersion clang}/include
                  -isystem ${llvmPackages.libclang.out}/lib/clang/${lib.getVersion clang}/include
                  -isystem ${glibc.dev}/include
                '';

        # For Rust language server and rust-analyzer
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        RUST_BACKTRACE=1;
        checkFlags="--skip ui_tests";
        # cargoSha256 = pkgs.lib.fakeSha256;
        # cargoSha256 = "sha256-wA6i8IUhY9pABuQze8T2CQSnNjp42xLWzCNqnHzNoi8=";
        cargoHash = pkgs.lib.fakeHash;
        # cargoHash = "sha256-zwJWuZzZkVRY5q0fXyL9OmIo0FXhl34SdFmZXsW2Mak=";
        #cargoPatches = [
        #    # a patch file to add/update Cargo.lock in the source code
        #    ./add-Cargo.lock.patch
        #  ];
        postPatch = ''
          cp ${./../GDNative.Cargo.lock} Cargo.lock
        '';
        cargoLock = {
            lockFile = ./../GDNative.Cargo.lock;
          };

        nativeBuildInputs = buildTools ++ libs;
        buildInputs = libs ++ (with pkgs;[
          pkgs.cargo
          pkgs.rustc
          pkgs.libclang
          _nixgl.auto.nixGLDefault
        ]);
        # enableParallelBuilding = true;
  };

  custPkgs = pkgs // {
    rust = pkgs.rust // {
      packages = pkgs.rust.packages // {
        stable = pkgs.rust.packages.stable // {
          gdnative = gdnative;
        };
      };
    };
  };
in rec {
  pkgs = custPkgs;
  nixgl = _nixgl;
  }