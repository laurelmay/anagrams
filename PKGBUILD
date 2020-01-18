# Maintainer: Kyle Laker <kyle@laker.email>

pkgname=anagrams
pkgver=0.2.0
pkgrel=1
pkgdesc="A program to find anagrams"
arch=("x86_64")
url="https://github.com/kylelaker/anagrams"
license=(MIT)
depends=()
makedepends=()
source=("${url}/archive/v${pkgver}.tar.gz")
sha512sums=('SKIP')

build() {
  cd "$srcdir/$pkgname-${pkgver}"
  cargo build --release --locked --all-features
}

check() {
  cd "$srcdir/$pkgname-${pkgver}"
  cargo test --release --locked
}

package() {
  cd "$srcdir/$pkgname-${pkgver}"
  install -Dm 755 "target/release/${pkgname}" -t "${pkgdir}/usr/bin"
  install -Dm 644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}
