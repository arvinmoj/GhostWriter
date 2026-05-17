#!/bin/bash
# Sign the macOS .app bundle with ad-hoc signature
APP_PATH="src-tauri/target/release/bundle/macos/GhostWriter.app"
DMG_DIR="src-tauri/target/release/bundle/dmg"

if [ -d "$APP_PATH" ]; then
    echo "Signing GhostWriter.app with ad-hoc signature..."
    codesign --force --deep --sign - --entitlements src-tauri/entitlements/ghostwriter.entitlements "$APP_PATH"

    echo "Removing quarantine attribute..."
    xattr -rd com.apple.quarantine "$APP_PATH" 2>/dev/null

    echo "Verifying signing..."
    codesign -dvvv "$APP_PATH" 2>&1 | grep -E "flags|Identifier"

    # Recreate DMG with signed app
    DMG_NAME="GhostWriter_0.1.0_x64.dmg"
    if [ -f "$DMG_DIR/$DMG_NAME" ]; then
        echo "Recreating DMG with signed app..."
        mkdir -p "$DMG_DIR/dmg_tmp"
        cp -R "$APP_PATH" "$DMG_DIR/dmg_tmp/"
        ln -sf /Applications "$DMG_DIR/dmg_tmp/"
        rm -f "$DMG_DIR/$DMG_NAME"
        hdiutil create -volname GhostWriter -srcfolder "$DMG_DIR/dmg_tmp" -ov -format UDZO "$DMG_DIR/$DMG_NAME" -quiet
        rm -rf "$DMG_DIR/dmg_tmp"
        echo "DMG recreated: $DMG_DIR/$DMG_NAME"
    fi

    echo "Signing complete!"
else
    echo "App bundle not found at $APP_PATH"
    echo "Run 'npm run tauri build' first"
    exit 1
fi