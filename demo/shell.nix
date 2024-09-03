# You can use this file to set up all the dependencies
# for this project. Run `nix-shell`.
{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    z3
  ];
  
  FLUX_CHECK_OVERFLOW=1; # need overflow checking for `abs`
  shellHook = ''
    PATH=/home/mod/.local/bin:$PATH
  '';
}
