#!/bin/bash
echo "# Extract version from Cargo.toml"
version=$(cat Cargo.toml | awk -F'"' '/^version =/ {print $2}')

echo "# Create dev tag"
git tag $version
git push origin --tags

echo "# Create new release"
gh release create $version --title "Development Release" --release <<< ""