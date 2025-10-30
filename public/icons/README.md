# Web/PWA Icons

This directory should contain web and PWA icons for the Chess Engine application.

## Required Icons

Place the following icon files in this directory:

- `icon-192.png` - 192×192 PNG (referenced by apple-touch-icon in index.html)
- `icon-512.png` - 512×512 PNG (for PWA manifest)

## Generation

These icons can be generated from the source icon using an image editor or automated tool:

1. Start with `src-tauri/icons/app-icon.png` (1024×1024)
2. Resize to 192×192 and save as `icon-192.png`
3. Resize to 512×512 and save as `icon-512.png`

### Using ImageMagick (if installed):

```bash
# From the project root
convert src-tauri/icons/app-icon.png -resize 192x192 public/icons/icon-192.png
convert src-tauri/icons/app-icon.png -resize 512x512 public/icons/icon-512.png
```

### Using Online Tools:

- https://www.iloveimg.com/resize-image
- https://imageresizer.com/

## Current Status

⚠️ **Icons not yet generated** - These files need to be created before deployment.
