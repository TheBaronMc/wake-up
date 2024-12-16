#!/bin/bash

echo "# Delete previous dev tag"
if gh release list | grep dev > /dev/null 2>&1; then
    gh release delete dev --cleanup-tag -y
    git tag -d dev
    git push origin dev
fi

echo "# Create dev tag"
git tag dev
git push origin --tags

echo "# Create new pre-release"
gh release create dev --title "Development Release" --prerelease <<< ""