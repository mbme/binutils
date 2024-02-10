# vim: set ft=make :

prod-build-install:
  cp PKGBUILD.template PKGBUILD
  makepkg -efi || true
  rm -rf pkg
  rm -f *.pkg.tar.zst
  rm PKGBUILD
  systemctl --user daemon-reload
  systemctl --user restart arhiv-server.service

check:
  cargo clippy --all-targets --all-features -- -D warnings
  cargo test

clean-all:
  cargo clean
  cargo clean --release
  rm -rf .log
