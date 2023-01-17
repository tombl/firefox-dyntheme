# shellcheck shell=bash

pkgname=firefox-dyntheme
pkgver=1.0.0
pkgrel=1
pkgdesc="Firefox dynamic theme reloader"
arch=('any')
url="https://github.com/tombl/firefox-dyntheme"
license=('MIT')
makedepends=('cargo' 'web-ext')
source=(".")

# /usr/lib/firefox/browser/extensions/dyntheme@tombl.dev.xpi
# /usr/lib/mozilla/native-messaging-hosts/dyntheme@tombl.dev.json

build() {
    echo $srcdir

    # cd "$srcdir/native"
    # cargo build --release

    # cd "$srcdir/ext"
    # web-ext build
}

package() {
    :
    # cd "$srcdir/native"
    # install -Dm755 "target/release/firefox_dyntheme" "$pkgdir/usr/bin/firefox_dyntheme"

    # cd "$srcdir/ext"
    # install -Dm644 "
}