# Mainainer: Animo Solutions development@animo.id
pkgname="aries-cli"
pkgver="0.1.0"
pkgrel="1"
pkgdesc="A CLI tool for Aries written in rust"
arch=(x86_64)
license=('MIT')
depends=('sudo' 'libxcb')
provides=('aries-cli')
md5sums=()

package() {
  echo "Fetching aries-cli binary"
  sudo curl -L -o /usr/bin/aries-cli https://github.com/animo/aries-cli/releases/download/v${pkgver}/linux-x86_64-aries-cli
  sudo chmod +x /usr/bin/aries-cli
  echo "Successfully installed aries-cli"
  echo "Enjoy the tool"
}
