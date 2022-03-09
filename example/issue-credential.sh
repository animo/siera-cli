#!/usr/bin/env sh

ENDPOINT="https://agent.community.animo.id"

CONNECTION=$(cargo run -q -- -e $ENDPOINT connections --all | jq -r '.[0].connection_id' )

SCHEMA_ID=$(cargo run -q -- -e $ENDPOINT schema create -n test-demo -a blob -a bleb)

CRED_DEF_ID=$(cargo run -q -- -e $ENDPOINT credential-definition create --schema-id=${SCHEMA_ID})

# Issue credential
cargo run -q --  -e $ENDPOINT credentials offer --connection-id $CONNECTION --cred-def-id $CRED_DEF_ID --key bleb --value 10 --key blob --value bob
