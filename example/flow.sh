#!/usr/bin/env sh

# Issue credential
aries-cli issue-credential --full-output \ 
                     --connection-id $(aries-cli connections | jq -r '.[0].connection_id') \
                     --credential-definition-id $(aries-cli credential-definition | jq -r '.[0]') \
                     --key leeftijd \
                     --value 10 \
                     --key naam \
                     --value bob
