# Maintainer: Goran Borovecki  <goranborovecki1@gmail.com>
pkgname=minigrep
pkgver=2.0.0
pkgrel=1
pkgdesc="Lightweight version of grep (build from source)"
arch=('x86_64')
url="https://github.com/gixorian/minigrep"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')

source=("$pkgname-$pkgver.tar.gz::https://github.com/gixorian/minigrep/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('593390c89baf0d9e806240cd2367648424d9d4732ae2246e9891d69c6e2206a5')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  install -Dm755 "$srcdir/$pkgname-$pkgver/target/release/minigrep" \
    "$pkgdir/usr/bin/minigrep"
}
