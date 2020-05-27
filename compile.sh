#!/bin/bash

features=(
    "google"
    "google"
    "google"
    "wikipedia"
)
providers=(
    "Google"
    "Google Image"
    "Youtube"
    "Wikipedia"
)
default_url=(
    "https://google.com"
    "https://www.google.com/imghp"
    "https://youtube.com"
    "https://wikipedia.org"
)
mkdir -p outcome
for index in ${!providers[@]}; do
    PROVIDER_NAME=${providers[$index]} DEFAULT_URL=${default_url[$index]} cargo build --features ${features[$index]} --release
    mkdir -p temporary
    mkdir -p temporary/executables
    mkdir -p temporary/resources
    provider=`echo "${providers[$index]}" |\
        tr '[:upper:]' '[:lower:]' | tr " " "_"`
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
