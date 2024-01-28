# Maintainer: Gaurav Atreya <allmanpride@gmail.com>
pkgname=beamer-quickie
pkgver=0.2
pkgrel=1
pkgdesc="Quickly copy your beamer slides with GUI"
arch=('x86_64')
url="https://github.com/Atreyagaurav/${pkgname}"
license=('GPL3')
# need to figure out what is make depend and not
depends=('gcc-libs' 'gtk4' 'gtksourceview5')
makedepends=('rust' 'cargo' 'git')

build() {
	cargo build --release
}

package() {
    cd "$srcdir"
    mkdir -p "$pkgdir/usr/bin"
    cp "../target/release/${pkgname}" "$pkgdir/usr/bin/${pkgname}"
    mkdir -p "$pkgdir/usr/share/applications/"
    cp "../${pkgname}.desktop" "$pkgdir/usr/share/applications/${pkgname}.desktop"
    mkdir -p "$pkgdir/usr/share/${pkgname}/icons/"
    cp "../resources/window.ui" "$pkgdir/usr/share/${pkgname}/"
    cp "../resources/slide.ui" "$pkgdir/usr/share/${pkgname}/"
    cp "../resources/resources.gresource.xml" "$pkgdir/usr/share/${pkgname}/"
    cp "../resources/icons/slide.svg" "$pkgdir/usr/share/${pkgname}/icons/"
}
