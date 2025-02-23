PKG_NAME="red"
LICENSE="MIT"
AUTHORS="Kaedehito"
VERSION="1.0.0"
DESCRIPTON="Rust version of the Ed editor"

BUILD(){
  cargo build --release
}

INSTALL(){
  mv target/release/red .
}
