PKG_NAME="red"
LICENSE="MIT"
AUTHORS="Kaedehito"
VERSION="1.0.0"
DESCRIPTON="Rust version of ed"

BUILD(){
  cargo build --release
}

INSTALL(){
  # Nothing to do
  true
}
