{

  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.url = "nixpkgs/release-21.05";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
    devshell.url = "github:numtide/devshell";
  };

  outputs = { self, utils, rust-overlay, devshell, nixpkgs, naersk, flake-compat, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        inherit (naersk.lib.${system}) buildPackage;
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlay rust-overlay.overlay ];
        };
        rust-stable = pkgs.rust-bin.stable."1.54.0".default.override {
            extensions = [
              "cargo"
              "clippy"
              "rust-docs"
              "rust-src"
              "rust-std"
              "rustc"
              "rustfmt"
            ];
          };

          naersk-lib = (naersk.lib."${system}".override {
            cargo = rust-stable;
            rustc = rust-stable;
          }); 

        codeSettings = pkgs.writeScriptBin "init_vscodeSettings.sh" ''
#!${pkgs.stdenv.shell}



if [ ! -f "''${DEVSHELL_ROOT}/.vscode/settings.json" ]; then
  if [ ! -d "''${DEVSHELL_ROOT}/.vscode" ]; then
    mkdir "''${DEVSHELL_ROOT}/.vscode"
  fi
  
cat <<EOF > $DEVSHELL_ROOT/.vscode/settings.json
{
    "rust-analyzer.trace.extension": true,
    "rust-analyzer.trace.server": "messages",
    "rust-analyzer.server.path": "${pkgs.rust-analyzer}/bin/rust-analyzer",
    "terminal.integrated.profiles.linux": {
        "bash": {
            "path": "bash"
        },
        "zsh": {
            "path": "zsh"
        },
        "nix" : {
            "path": "nix-shell"
        }
    },
    "terminal.integrated.defaultProfile.linux": "nix",
    "editor.insertSpaces": false,
}
EOF

else
  line=$(grep '"rust-analyzer.server.path"' $DEVSHELL_ROOT/.vscode/settings.json)
  nline='"rust-analyzer.server.path": "${pkgs.rust-analyzer}/bin/rust-analyzer",'
  sed -i "s|$line|$nline|g" $DEVSHELL_ROOT/.vscode/settings.json
fi
'';

/*
ADD
https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens
*/

          extensions = (with pkgs.vscode-extensions; [
          bbenoist.Nix
          matklad.rust-analyzer
          tamasfe.even-better-toml
          pkief.material-icon-theme
        ]) ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [
          {
            name = "spacemacs";
            publisher = "cometeer";
            version = "1.1.1";
            sha256 =
              "da54d2a40b72bb814b2e4af6b03eff6b3982162ae6f4492e6ceccad8f70cc7d3";
          }
          {
            name = "search-crates-io";
            publisher = "belfz";
            version = "1.2.1";
            sha256 =
              "2b61f83871fabe042f86170e15d3f7443d1f3e0840c716e0babbfe37cda914db";
          }
        ];
        vscodium-with-extensions = pkgs.vscode-with-extensions.override {
          vscode = pkgs.vscodium;
          vscodeExtensions = extensions;
        };
      in
      {

        # nix build
        defaultPackage = buildPackage {
          pname = "nxpkgr";
          nativeBuildInputs = with pkgs; [ pkg-config ];
          root = ./.;
        };


        # nix develop
        devShell = pkgs.devshell.mkShell {
          name = "nxpkgr";
          bash.interactive = ''
bash -c ${codeSettings}/bin/init_vscodeSettings.sh
temp_dir=$(mktemp -d)
cat <<'EOF' >"$temp_dir/.zshrc"
source ${pkgs.zsh-autosuggestions}/share/zsh-autosuggestions/zsh-autosuggestions.zsh
if [[ $TERM != "dumb" && (-z $INSIDE_EMACS || $INSIDE_EMACS == "vterm") ]]; then
  eval "$(${pkgs.starship}/bin/starship init zsh)"
fi
eval "$(${pkgs.direnv}/bin/direnv hook zsh)"

zstyle ':completion:*:*:*:*:*' menu select
zstyle ':completion:*:matches' group 'yes'
zstyle ':completion:*:options' description 'yes'
zstyle ':completion:*:options' auto-description '%d'
zstyle ':completion:*:corrections' format ' %F{green}-- %d (errors: %e) --%f'
zstyle ':completion:*:descriptions' format ' %F{yellow}-- %d --%f'
zstyle ':completion:*:messages' format ' %F{purple} -- %d --%f'
zstyle ':completion:*:warnings' format ' %F{red}-- no matches found --%f'
zstyle ':completion:*:default' list-prompt '%S%M matches%s'
zstyle ':completion:*' format ' %F{yellow}-- %d --%f'
zstyle ':completion:*' group-name '\'
zstyle ':completion:*' verbose yes
zstyle ':completion::complete:*' cache-path "$XDG_CACHE_HOME/zsh/zcompcache"
zstyle ':completion::complete:*' use-cache on
zstyle ':completion:*' list-colors ''${(s.:.)LS_COLORS}
zstyle ':completion:*:*:kill:*:processes' list-colors '=(#b) #([0-9]#) ([0-9a-z-]#)*=01;34=0=01'
zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}' 'r:|[._-]=* r:|=*' 'l:|=* r:|=*'
zstyle ':completion:*:functions' ignored-patterns '(_*|pre(cmd|exec))'
zstyle ':completion:*' rehash true
zmodload zsh/complist

alias ..='cd ..'
alias 000='chmod -R 000'
alias 644='chmod -R 644'
alias 666='chmod -R 666'
alias 755='chmod -R 755'
alias 777='chmod -R 777'
alias ali='alias | bat --style=numbers,grid -l cpp'
alias c='clear'
alias countfiles='fd -t f | wc -l'
alias f='fd . | grep '
alias folders='du -h --max-depth=1'
alias folderssort='fd . -d 1 -t d -print0 | xargs -0 du -sk | sort -rn'
alias gt='cd $(fd -H -t d -j $(nproc) | sk )'
alias h='history | grep '
alias l='exa --icons'
alias la='exa --all --icons'
alias ll='exa --long --header --git --icons'
alias ls='exa --icons'
alias lsa='exa --all --icons'
alias lsal='exa --long --all --header --git --icons'
alias lsl='exa --long --header --git --icons'
alias lsla='exa --long --all --header --git --icons'
alias mem='free -h --si'
alias sl='exa --icons'
alias sla='exa --all --icons'
alias slal='exa --long --all --header --git --icons'
alias sll='exa --long --header --git --icons'
alias slla='exa --long --all --header --git --icons'
alias topcpu='ps -eo pcpu,pid,user,args | sort -k 1 -r | head -10'
alias tree='tree -CAhF --dirsfirst'
alias treed='tree -CAFd'
alias tst='hyperfine'
alias tstc='hyperfine --prepare "sync; echo 3 | sudo tee /proc/sys/vm/drop_caches"'
alias tstw='hyperfine -w 10'
alias unbz2='tar -xvjf'
alias ungz='tar -xvzf'
alias untar='tar -xvf'
alias wget='wget --hsts-file="$XDG_CACHE_HOME/wget-hsts"'
alias x='chmod +x'
EOF

menu

ZDOTDIR=$temp_dir zsh -i 
exit
rm -dR $temp_dir
'';

          # Custom scripts. Also easy to use them in CI/CD
          commands = [
            {
              name = "build";
              help = "Build nxpkgr";
              command = "cargo build";
            }
            {
              name = "code";
              help = "Open vscodium";
              command = "${vscodium-with-extensions}/bin/codium $DEVSHELL_ROOT";
            }
            {
              name = "fmt";
              help = "Check formatting formatting";
              command = "nixpkgs-fmt \${@} $DEVSHELL_ROOT && cargo fmt";
            }
            {
              name = "run";
              help = "Run nxpkgr";
              command = "cargo run";
            }
            {
              name = "test";
              help = "Run test for nxpkgr";
              command = "cargo test";
            }
          ];

          packages = with pkgs;[ nixpkgs-fmt rust-stable rust-analyzer stdenv.cc pkg-config cargo-whatfeatures neofetch bc starship direnv ] ++ [ vscodium-with-extensions ];
          env = [
            {name = "RUST_SRC_PATH"; eval = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";}
          ];
        };
      });
}
