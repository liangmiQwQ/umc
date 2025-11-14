set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

_default:
  @just --list -u


init:
  cargo install cargo-binstall
  cargo binstall cargo-insta cargo-shear -y
  corepack enable
  pnpm install
  
fix:
  cargo fmt --all -- --emit=files
  cargo fix --allow-dirty --allow-staged
  -cargo shear --fix
  pnpm fmt
  pnpm lint --fix

update:
  cargo update
  pnpm deps

test: 
  pnpm test run
  cargo test --all-features --workspace

ready:
  # git diff --exit-code --quiet
  cargo clippy --workspace --all-targets --all-features
  just fix
  just test
  git status
