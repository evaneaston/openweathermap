#!/bin/bash
# .github/scripts/verify-versions.sh
# Verifies that the tag version matches the versions in Cargo.toml files

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

TAG_VERSION="$1"
echo "Verifying tag version: $TAG_VERSION"

# Function to check version in a Cargo.toml file
check_cargo_version() {
    local cargo_file="$1"
    local expected_version="$2"
    
    if [ ! -f "$cargo_file" ]; then
        echo "Warning: $cargo_file not found, skipping..."
        return 0
    fi
    
    # Extract version from Cargo.toml
    local cargo_version
    cargo_version=$(grep -E '^version\s*=' "$cargo_file" | head -n1 | sed -E 's/.*version\s*=\s*"([^"]+)".*/\1/')
    
    if [ -z "$cargo_version" ]; then
        echo "Error: Could not extract version from $cargo_file"
        return 1
    fi
    
    echo "Found version in $cargo_file: $cargo_version"
    
    if [ "$cargo_version" != "$expected_version" ]; then
        echo "Error: Version mismatch in $cargo_file"
        echo "  Expected: $expected_version"
        echo "  Found: $cargo_version"
        return 1
    fi
    
    echo "✓ Version in $cargo_file matches tag"
    return 0
}

# Check root Cargo.toml
if [ -f "Cargo.toml" ]; then
    check_cargo_version "Cargo.toml" "$TAG_VERSION"
fi

# Find and check all workspace member Cargo.toml files
if [ -f "Cargo.toml" ]; then
    # Extract workspace members from Cargo.toml
    workspace_members=$(cargo metadata --format-version 1 --no-deps | jq -r '.workspace_members[]' | cut -d' ' -f1)
    
    for member in $workspace_members; do
        # Convert package name to likely directory path
        member_dir=$(echo "$member" | sed 's/@.*//' | tr '-' '_')
        
        # Try common locations for workspace member Cargo.toml
        possible_paths=(
            "$member_dir/Cargo.toml"
            "crates/$member_dir/Cargo.toml"
            "packages/$member_dir/Cargo.toml"
            "libs/$member_dir/Cargo.toml"
        )
        
        found=false
        for path in "${possible_paths[@]}"; do
            if [ -f "$path" ]; then
                check_cargo_version "$path" "$TAG_VERSION"
                found=true
                break
            fi
        done
        
        if [ "$found" = false ]; then
            echo "Warning: Could not find Cargo.toml for workspace member: $member"
        fi
    done
else
    echo "No root Cargo.toml found - checking for standalone crate"
fi

echo "✓ All version checks passed!"