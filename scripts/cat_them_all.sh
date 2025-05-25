#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
# Treat unset variables as an error.
# Pipelines return status of the last command to exit with non-zero status,
# or zero if all commands exit successfully.
set -euo pipefail

FILES="
src/main.rs
src/config/mod.rs
src/config/loader.rs
src/config/app.rs
src/config/server.rs
src/config/auth.rs
src/config/route.rs
src/logging/mod.rs
src/logging/config.rs
src/logging/console.rs
src/server/mod.rs
src/server/error.rs
src/server/proxy.rs
src/server/routes.rs
"


# Function to process each file
process_file() {
    local file="$1"
    # Use realpath to ensure consistent relative paths
    local relative_path
    relative_path="$(realpath --relative-to="." "$file")"

    # Skip if file is not readable or is empty
    if [ ! -r "$file" ] || [ ! -s "$file" ]; then
        return
    fi

    # Use 'file' command to determine the mime type.
    # -b (--brief): Do not prepend filename to output lines.
    # Check if the mime type starts with "text/". If it DOES NOT, skip the file.
    if ! file -b --mime-type "$file" | grep -q '^text/'; then
        return
    fi

    printf "=== File: %s ===\n\n" "$relative_path"
    cat "$file"
    printf "\n\n"
}

# Main script logic
main() {
    for file in $FILES; do
        process_file "$file"
    done
}

# Run the main function
main

exit 0
