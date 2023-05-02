# Maintainer: Jere Hasu <jerehasu04 at gmail dot com>
pkgname=ultimate-cowardice-git
pkgver=r5.3bc047b
pkgrel=1
pkgdesc="Randomizing roles and champions for league of legends"
arch=('x86_64')
url="https://github.com/zexee24/ultimate-cowardice/"
license=('unknown')
depends=()
makedepends=('cargo' 'git') 
source=("$pkgname::git+https://github.com/zexee24/ultimate-cowardice")
md5sums=('SKIP')


pkgver() {
	cd "$pkgname"
	printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

prepare() {
	cd "$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
	cd "$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
	cd "$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
	cd "$pkgname"
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/ultimate-cowardice"
}
