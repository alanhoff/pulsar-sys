#!/bin/bash -eux
export $(cat .env | xargs)

curl -O https://archive.apache.org/dist/pulsar/pulsar-$PULSAR_VERSION/DEB/apache-pulsar-client.deb
curl -O https://archive.apache.org/dist/pulsar/pulsar-$PULSAR_VERSION/DEB/apache-pulsar-client-dev.deb
sudo apt purge -y apache-pulsar-client apache-pulsar-client-dev || true
sudo apt install -y ./apache-pulsar-client.deb ./apache-pulsar-client-dev.deb
rm -rf apache-pulsar-client.deb apache-pulsar-client-dev.deb
