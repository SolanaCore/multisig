#!/bin/bash

###############################################################################
# Script to publish an npm package using an access token.
# It does NOT modify the version in package.json.
#
# Usage:
#   ./publish_package.sh [<package-folder>]
#
# Example:
#   ./publish_package.sh packages/common
#
# Prerequisites:
# - Set NPM_TOKEN environment variable with a valid automation token
# - Add `.npmrc` in repo root or package folder with:
#     //registry.npmjs.org/:_authToken=${NPM_TOKEN}
###############################################################################

set -e

PACKAGE_DIR=${1:-"."}  # Default to current directory if none provided

# Check for NPM token
if [ -z "$NPM_TOKEN" ]; then
  echo "❌ NPM_TOKEN environment variable not set."
  exit 1
fi

echo "📦 Publishing package from: $PACKAGE_DIR"

# Move into the package folder
pushd "$PACKAGE_DIR" > /dev/null

# Show version being published
VERSION=$(grep '"version"' package.json | head -1 | sed -E 's/.*: "(.*)",/\1/')
echo "📝 Detected version: $VERSION"

# Optional: Run build if build script exists
if grep -q '"build"' package.json; then
  echo "🛠️  Running build..."
  npm run build
else
  echo "⚠️  No build script found. Skipping build."
fi

# Publish the package
echo "🚀 Publishing to npm..."
npm publish --access public

# Return to original directory
popd > /dev/null
echo "✅ Successfully published version $VERSION from $PACKAGE_DIR"

