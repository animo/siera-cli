#!/usr/bin/env sh

ENDPOINT=https://agent.community.animo.id


echo "Some mock tests... Just for input/output parsing"
echo "Everything should not log anything, unless"
echo "'... -> should fail' is specified"
echo "------------------------------------------------"

echo "features"
cargo run -q -- -q -e=$ENDPOINT features
echo "connections"
CONNECTIONS=`cargo run -q -- -q -e=$ENDPOINT connections --all`
echo "invite"
INVITE=`cargo run -q -- -q -e=$ENDPOINT connections invite`
echo "schema"
SCHEMA_ID=`cargo run -q -- -e=$ENDPOINT schema create -n=test -a=help`
echo "cred def id"
CRED_DEF_ID=`cargo run -q -- -q -e=$ENDPOINT credential-definition create --schema-id=$SCHEMA_ID`
echo "message -> should fail"
cargo run -q -- -q -e=$ENDPOINT message --id=FOO --message=BAR
echo "credential offer -> should fail"
cargo run -q -- -q credentials offer --connection-id=FOO --cred-def-id=BAR -k=A -v=B
