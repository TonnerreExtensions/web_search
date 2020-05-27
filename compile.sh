#!/bin/bash

features=(
    "google"
)
providers=(
    "Google"
)
default_url=(
    "https://google.com"
)
mkdir -p outcome
for index in ${!providers[@]}; do
    PROVIDER_NAME=${providers[$index]} DEFAULT_URL=${default_url[$index]} cargo build --features ${features[$index]} --release
    mkdir -p temporary
    mkdir -p temporary/executables
    mkdir -p temporary/resources
    provider=`echo "${providers[$index]}" | tr '[:upper:]' '[:lower:]'`
    cp target/release/web_search temporary/executables/main
    cp "profiles/$provider.yaml" temporary/profile.yaml
    cp "settings/$provider.yaml" temporary/settings.yaml
    cp "resources/$provider.png" temporary/resources/icon.png
    cd temporary
    zip -r "$provider.zip" .
    mv "$provider.zip" ../outcome
    cd ..
    rm -r temporary
done
