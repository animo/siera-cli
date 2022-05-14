#!/usr/bin/env bash

ENDPOINT=https://agent.community.animo.id

echo "Some mock tests... Just for input/output parsing"
echo "------------------------------------------------"

feature() {
  echo "--- Features ---"
  cargo run -q -- -u=$ENDPOINT  feature &> /dev/null
  handle_out $? 0 "Features: Get All"
}

message() {
  echo "--- Message ---"
  cargo run -q -- -q -u=$ENDPOINT  message --connection-id=FOO --message=BAR &> /dev/null
  handle_out $? 1 "Message: Send Message"
}

connection() {
  echo "--- Connections ---"
  cargo run -q -- -q -u=$ENDPOINT  connection list &> /dev/null
  handle_out $? 0 "Connections: Get All"

  cargo run -q -- -q -u=$ENDPOINT  connection invite &> /dev/null
  handle_out $? 0 "Connections: Invite"
}

schema() {
  echo "--- Schemas ---"
  SCHEMA_ID=`cargo run -q -- -u=$ENDPOINT schema create -n=foo -a=bar -a=baz 2> /dev/null`
  handle_out $? 0 "Schemas: Create"

  cargo run -q -- -q -u=$ENDPOINT schema list &> /dev/null
  handle_out $? 0 "Schemas: Get All"

  cargo run -q -- -q -u=$ENDPOINT schema list --id=$SCHEMA_ID &> /dev/null
  handle_out $? 0 "Schemas: Get By Id"
}

credential_definition() {
  echo "--- Credential Definitions --- "
  SCHEMA_ID=`cargo run -q -- -u=$ENDPOINT schema create -n=foo -a=bar -a=baz 2> /dev/null`

  CRED_DEF_ID=`cargo run -q -- -u=$ENDPOINT credential-definition create --schema-id=$SCHEMA_ID 2> /dev/null`
  handle_out $? 0 "Credential Definitions: create"

  cargo run -q -- -q -u=$ENDPOINT  credential-definition list &> /dev/null
  handle_out $? 0 "Credential Definitions: Get All"

  cargo run -q -- -q -u=$ENDPOINT  credential-definition list --id=$CRED_DEF_ID &> /dev/null
  handle_out $? 0 "Credential Definitions: Get By Id"
}

credential() {
  echo "--- Credentials ----"
  SCHEMA_ID=`cargo run -q -- -u=$ENDPOINT schema create -n=foo -a=bar -a=baz 2> /dev/null 2> /dev/null`
  CRED_DEF_ID=`cargo run -q -- -u=$ENDPOINT  credential-definition create --schema-id=$SCHEMA_ID 2> /dev/null`
  cargo run -q -- -q credential offer --connection-id=FOO --cred-def-id=$CRED_DEF_ID -k=bar -v=B -k=baz -v=C &> /dev/null
  handle_out $? 1 "credential: Offer"
}

finish() {
  echo "+-----------------------+"
  echo "| Tests have completed! |"
  echo "+-----------------------+"
}

handle_out() {
  EXIT_CODE=$1
  SHOULD_BE=$2
  TEST_NAME=$3

  if [[ $EXIT_CODE != $SHOULD_BE ]]
  then
    echo "${TEST_NAME} went wrong."
    exit 1
  else
    echo "${TEST_NAME} finished correctly with status $1"
  fi
}


feature
message
connection
schema
credential_definition
credential
finish
