#!/usr/bin/env bash

set -euxo pipefail

if [ -f ${SRC_PKG}/Cargo.lock ]; then
    echo "Building rust cargo project ${srcDir}"
    ln -sf ${SRC_PKG} ${srcDir}
else
  echo "Not a rust cargo project to build"
  exit 1
fi

cd ${srcDir}

metadata=$(cargo metadata --format-version 1)

target_dir=$(echo $metadata | jq ".target_directory" | sed 's/"//g')
project_name=$(echo $metadata | jq ".workspace_members[0]" | cut -d ' ' -f 1 | sed 's/"//g')

cargo build --release

cp "${target_dir}"/"${project_name}".so "${DEPLOY_PKG}"