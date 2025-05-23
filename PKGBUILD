# Maintainer: Your Name <you@example.com>
pkgname=minigrep
pkgver=1.0.0
pkgrel=1
pkgdesc="Lightweight version of grep (build from source)"
arch=('x86_64')
url="https://github.com/gixorian/minigrep"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')

source=("$pkgname-$pkgver.tar.gz::https://github.com/gixorian/minigrep/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('INSERT_REAL_SHA256_HERE')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  install -Dm755 "$srcdir/$pkgname-$pkgver/target/release/minigrep" \
    "$pkgdir/usr/bin/minigrep"
}
