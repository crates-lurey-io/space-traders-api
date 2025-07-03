#!/bin/sh

OPENAPI_GENERATOR_VERSION=7.14.0 ./vendor/openapi-generator-cli generate \
   -i vendor/SpaceTraders.json \
   -g rust \
   -o . \
   --additional-properties packageName=space-traders-api
