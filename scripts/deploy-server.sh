#!/usr/bin/env bash
set -e

SERVER=reineke.hannobraun.de
SERVICE=vndf-server

(
    cd $SERVICE
    makepkg --noconfirm --force --syncdeps --install)

PACKAGE_PATH=$(find $SERVICE -name $SERVICE-*.pkg.tar.xz)
PACKAGE_NAME=$(basename $PACKAGE_PATH)
TARGET_PATH=/tmp/$PACKAGE_NAME

scp $PACKAGE_PATH root@$SERVER:$TARGET_PATH

COMMANDS="\
pacman -U $TARGET_PATH --noconfirm &&
systemctl enable $SERVICE &&
systemctl restart $SERVICE"

ssh root@$SERVER \
	-o "BatchMode yes" \
	"$COMMANDS"
