#!/bin/bash -eux
export $(cat .env | xargs)

rm -rf include
curl -Ls "https://github.com/apache/pulsar/archive/v$PULSAR_VERSION.tar.gz" | tar -zxvf - "pulsar-$PULSAR_VERSION/pulsar-client-cpp/include"
mv "pulsar-$PULSAR_VERSION/pulsar-client-cpp/include" .
rm -rf "pulsar-$PULSAR_VERSION"
