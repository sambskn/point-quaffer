{ pkgs, lib, config, inputs, ... }:
{
  # https://devenv.sh/basics/
  env.GREET = "the quaff zone";
  env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  env.BINDGEN_EXTRA_CLANG_ARGS = builtins.concatStringsSep " " [
    ''-I"${pkgs.glibc.dev}/include"''
    ''-I"${pkgs.clang_18}/resource-root/include"''
  ];
  env.LD_LIBRARY_PATH = lib.makeLibraryPath [ 
    pkgs.llvm_18 
    pkgs.clang_18 
    pkgs.libclang.lib 
    pkgs.stdenv.cc.cc.lib 
  ];
  
  # https://devenv.sh/packages/
  packages = [ 
    pkgs.git 
    pkgs.gdal 
    pkgs.helix
    pkgs.lldb 
    pkgs.libclang 
    pkgs.gcc 
    pkgs.glibc.dev 
    pkgs.stdenv.cc.cc.lib
  ];
  
  # https://devenv.sh/languages/
  languages.rust.enable = true;
  
  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';
  
  # https://devenv.sh/basics/
  enterShell = ''
    hello         # Run scripts directly
    git --version # Use packages
  '';
  
  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';
}
