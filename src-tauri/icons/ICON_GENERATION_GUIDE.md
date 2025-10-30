# Icon Generation Guide for Chess Engine

This guide provides complete instructions for generating all required icons for the Chess Engine app across all platforms (Desktop, Android, iOS, and Web/PWA).

## Prerequisites

- Node.js and npm installed
- Tauri CLI installed (via `npm install`)
- A source icon file (1024×1024 PNG recommended)

## Step 1: Create Source Icon

Create a **square 1024×1024 PNG** icon file and save it as:
```
src-tauri/icons/app-icon.png
```

### Icon Design Recommendations:
- Use a square canvas (1024×1024 pixels)
- Keep important content within the center 80% (safe area for rounded corners)
- Use high contrast colors for visibility
- Test on both light and dark backgrounds
- Avoid fine details that won't scale well to small sizes

### Creating the Icon:

**Option 1: Design Software**
- Adobe Illustrator / Photoshop
- Figma (export at 1024×1024)
- Inkscape (free, open-source)
- GIMP (free, open-source)

**Option 2: AI Generation**
- DALL-E, Midjourney, or similar AI tools
- Prompt example: "A minimalist chess piece icon, clean design, suitable for app icon"

**Option 3: Icon Templates**
- Canva (search for "app icon template")
- IconKitchen: https://icon.kitchen/

**Option 4: Stock Icons**
- The Noun Project: https://thenounproject.com/
- Flaticon: https://www.flaticon.com/
- (Ensure proper licensing for commercial use)

## Step 2: Generate Platform Icons with Tauri CLI

Once you have `src-tauri/icons/app-icon.png`, run the Tauri icon generator:

```bash
npm run tauri icon src-tauri/icons/app-icon.png
```

This command automatically generates all required icon sizes and formats:

### Desktop Icons (auto-generated):
- `icons/32x32.png` - Windows taskbar
- `icons/128x128.png` - macOS dock (1x)
- `icons/128x128@2x.png` - macOS dock (2x retina)
- `icons/icon.ico` - Windows executable (multi-size)
- `icons/icon.icns` - macOS application (multi-size)

### Mobile Icons (auto-generated):
The CLI also updates Android and iOS icon assets in:
- `gen/android/app/src/main/res/mipmap-*/` - Android launcher icons
- `gen/apple/Assets.xcassets/AppIcon.appiconset/` - iOS app icons

**Note:** The `tauri.conf.json` is already configured to reference these icons in the `bundle.icon` array.

## Step 3: Generate Web/PWA Icons

The Tauri CLI doesn't generate web icons, so create these manually:

```bash
# Using ImageMagick (install from https://imagemagick.org/)
convert src-tauri/icons/app-icon.png -resize 192x192 public/icons/icon-192.png
convert src-tauri/icons/app-icon.png -resize 512x512 public/icons/icon-512.png
```

**Alternative:** Use an online resizer:
- https://www.iloveimg.com/resize-image
- https://imageresizer.com/
- https://squoosh.app/

Place the generated files in:
- `public/icons/icon-192.png` (192×192) - Referenced by `index.html`
- `public/icons/icon-512.png` (512×512) - For PWA manifest (future use)

## Step 4: Initialize Mobile Platforms (If Not Done)

If you haven't initialized Android/iOS yet, run:

```bash
# For Android
npm run tauri android init

# For iOS (macOS only)
npm run tauri ios init
```

After initialization, re-run the icon generator:
```bash
npm run tauri icon src-tauri/icons/app-icon.png
```

## Step 5: Customize Splash Screens (Optional)

### Android Splash Screen

Edit the launch background:
```
src-tauri/gen/android/app/src/main/res/drawable/launch_background.xml
```

Or customize the theme colors in:
```
src-tauri/gen/android/app/src/main/res/values/styles.xml
```

Example `launch_background.xml`:
```xml
<?xml version="1.0" encoding="utf-8"?>
<layer-list xmlns:android="http://schemas.android.com/apk/res/android">
  <item android:drawable="@color/splash_background"/>
  <item>
    <bitmap
      android:gravity="center"
      android:src="@mipmap/ic_launcher"/>
  </item>
</layer-list>
```

### iOS Splash Screen (macOS only)

Open the Xcode project:
```bash
open src-tauri/gen/apple/Chess\ Engine.xcodeproj
```

Navigate to `Chess Engine > LaunchScreen.storyboard` and customize using Xcode's Interface Builder.

## Step 6: Verify Icons

### Desktop Build
```bash
npm run tauri build
```

Check the built application icon:
- **Windows:** Right-click the `.exe` → Properties → check icon
- **macOS:** Check the `.app` icon in Finder
- **Linux:** Check the `.AppImage` or `.deb` icon

### Android Build
```bash
npm run tauri android build
```

Install the APK on a device/emulator and verify the launcher icon.

### iOS Build (macOS only)
```bash
npm run tauri ios build
```

Install on simulator/device and verify the home screen icon.

### Web Development
```bash
npm run dev
```

Check the browser tab icon and bookmarks icon.

## Troubleshooting

### Icons not showing after generation

1. **Clean and rebuild:**
   ```bash
   rm -rf src-tauri/target
   npm run tauri build
   ```

2. **For mobile, clean build:**
   ```bash
   npm run tauri android build -- --clean
   npm run tauri ios build -- --clean
   ```

### Icon appears pixelated

- Ensure source icon is at least 1024×1024
- Use PNG format, not JPG
- Don't use a transparent background with complex alpha channels

### Android adaptive icon issues

The Tauri CLI generates adaptive icons automatically. If you need custom foreground/background layers, manually edit:
```
gen/android/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml
```

### macOS icon not updating

macOS caches icons aggressively. Force refresh:
```bash
rm ~/Library/Caches/com.apple.iconservices.store
killall Dock
killall Finder
```

## File Checklist

After completing all steps, verify these files exist:

**Source:**
- [x] `src-tauri/icons/app-icon.png` (1024×1024)

**Generated by Tauri CLI:**
- [x] `src-tauri/icons/32x32.png`
- [x] `src-tauri/icons/128x128.png`
- [x] `src-tauri/icons/128x128@2x.png`
- [x] `src-tauri/icons/icon.ico`
- [x] `src-tauri/icons/icon.icns`

**Web/PWA (manual):**
- [x] `public/icons/icon-192.png`
- [x] `public/icons/icon-512.png`

**Mobile (auto-generated after init):**
- [x] `gen/android/app/src/main/res/mipmap-*/ic_launcher.png`
- [x] `gen/apple/Assets.xcassets/AppIcon.appiconset/*.png`

## Quick Start Script

For convenience, here's a complete script (save as `generate-icons.sh`):

```bash
#!/bin/bash

# Check if source icon exists
if [ ! -f "src-tauri/icons/app-icon.png" ]; then
  echo "❌ Error: src-tauri/icons/app-icon.png not found"
  echo "Please create a 1024×1024 PNG source icon first"
  exit 1
fi

# Generate Tauri icons
echo "🎨 Generating Tauri icons..."
npm run tauri icon src-tauri/icons/app-icon.png

# Create web icons directory
mkdir -p public/icons

# Generate web icons (requires ImageMagick)
if command -v convert &> /dev/null; then
  echo "🌐 Generating web icons..."
  convert src-tauri/icons/app-icon.png -resize 192x192 public/icons/icon-192.png
  convert src-tauri/icons/app-icon.png -resize 512x512 public/icons/icon-512.png
  echo "✅ All icons generated successfully!"
else
  echo "⚠️  ImageMagick not found - please generate web icons manually"
  echo "   Required: public/icons/icon-192.png and public/icons/icon-512.png"
fi

echo ""
echo "📋 Next steps:"
echo "1. Verify icons in src-tauri/icons/"
echo "2. Verify web icons in public/icons/"
echo "3. Run 'npm run tauri build' to test desktop build"
echo "4. For mobile: 'npm run tauri android build' or 'npm run tauri ios build'"
```

Make it executable:
```bash
chmod +x generate-icons.sh
./generate-icons.sh
```

## Additional Resources

- [Tauri Icon Generation Docs](https://tauri.app/v1/guides/features/icons)
- [Android Adaptive Icons](https://developer.android.com/guide/practices/ui_guidelines/icon_design_adaptive)
- [iOS App Icon Guidelines](https://developer.apple.com/design/human-interface-guidelines/app-icons)
- [PWA Icon Requirements](https://web.dev/add-manifest/)

## Current Status

⚠️ **Action Required:**
1. Create source icon: `src-tauri/icons/app-icon.png` (1024×1024 PNG)
2. Run: `npm run tauri icon src-tauri/icons/app-icon.png`
3. Generate web icons in `public/icons/`
4. Test build on all target platforms
