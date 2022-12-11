options: self: super: {

  optics-src = super.fetchFromGitHub {
    owner = "godot-rust";
    repo = "gdnative";
    sha256 = "1vlblxsha2667hxkmp845s9bz5dgc0f452xk049x7bcwc5g66fsp";
    rev = "v0.10.1";
    # rev = "ee1ceb28341e78e0bcb20b6969efaa49a254c40a";
  };

  servant-src = super.fetchFromGitHub {
    owner = "haskell-servant";
    repo = "servant";
    sha256 = "11mb9j0qldfqprwvbq20f40ray7bdqk21gl5d81jvya5bh29bzzj";
    rev = "a1a99552b58bf49f8f142be62eef8b6eae14b7c0";
  };

  servant-client-ghcjs = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghcjs86.callCabal2nix "servant-client-ghcjs"
      (self.servant-src + "/servant-client-ghcjs") { });

  optics-core = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghcjs86.callCabal2nix "optics-core"
      (self.optics-src + "/optics-core") {indexed-profunctors = self.indexed-profunctors; });
  optics-extra = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghcjs86.callCabal2nix "optics-extra"
      (self.optics-src + "/optics-extra") {indexed-profunctors = self.indexed-profunctors; optics-core = self.optics-core; });
  optics-th = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghcjs86.callCabal2nix "optics-th"
      (self.optics-src + "/optics-th") { optics-core = self.optics-core; });
  
  indexed-profunctors = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghcjs86.callCabal2nix "indexed-profunctors"
      (self.optics-src + "/indexed-profunctors") { });
  
  optics = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghcjs86.callCabal2nix "optics"
      (self.optics-src + "/optics") { optics-core = self.optics-core; optics-th = self.optics-th; optics-extra = self.optics-extra;   });

  platform-client = super.haskell.packages.ghcjs86.callCabal2nix "lf-platform-client" ../platform {
        servant-client-ghcjs = self.servant-client-ghcjs ;
        optics = self.optics;
        optics-core = self.optics-core;
   };

  soptics-core = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghc865.callCabal2nix "optics-core"
      (self.optics-src + "/optics-core") {indexed-profunctors = self.sindexed-profunctors; });
  soptics-extra = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghc865.callCabal2nix "optics-extra"
      (self.optics-src + "/optics-extra") {indexed-profunctors = self.sindexed-profunctors; optics-core = self.soptics-core; });
  soptics-th = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghc865.callCabal2nix "optics-th"
      (self.optics-src + "/optics-th") { optics-core = self.soptics-core; });
  
  sindexed-profunctors = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghc865.callCabal2nix "indexed-profunctors"
      (self.optics-src + "/indexed-profunctors") { });
  
  soptics = super.haskell.lib.doJailbreak
    (super.haskell.packages.ghc865.callCabal2nix "optics"
      (self.optics-src + "/optics") { optics-core = self.soptics-core; optics-th = self.soptics-th; optics-extra = self.soptics-extra; indexed-profunctors = self.sindexed-profunctors;   });
  
  platform-server = super.haskell.packages.ghc865.callCabal2nix "lf-platform-server" ../. {
    optics = self.soptics;
   };
}
