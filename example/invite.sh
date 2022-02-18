#!/usr/bin/env sh

ENDPOINT="https://agent.community.animo.id"
ALIAS="demo"

aries-cli -e $ENDPOINT invite -l $ALIAS --qr
