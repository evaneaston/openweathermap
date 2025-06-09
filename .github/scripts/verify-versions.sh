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

# Check if this is a workspace or single crate project
if [ -f "Cargo.toml" ]; then
    if cargo metadata --format-version 1 --no-deps | jq -e '.workspace_root' > /dev/null 2>&1; then
        echo "Detected workspace project - checking workspace members only"
        
        # Get workspace member information using cargo metadata
        workspace_info=$(cargo metadata --format-version 1 --no-deps)
        
        # Check each workspace member
        echo "$workspace_info" | jq -r '.packages[] | "\(.name) \(.manifest_path)"' | while read -r crate_name manifest_path; do
            echo "Checking workspace member: $crate_name"
            check_cargo_version "$manifest_path" "$TAG_VERSION"
        done
        
        # Check if we found any workspace members
        member_count=$(echo "$workspace_info" | jq -r '.packages | length')
        if [ "$member_count" -eq 0 ]; then
            echo "Warning: No workspace members found"
        else
            echo "Checked $member_count workspace member(s)"
        fi
    else
        echo "Detected single crate project - checking root Cargo.toml"
        check_cargo_version "Cargo.toml" "$TAG_VERSION"
    fi
else
    echo "Error: No Cargo.toml found in current directory"
    exit 1
fi

echo "✓ All version checks passed!"