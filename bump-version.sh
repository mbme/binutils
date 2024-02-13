#! /usr/bin/env bash

CURRENT_VERSION=$(git describe --abbrev=0 --tags)
NEXT_VERSION=$(($CURRENT_VERSION + 1))

echo "Current version is $CURRENT_VERSION"

read -p "Are you sure you want to create new tag $NEXT_VERSION? (y)" -n 1 -r
echo    # (optional) move to a new line
if [[ $REPLY =~ ^[Yy]$ ]]
then
  git tag $NEXT_VERSION
  git push
  git push --tags
else
  echo "Cancelled"
fi
