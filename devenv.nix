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
    ## general stuff to have for dev work
    pkgs.git # i love commiting
    pkgs.helix # real gamer ide
    pkgs.marksman # language server for markdown
    pkgs.eza # modern ls
    pkgs.trunk # for doing wasm builds/local serving
    pkgs.taplo # toml formatter / linter
    ## gis
    pkgs.gdal # the big dog - nixpkgs grabs 3.11 at time of writing
    # gdal-sys/bindgen helpers
    #   ( because rust 'gdal' crate is linked to v3.4 out of the box
    #   so we need a bunch of c related deps to do bindgen for our gdal version )
    pkgs.lldb 
    pkgs.libclang 
    pkgs.gcc 
    pkgs.glibc.dev 
    pkgs.stdenv.cc.cc.lib
    pkgs.pqrs # cli for auditing .parquet files
  ];
  # get the real gamer command prompt
  starship.enable = true;
  
  # https://devenv.sh/languages/
  languages.rust.enable = true;
  languages.rust.channel = "nightly";
  languages.rust.targets = ["wasm32-unknown-unknown"];
  
  # https://devenv.sh/scripts/
  # Runs Bevy viz WASM app hosted through trunk locally
  scripts.viz.exec = ''
    # (fyi trunk serve also does a build/watch)
    trunk serve
  '';
  # do a cargo check on the wasm build
  scripts.check_viz.exec = ''
    cargo check --no-default-features --features wasm_viz --target wasm32-unknown-unknown
  '';
  # download test data from USGS of some area in kansas to `test_kansas.laz` (~317M)
  scripts.test_data_kansas.exec = ''
    curl -o test_kansas.laz https://rockyweb.usgs.gov/vdelivery/Datasets/Staged/Elevation/LPC/Projects/KS_Statewide_2018_A18/KS_Statewide_B2_2018/LAZ/USGS_LPC_KS_Statewide_2018_A18_14S_KH_5005.laz
  '';
  
  # https://devenv.sh/basics/
  enterShell = ''
    # here you can run a shell script on boot of the shell
    # for now we just print some fun stuff to the console
    cat dev_env_start_msg.txt
  '';
  
  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests (just the default one, these dont really test anything rn)"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';
}
