#!/bin/sh

./vendor/openapi-generator-cli generate \
   -i vendor/SpaceTraders.json \
   -g rust \
   -o . \
   --additional-properties packageName=space-traders-api
