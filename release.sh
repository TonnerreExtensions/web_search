#!/bin/bash

GITHUB_TOKEN=$1
VERSION=$2
TITLE=$3
BODY=$4

names=(
    'google'
    'google_image'
    'youtube'
    'wikipedia'
    'google_maps'
)
repos=(
    'GoogleSearch'
    'GoogleImage'
    'YouTube'
    'Wikipedia'
    'GoogleMaps'
)

function help {
    echo "./release.sh <GITHUB_TOKEN> <VERSION> <TITLE> <BODY>"
    exit 1
}

[[ -z $GITHUB_TOKEN ]] && help
[[ -z $VERSION ]] && help
[[ -z $TITLE ]] && help

for index in ${!names[@]}; do
    name=${names[$index]}
    repo=${repos[$index]}
    echo "Uploading $repo..."
    response=`(curl -H "Authorization: token $GITHUB_TOKEN" -H "Content-Type: application/json" --request POST --data '{"tag_name": '"\"$VERSION\""', "target_commitish": "master", "name": '"\"$TITLE\""', "body": '"\"$BODY\""', "draft": false, "prerelease": false}' "https://api.github.com/repos/TonnerreExtensions/$repo/releases")`
    id=`echo $response | python3 -c "import sys, json; print(json.load(sys.stdin)['id'])"`
    curl -H "Authorization: token $GITHUB_TOKEN" -H "Content-Type: application/octet-stream" --data-binary @"outcome/$name.zip" --request POST "https://uploads.github.com/repos/TonnerreExtensions/$repo/releases/$id/assets?name=$name.zip"
done
