#!/bin/bash
# .github/scripts/publish-crates.sh
# Publishes all workspace members to crates.io in dependency order

set -e

if [ -z "$CARGO_REGISTRY_TOKEN" ]; then
    echo "Error: CARGO_REGISTRY_TOKEN environment variable is not set"
    exit 1
fi

echo "Starting crates.io publication process..."

# Function to check if a crate is already published at the current version
is_published() {
    local crate_name="$1"
    local version="$2"
    
    echo "Checking if $crate_name@$version is already published..."
    
    # Query crates.io API to check if this version exists
    local response
    response=$(curl -s "https://crates.io/api/v1/crates/$crate_name" || echo "")
    
    if [ -z "$response" ]; then
        echo "$crate_name not found on crates.io, will publish"
        return 1
    fi
    
    # Check if the specific version exists
    local version_exists
    version_exists=$(echo "$response" | jq -r ".versions[] | select(.num == \"$version\") | .num" 2>/dev/null || echo "")
    
    if [ "$version_exists" = "$version" ]; then
        echo "$crate_name@$version already published, skipping"
        return 0
    else
        echo "$crate_name@$version not published, will publish"
        return 1
    fi
}

# Function to publish a single crate
publish_crate() {
    local crate_path="$1"
    local crate_name="$2"
    local version="$3"
    
    echo "Publishing $crate_name@$version from $crate_path..."
    
    # Check if already published
    if is_published "$crate_name" "$version"; then
        return 0
    fi
    
    # Publish the crate
    cd "$crate_path"
    
    echo "Running cargo publish for $crate_name..."
    if cargo publish --token "$CARGO_REGISTRY_TOKEN"; then
        echo "✓ Successfully published $crate_name@$version"
        
        # Wait a bit for crates.io to update
        echo "Waiting 30 seconds for crates.io to update..."
        sleep 30
    else
        echo "✗ Failed to publish $crate_name@$version"
        exit 1
    fi
    
    cd - > /dev/null
}

# Get workspace information
echo "Analyzing workspace structure..."

if [ -f "Cargo.toml" ]; then
    # Check if this is a workspace or a single crate
    if cargo metadata --format-version 1 --no-deps | jq -e '.workspace_root' > /dev/null 2>&1; then
        echo "Detected workspace project"
        
        # Get all workspace members with their metadata
        workspace_info=$(cargo metadata --format-version 1 --no-deps)
        workspace_members=$(echo "$workspace_info" | jq -r '.workspace_members[]')
        
        # Create arrays to store crate information
        declare -a crate_names
        declare -a crate_paths
        declare -a crate_versions
        
        # Collect information about each workspace member
        for member in $workspace_members; do
            crate_name=$(echo "$member" | cut -d' ' -f1)
            crate_version=$(echo "$member" | cut -d' ' -f2 | sed 's/[()]//g')
            
            # Find the manifest path for this crate
            manifest_path=$(echo "$workspace_info" | jq -r ".packages[] | select(.name == \"$crate_name\") | .manifest_path")
            crate_path=$(dirname "$manifest_path")
            
            crate_names+=("$crate_name")
            crate_paths+=("$crate_path")
            crate_versions+=("$crate_version")
            
            echo "Found workspace member: $crate_name@$crate_version at $crate_path"
        done
        
        # Publish crates in dependency order
        # We'll use a simple approach: try to publish each crate, and retry failed ones
        max_attempts=3
        attempt=1
        
        while [ $attempt -le $max_attempts ]; do
            echo "Publication attempt $attempt of $max_attempts"
            
            published_this_round=false
            
            for i in "${!crate_names[@]}"; do
                crate_name="${crate_names[$i]}"
                crate_path="${crate_paths[$i]}"
                crate_version="${crate_versions[$i]}"
                
                # Skip if already published
                if is_published "$crate_name" "$crate_version"; then
                    continue
                fi
                
                # Try to publish
                echo "Attempting to publish $crate_name..."
                if publish_crate "$crate_path" "$crate_name" "$crate_version"; then
                    published_this_round=true
                else
                    echo "Failed to publish $crate_name, will retry in next round"
                fi
            done
            
            # If we didn't publish anything this round, we're either done or stuck
            if [ "$published_this_round" = false ]; then
                # Check if all crates are published
                all_published=true
                for i in "${!crate_names[@]}"; do
                    if ! is_published "${crate_names[$i]}" "${crate_versions[$i]}"; then
                        all_published=false
                        break
                    fi
                done
                
                if [ "$all_published" = true ]; then
                    echo "✓ All crates successfully published!"
                    break
                else
                    echo "✗ Some crates failed to publish after $max_attempts attempts"
                    exit 1
                fi
            fi
            
            attempt=$((attempt + 1))
        done
        
    else
        echo "Detected single crate project"
        
        # Single crate - get its information
        crate_info=$(cargo metadata --format-version 1 --no-deps)
        crate_name=$(echo "$crate_info" | jq -r '.packages[0].name')
        crate_version=$(echo "$crate_info" | jq -r '.packages[0].version')
        
        echo "Publishing single crate: $crate_name@$crate_version"
        publish_crate "." "$crate_name" "$crate_version"
    fi
else
    echo "Error: No Cargo.toml found in current directory"
    exit 1
fi

echo "✓ Publication process completed successfully!"