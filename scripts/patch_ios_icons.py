#!/usr/bin/env python3
"""
Patches the Dioxus iOS app bundle with proper icon files.
Dioxus 0.7.x does not generate iOS icons from Dioxus.toml — this script fills the gap.

Usage:
    python3 scripts/patch_ios_icons.py [--bundle PATH] [--src PATH]
"""
import argparse
import os
import plistlib
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DEFAULT_BUNDLE = os.path.join(
    ROOT,
    "target/dx/focus_flow_app/debug/ios/FocusFlowApp.app",
)
DEFAULT_SRC = os.path.join(ROOT, "focus_flow_app/assets/icon.png")

SIZES = [
    ("Icon-20@2x",    40,  40),
    ("Icon-20@3x",    60,  60),
    ("Icon-29@2x",    58,  58),
    ("Icon-29@3x",    87,  87),
    ("Icon-40@2x",    80,  80),
    ("Icon-40@3x",   120, 120),
    ("Icon-60@2x",   120, 120),
    ("Icon-60@3x",   180, 180),
    ("Icon-76@2x",   152, 152),
    ("Icon-83.5@2x", 167, 167),
    ("Icon-1024",   1024,1024),
]

PLIST_ICONS_IPHONE = {
    "CFBundlePrimaryIcon": {
        "CFBundleIconFiles": [
            "Icon-20@2x", "Icon-20@3x",
            "Icon-29@2x", "Icon-29@3x",
            "Icon-40@2x", "Icon-40@3x",
            "Icon-60@2x", "Icon-60@3x",
        ]
    }
}

PLIST_ICONS_IPAD = {
    "CFBundlePrimaryIcon": {
        "CFBundleIconFiles": [
            "Icon-20@2x",
            "Icon-40@2x",
            "Icon-76@2x",
            "Icon-83.5@2x",
        ]
    }
}


def sips_resize(src, dst, w, h):
    subprocess.run(
        ["sips", "-z", str(h), str(w), src, "--out", dst],
        check=True,
        capture_output=True,
    )


def patch(bundle_path, src_icon):
    if not os.path.isdir(bundle_path):
        print(f"ERROR: bundle not found: {bundle_path}", file=sys.stderr)
        sys.exit(1)

    if not os.path.isfile(src_icon):
        print(f"ERROR: source icon not found: {src_icon}", file=sys.stderr)
        sys.exit(1)

    print(f"Bundle : {bundle_path}")
    print(f"Source : {src_icon}")

    # Generate icon sizes into bundle root
    for name, w, h in SIZES:
        dst = os.path.join(bundle_path, f"{name}.png")
        if w == 1024 and h == 1024:
            shutil.copy2(src_icon, dst)
        else:
            sips_resize(src_icon, dst, w, h)
        print(f"  ✓ {name}.png ({w}×{h})")

    # Patch Info.plist
    plist_path = os.path.join(bundle_path, "Info.plist")
    with open(plist_path, "rb") as f:
        data = plistlib.load(f)

    data["CFBundleIcons"] = PLIST_ICONS_IPHONE
    data["CFBundleIcons~ipad"] = PLIST_ICONS_IPAD

    with open(plist_path, "wb") as f:
        plistlib.dump(data, f, fmt=plistlib.FMT_XML)

    print("  ✓ Info.plist patched (CFBundleIcons added)")

    # Reinstall to booted simulator
    result = subprocess.run(
        ["xcrun", "simctl", "install", "booted", bundle_path],
        capture_output=True,
        text=True,
    )
    if result.returncode == 0:
        print("  ✓ Reinstalled to booted simulator")
    else:
        print(f"  ⚠ simctl install failed: {result.stderr.strip()}")
        print("    Run manually: xcrun simctl install booted <bundle>")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--bundle", default=DEFAULT_BUNDLE)
    parser.add_argument("--src", default=DEFAULT_SRC)
    args = parser.parse_args()
    patch(args.bundle, args.src)
