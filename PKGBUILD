pkgname=ultimate-cowardice-git
pkgver=r2.315ec99
pkgrel=1
pkgdesc="Randomizing roles and champions for league of legends"
arch=('x86_64')
url="https://github.com/zexee24/ultimate-cowardice/"
license=('unknown')
depends=()
makedepends=('cargo' 'git') 
source=('$pkgname::https://github.com/zexee24/ultimate-cowardice.git')
noextract=()
md5sums=('SKIP')


pkgver() {
	printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/ultimate-cowardice"
}
