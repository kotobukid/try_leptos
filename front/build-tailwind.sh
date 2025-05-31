#!/bin/bash


cd "$(dirname "$0")"

# デバッグ出力を追加
echo "Debug information:"
echo "TRUNK_PROFILE: $TRUNK_PROFILE"
echo "NODE_ENV: $NODE_ENV"
echo "PWD: $(pwd)"

if [ ! -d "node_modules" ]; then
  echo "Installing npm dependencies..."
  npm install
fi

## Check if this is a release build by checking the TRUNK_PROFILE environment variable
#if [[ "$TRUNK_PROFILE" == "release" ]]; then
#  echo "Building Tailwind CSS for release (purged and minified)..."
#  # Set NODE_ENV to production to ensure Tailwind's purging is enabled
#  NODE_ENV=production npx tailwindcss -i ./tailwind.css -o ./dist/tailwind.css --minify
#else
#  echo "Building Tailwind CSS for development..."
#  npx tailwindcss -i ./tailwind.css -o ./dist/tailwind.css
#fi

if [ "$TRUNK_PROFILE" = "release" ] || [ "$NODE_ENV" = "production" ]; then
  echo "Building Tailwind CSS for release (purged and minified)..."
  NODE_ENV=production npx tailwindcss -i ./tailwind.css -o ./dist/tailwind.css --minify
else
  echo "Building Tailwind CSS for development..."
  npx tailwindcss -i ./tailwind.css -o ./dist/tailwind.css
fi

# If we're building to a different dist directory (like in gen.sh),
# copy the CSS file there as well
if [ -n "$TRUNK_DIST_DIR" ]; then
  # Get absolute paths for both source and destination
  SRC_PATH=$(realpath ./dist/tailwind.css)
  DEST_DIR=$(realpath "$TRUNK_DIST_DIR")
  DEST_PATH="$DEST_DIR/tailwind.css"

  # Only copy if source and destination are different files
  if [ "$SRC_PATH" != "$DEST_PATH" ]; then
    echo "Copying Tailwind CSS to custom dist directory: $TRUNK_DIST_DIR"
    mkdir -p "$DEST_DIR"
    cp "$SRC_PATH" "$DEST_PATH"
  else
    echo "Source and destination are the same file, skipping copy"
  fi
fi
