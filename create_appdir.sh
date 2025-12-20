#!/bin/bash
# Creates an appimage Appdir from our build
mkdir FTCTui.AppDir
cd FTCTui.AppDir

cp ../target/release/ftctui ./AppRun
cp ../assets/app_icon.png ./icon.png
mkdir Help

mkdir -p usr/share/applications
mkdir -p usr/share/metainfo

echo '<launchable type="desktop-id">ftctui.desktop</launchable>' > usr/share/metainfo/ftctui.appdata.xml

echo "[Desktop Entry]
Type=Application
Name=ftctui
Exec=AppRun
Icon=icon
Terminal=true

NoDisplay=false
Comment=
Categories=Education;
Keywords=ftctui;ftc;fgc;first;" > usr/share/applications/ftctui.desktop

ln -s usr/share/applications/ftctui.desktop ftctui.desktop

cd ..
