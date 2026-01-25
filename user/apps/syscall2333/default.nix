{ stdenv, lib, pkgs ? null }:

stdenv.mkDerivation {
  pname = "syscall2333";
  version = "0.1.0";
  src = ./.;

  buildPhase = ''
    make
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp syscall2333 $out/bin/
  '';

  meta = with lib; {
    description = "DragonOS syscall 2333 user test app";
    platforms = platforms.linux;
  };
}
