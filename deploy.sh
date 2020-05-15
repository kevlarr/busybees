#!/usr/bin/sh

if [ -z "$TARGETDIR" ]
then
    echo Error: Must set TARGETDIR
    exit 1
fi

set -e

VERSION=$(cat Cargo.toml | grep version -m 1 | grep -oE '[0-9\.]+')
TARGETDIR="$TARGETDIR/$VERSION"

echo "Building busybees:$VERSION"
(
    set -x
    cargo build --all --release
)

echo "\nCopying binary and assets"
(
    set -x
    mkdir $TARGETDIR
    cp target/debug/server $TARGETDIR/server-$VERSION
    cp .env $TARGETDIR/
    cp -r www $TARGETDIR
)

echo "\nStarting server"
(
    set -x
    nohup $TARGETDIR/server-$VERSION &
)
sleep 1
cat nohup.out
