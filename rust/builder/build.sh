#!/usr/bin/env sh

export RUSTFLAGS="-C target-feature=-crt-static"
if [ -f "${SRC_PKG}"/Cargo.lock ]; then
    echo "Building rust cargo project in $SRC_PKG"
else
  echo "Not a rust cargo project to build"
  exit 1
fi

echo "0. use deploy dir $DEPLOY_PKG"
echo "1. go into $SRC_PKG"
cd "$SRC_PKG"
echo "2. now in $(pwd)"

echo "3. try to get target_dir"
target_dir=$(cargo metadata --format-version 1 | jq ".target_directory" | sed 's/"//g')
echo "4. found target_dir: $target_dir"

echo "5. try to get project_name"
project_name=$(cargo metadata --format-version 1  | jq ".workspace_members[0]" | cut -d ' ' -f 1 | sed 's/"//g' | sed 's/-/_/g')
echo "6. found project_name: $project_name"

echo "7. start cargo build"
cargo build --release --verbose
echo "8. end cargo build"

echo "9. ls target_dir/release"
ls -l ${target_dir}/release/
echo "10. end ls target_dir"

echo "11. ls deploy pkg $DEPLOY_PKG"
ls -l $DEPLOY_PKG
echo "12. end ls deploy pkg"

echo "13. rename by copy => ${target_dir}/release/lib${project_name}".so "${target_dir}/release/handler.so"
cp "${target_dir}/release/lib${project_name}".so "${target_dir}/release/handler.so"

echo "14. copy ${target_dir}/release/lib${project_name}".so "${DEPLOY_PKG}/"
cp "${target_dir}/release/lib${project_name}".so "${DEPLOY_PKG}/handler.so"

echo "15. ls deploy pkg $DEPLOY_PKG"
ls -l $DEPLOY_PKG
echo "16. end ls deploy pkg"