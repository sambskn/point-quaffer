{ pkgs, lib, config, inputs, ... }:
{
  # https://devenv.sh/basics/
  env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  env.BINDGEN_EXTRA_CLANG_ARGS = builtins.concatStringsSep " " [
    ''-I"${pkgs.glibc.dev}/include"''
    ''-I"${pkgs.clang_18}/resource-root/include"''
  ];
  env.RUST_BACKTRACE = 1;
  
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

  # download kansas laz
  scripts.data.exec = ''
    curl -o test_kansas.laz https://rockyweb.usgs.gov/vdelivery/Datasets/Staged/Elevation/LPC/Projects/KS_Statewide_2018_A18/KS_Statewide_B2_2018/LAZ/USGS_LPC_KS_Statewide_2018_A18_14S_KH_5005.laz
  '';
  
  # https://devenv.sh/basics/
  enterShell = ''
    # here you can run a shell script on boot of the shell
    echo 'ayo this is a dev shell for point quaffer, got ya rust/gdal/helix/git/pqrs ready to go bub'
  '';
  
  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';
}
