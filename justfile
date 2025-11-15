set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

_default:
  @just --list -u


init:
  cargo install cargo-binstall
  cargo binstall cargo-insta cargo-shear -y
  corepack enable
  pnpm install
  
fmt: 
  pnpm fmt
  cargo fmt --all -- --emit=files

fix:
  just fmt
  cargo fix --allow-dirty --allow-staged
  -cargo shear --fix
  pnpm lint --fix

update:
  cargo update
  pnpm deps

test: 
  pnpm test run
  cargo test --all-features --workspace

ready:
  git diff --exit-code --quiet
  just lint
  just fix
  just test
  git status
  git diff --exit-code --quiet

lint: 
  cargo shear
  cargo clippy --workspace --all-targets --all-features
  pnpm lint

build:
  cargo build
  pnpm build
