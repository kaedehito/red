PKG_NAME="red"
LICENSE="MIT"
AUTHORS="Kaedehito"
VERSION="1.0.0"
DESCRIPTON="The test package for spkg"

BUILD(){
  cargo build --release
}

INSTALL(){
  # Nothing to do
  true
}
