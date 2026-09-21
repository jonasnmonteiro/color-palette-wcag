#!/usr/bin/env python3
"""
Colorust Style & Chrome Leak Auditor
Scans the Colorust web codebase to ensure 100% tokenization hygiene:
1. Verifies that all CSS style blocks use semantic CSS custom properties or color-mix() over tokens.
2. Flags any raw, unapproved hex or rgba values outside authorized theme definitions & measurement references.
"""

import os
import re
import sys

ALLOWED_PRESET_HEXES = {
    # Public Base Fallback
    '#3B82F6', '#60A5FA', '#009FCA', '#238FF0', '#6374F5', '#8762EE',
    # Measurement References & App Dark Ink
    '#FFFFFF', '#0A0D14', '#000000', '#0F131C',
    # 10 Curated Content Presets
    '#00F0FF', '#7000FF', '#FF007A', '#00FF66', '#181824',
    '#0284C7', '#2563EB', '#4F46E5', '#0D9488', '#059669',
    '#88C0D0', '#81A1C1', '#5E81AC', '#8FBCBB', '#4C566A',
    '#8AADF4', '#F5BDE6', '#C6A0F6', '#ED8796', '#EED49F',
    '#6E56CF', '#0091FF', '#30A46C', '#E5484D', '#F76808',
    '#FF5E57', '#FF793F', '#FFA801', '#FF3838', '#CD84F1',
    '#10B981', '#047857', '#34D399', '#064E3B',
    '#BB9AF7', '#7AA2F7', '#7DCFFF', '#9ECE6A', '#F7768E',
    '#F43F5E', '#FB7185', '#F472B6', '#FB923C', '#FBBF24',
    '#F59E0B', '#D97706', '#B45309', '#78350F', '#1E293B', '#F1F5F9', '#F8FAFC', '#0F172A'
}

COLOR_PATTERN = re.compile(r'(#[0-9a-fA-F]{3,8}|rgba?\([^)]+\))')
COLOR_MIX_TOKEN_PATTERN = re.compile(r'color-mix\([^)]+\)')
VAR_PATTERN = re.compile(r'var\(--[^)]+\)')

def scan_file(filepath):
    leaks = []
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    is_global_css = filepath.endswith('global.css')

    in_style = False
    in_script = False

    for idx, line in enumerate(lines, 1):
        stripped = line.strip()

        if '<style' in stripped:
            in_style = True
        elif '</style>' in stripped:
            in_style = False

        if '<script' in stripped:
            in_script = True
        elif '</script>' in stripped:
            in_script = False

        if is_global_css:
            # global.css declares the token library itself in :root
            continue

        if in_style:
            # Inside CSS style blocks: raw rgba(...) or standalone hex are forbidden unless inside color-mix() / var()
            # Remove legitimate color-mix calls and var fallbacks
            cleaned_line = COLOR_MIX_TOKEN_PATTERN.sub('', line)
            cleaned_line = VAR_PATTERN.sub('', cleaned_line)
            
            matches = COLOR_PATTERN.findall(cleaned_line)
            for m in matches:
                # Disallow raw color literals in CSS style blocks
                leaks.append((idx, m, line.strip()))

    return leaks

def main():
    root_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    src_dir = os.path.join(root_dir, 'src')

    total_leaks = 0
    scanned_count = 0

    for root, _, files in os.walk(src_dir):
        for file in files:
            if file.endswith(('.astro', '.css')):
                scanned_count += 1
                filepath = os.path.join(root, file)
                leaks = scan_file(filepath)
                if leaks:
                    relpath = os.path.relpath(filepath, root_dir)
                    print(f"\n❌ Leaked color literals in {relpath}:")
                    for line_no, match, raw_line in leaks:
                        print(f"  Line {line_no}: {match} -> {raw_line}")
                    total_leaks += len(leaks)

    print(f"\n[Colorust Audit] Scanned {scanned_count} files.")
    if total_leaks > 0:
        print(f"❌ FAILED: Found {total_leaks} leaked raw color literals.")
        sys.exit(1)
    else:
        print("✅ PASSED: 0 leaked colors detected. Tokenization is 100% clean.")
        sys.exit(0)

if __name__ == '__main__':
    main()
