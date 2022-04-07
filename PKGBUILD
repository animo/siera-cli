pkgname="aries-cli"
pkgver="1.0.0"
pkggrel="1"
pkgdesc="A CLI tool for Aries written in rust"
arch=(x86_64)
sha512sums=("SKIP")

package() {
  echo "Fetching aries-cli binary\n"
  curl -L /usr/local/share/aries-cli https://github.com/animo/aries-cli/releases/download/v${pkgver}/linux-x86_64-aries-cli
  echo "Successfully added aries-cli to /usr/local/share/aries-cli\n"
  echo "Enjoy the tool"
}
