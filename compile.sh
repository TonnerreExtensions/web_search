#!/bin/bash

features=(
    "google"
    "google"
    "google"
    "wikipedia"
    "google_maps api_key"
)
providers=(
    "Google"
    "Google Image"
    "Youtube"
    "Wikipedia"
    "Google Maps"
)

function buildrs {
    echo "fn main() { println!(\"cargo:rustc-env=PROVIDER_NAME=$1\"); }" > build.rs
}

function clean_buildrs {
    rm build.rs
}

mkdir -p outcome
for index in ${!providers[@]}; do
    buildrs "${providers[$index]}"
    features=${features[$index]}

    PROVIDER_NAME=${providers[$index]} cargo build\
        --features "$features"\
        --release

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
clean_buildrs
