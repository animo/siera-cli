#!/usr/bin/env sh

set -e

ENDPOINT=https://agent.community.animo.id

echo "Some mock tests... Just for input/output parsing"
echo "------------------------------------------------"

features() {
  echo "--- Features ---"
  cargo run -q -- -u=$ENDPOINT  features &> /dev/null
  handle_out $? 0 "Features: Get All"
}

message() {
  echo "--- Message ---"
  cargo run -q -- -q -u=$ENDPOINT  message --id=FOO --message=BAR &> /dev/null
  handle_out $? 1 "Message: Send Message"
}

connections() {
  echo "--- Connections ---"
  cargo run -q -- -q -u=$ENDPOINT  connections &> /dev/null
  handle_out $? 0 "Connections: Get All"

  cargo run -q -- -q -u=$ENDPOINT  connections invite &> /dev/null
  handle_out $? 0 "Connections: Invite"
}

schemas() {
  echo "--- Schemas ---"
  SCHEMA_ID=`cargo run -q -- -u=$ENDPOINT schemas create -n=foo -a=bar -a=baz 2> /dev/null`
  handle_out $? 0 "Schemas: Create"

  cargo run -q -- -q -u=$ENDPOINT schemas &> /dev/null
  handle_out $? 0 "Schemas: Get All"

  cargo run -q -- -q -u=$ENDPOINT schemas --id=$SCHEMA_ID &> /dev/null
  handle_out $? 0 "Schemas: Get By Id"
}

credential_definitions() {
  echo "--- Credential Definitions --- "
  SCHEMA_ID=`cargo run -q -- -u=$ENDPOINT schemas create -n=foo -a=bar -a=baz 2> /dev/null`

  CRED_DEF_ID=`cargo run -q -- -u=$ENDPOINT credential-definitions create --schema-id=$SCHEMA_ID 2> /dev/null`
  handle_out $? 0 "Credential Definitions: create"

  cargo run -q -- -q -u=$ENDPOINT  credential-definitions &> /dev/null
  handle_out $? 0 "Credential Definitions: Get All"

  cargo run -q -- -q -u=$ENDPOINT  credential-definitions --id=$CRED_DEF_ID &> /dev/null
  handle_out $? 0 "Credential Definitions: Get By Id"
}

credentials() {
  echo "--- Credentials ----"
  SCHEMA_ID=`cargo run -q -- -u=$ENDPOINT schemas create -n=foo -a=bar -a=baz 2> /dev/null 2> /dev/null`
  CRED_DEF_ID=`cargo run -q -- -u=$ENDPOINT  credential-definitions create --schema-id=$SCHEMA_ID 2> /dev/null`
  cargo run -q -- -q credentials offer --connection-id=FOO --cred-def-id=$CRED_DEF_ID -k=bar -v=B -k=baz -v=C &> /dev/null
  handle_out $? 1 "credentials: Offer"
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


features
message
connections
schemas
credential_definitions
credentials
