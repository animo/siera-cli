#!/usr/bin/env sh

ENDPOINT="https://agent.community.animo.id"
ALIAS="demo"

CONNECTION=$(aries-cli -e $ENDPOINT connections --alias $ALIAS | jq -r '.[0].connection_id' )

CRED_DEF_ID=$(aries-cli -e $ENDPOINT credential-definition | jq -r '.[0]')

# Issue credential
aries-cli -e $ENDPOINT issue-credential --full-output --connection-id $CONNECTION --credential-definition-id $CRED_DEF_ID --key leeftijd --value 10 --key naam --value bob
