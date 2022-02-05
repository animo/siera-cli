#!/usr/bin/env sh

# Get the alias of the first connection and then get every connection with that alias
acl connections | jq '.[0].alias' | xargs acl connections --alias
