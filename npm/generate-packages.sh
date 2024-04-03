bin="rm-rs"

export node_version=$(node -p "require('./$bin/package.json').version")

function generate_package() {
    export node_os=$1
    export node_arch=$2

    # note: use 'windows' as OS name instead of 'win32' to avoid spam detection
    if [ "$node_os" = "win32" ]; then
        export node_pkg="${bin}-windows-${node_arch}"
    else
        export node_pkg="${bin}-${node_os}-${node_arch}"
    fi

    mkdir -p "${node_pkg}/bin"

    envsubst < template.package.json > "${node_pkg}/package.json"
}

generate_package darwin arm64
generate_package darwin x64
generate_package linux arm64
generate_package linux x64
generate_package win32 arm64
generate_package win32 x64
