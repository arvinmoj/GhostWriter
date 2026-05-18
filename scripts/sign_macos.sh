#!/bin/bash
# Sign the macOS .app bundle with ad-hoc signature
# Requires TARGET env var (e.g. x86_64-apple-darwin, aarch64-apple-darwin)

if [ -z "$TARGET" ]; then
    echo "ERROR: TARGET env var not set"
    exit 1
fi

# Derive architecture suffix
if [ "$TARGET" = "x86_64-apple-darwin" ]; then
    ARCH_SUFFIX="x64"
elif [ "$TARGET" = "aarch64-apple-darwin" ]; then
    ARCH_SUFFIX="aarch64"
else
    echo "ERROR: Unsupported target $TARGET"
    exit 1
fi

# Read version from tauri.conf.json
VERSION=$(grep '"version"' src-tauri/tauri.conf.json | head -1 | sed 's/.*: *"//;s/".*//')
if [ -z "$VERSION" ]; then
    echo "ERROR: Could not read version from tauri.conf.json"
    exit 1
fi

APP_PATH="src-tauri/target/${TARGET}/release/bundle/macos/GhostWriter.app"
DMG_DIR="src-tauri/target/${TARGET}/release/bundle/dmg"
DMG_NAME="GhostWriter_${VERSION}_${ARCH_SUFFIX}.dmg"

if [ -d "$APP_PATH" ]; then
    echo "Signing GhostWriter.app with ad-hoc signature (target: $TARGET)..."
    codesign --force --deep --sign - --entitlements src-tauri/entitlements/ghostwriter.entitlements "$APP_PATH"

    echo "Removing quarantine attribute..."
    xattr -rd com.apple.quarantine "$APP_PATH" 2>/dev/null

    echo "Verifying signing..."
    codesign -dvvv "$APP_PATH" 2>&1 | grep -E "flags|Identifier"

    # Recreate DMG with signed app
    if [ -f "$DMG_DIR/$DMG_NAME" ]; then
        echo "Recreating DMG with signed app..."
        mkdir -p "$DMG_DIR/dmg_tmp"
        cp -R "$APP_PATH" "$DMG_DIR/dmg_tmp/"
        ln -sf /Applications "$DMG_DIR/dmg_tmp/"
        rm -f "$DMG_DIR/$DMG_NAME"
        hdiutil create -volname GhostWriter -srcfolder "$DMG_DIR/dmg_tmp" -ov -format UDZO "$DMG_DIR/$DMG_NAME" -quiet
        rm -rf "$DMG_DIR/dmg_tmp"
        echo "DMG recreated: $DMG_DIR/$DMG_NAME"
    else
        echo "WARNING: Original DMG not found at $DMG_DIR/$DMG_NAME"
        echo "Creating new DMG from signed app..."
        mkdir -p "$DMG_DIR"
        mkdir -p "$DMG_DIR/dmg_tmp"
        cp -R "$APP_PATH" "$DMG_DIR/dmg_tmp/"
        ln -sf /Applications "$DMG_DIR/dmg_tmp/"
        hdiutil create -volname GhostWriter -srcfolder "$DMG_DIR/dmg_tmp" -ov -format UDZO "$DMG_DIR/$DMG_NAME" -quiet
        rm -rf "$DMG_DIR/dmg_tmp"
        echo "DMG created: $DMG_DIR/$DMG_NAME"
    fi

    echo "Signing complete!"
else
    echo "App bundle not found at $APP_PATH"
    echo "Run 'npm run tauri build' first"
    exit 1
fi
