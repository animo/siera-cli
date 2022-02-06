#!/usr/bin/env sh

acl issue-credential --full-output --connection-id $(acl connections | jq -r '.[0].connection_id') --credential-definition-id $(acl credential-definition | jq -r '.[0]') --key leeftijd --value 10 --key naam --value bob
