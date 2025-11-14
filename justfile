set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

_default:
  @just --list -u


init:
  cargo install cargo-binstall
  cargo binstall cargo-insta cargo-deny cargo-shear -y
  corepack enable
  pnpm install
  
fix:
  cargo fmt --all -- --emit=files
  -cargo shear --fix
  cargo fix --allow-dirty --allow-staged
  pnpm fmt
  pnpm lint --fix
