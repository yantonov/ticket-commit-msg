#!/bin/sh

GIT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null)
if [ $? -ne 0 ]; then
    echo '[ERROR] You are not inside git repository.'
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd $SCRIPT_DIR

if [ ! -f "${GIT_ROOT}/.git/hooks/commit-msg" ]; then
    HOOK_COUNT=$(ls -1 hooks | wc -l)
    if [ $HOOK_COUNT -gt 0 ]; then
        echo "These hooks are ready to be installed:"
        ls -1 hooks
        echo "Install hooks:"
        cp -irv hooks/* "$GIT_ROOT/.git/hooks"
    fi
else
    echo "commit hook is already exist, resolve conflict manually"
fi
