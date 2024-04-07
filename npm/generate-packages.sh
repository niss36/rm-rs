bin="rm-rs"

export node_version=$(node -p "require('./$bin/package.json').version")

function generate_package() {
    export node_os=$1
    export node_arch=$2

    if [ "$node_os" = "win32" ]; then
        # note: use 'windows' instead of 'win32' in package name to avoid spam detection
        export node_pkg="${bin}-windows-${node_arch}"
        export node_bin="bin/${bin}.exe"
    else
        export node_pkg="${bin}-${node_os}-${node_arch}"
        export node_bin="bin/${bin}"
    fi

    mkdir -p "${node_pkg}/bin"

    envsubst < template.package.json > "${node_pkg}/package.json"

    cp ../README.md "${node_pkg}"
}

generate_package darwin arm64
generate_package darwin x64
generate_package linux arm64
generate_package linux x64
generate_package win32 arm64
generate_package win32 x64
