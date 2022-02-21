#!/usr/bin/env sh

ENDPOINT="https://agent.community.animo.id"
ALIAS="demo"

aries-cli -e $ENDPOINT connections invite -l $ALIAS --qr