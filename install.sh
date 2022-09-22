#!/bin/sh

cargo build --release
echo "pls enter ur password to install prj :)"
sudo cp target/release/prj /bin/

mkdir ~/.prj
mkdir ~/.prj/pck

cp Config.toml ~/.prj/Config.toml
cp -r pck/* ~/.prj/pck/

echo "done!!"
