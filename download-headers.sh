#!/bin/bash -eux
VERSION="2.6.1"

rm -rf ./include
curl -Ls "https://github.com/apache/pulsar/archive/v$VERSION.tar.gz" | tar -xvf - "pulsar-$VERSION/pulsar-client-cpp/include"
mv "pulsar-$VERSION/pulsar-client-cpp/include" .
rm -rf "pulsar-$VERSION"
