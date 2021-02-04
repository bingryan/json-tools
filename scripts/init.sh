set -e

echo "*** Initializing  build environment"

if [ -z $CI ] ; then
   rustup update nightly
   rustup update stable
fi

rustup default nightly