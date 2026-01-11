#!/bin/sh

SCRIPT="$(basename "$0")"

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
        cp -irv hooks/* "$GIT_ROOT/.git/hooks"
        echo "[OK] Commit hook is successfully installed"
    fi
else
    if [ "$1" = "--force" ]; then
        if [ -z "$(cat ${GIT_ROOT}/.git/hooks/commit-msg | grep 'GENERIC_COMMIT_HOOK_PLACEHOLDER' || echo '')" ]; then
            mkdir -p "${GIT_ROOT}/.git/hooks/commit-msg-hooks"
            mv "${GIT_ROOT}/.git/hooks/commit-msg" "${GIT_ROOT}/.git/hooks/commit-msg-hooks/0-commit-msg"
            cp -irv hooks/* "$GIT_ROOT/.git/hooks"
            echo "[OK] Commit hook is successfully installed"
        else
            echo "[WARN] Commit hook is already exist, if you want to modify, do it manually"    
        fi    
    else
        echo "[WARN] Commit hook is already exist, if you want to modify, do it manually"
        echo "If you want to force general commit hook use --force flag"
        echo "${SCRIPT} --force"
        echo "Be careful with this action. Think twice! Manual editing may be needed for the hook configuration."
        exit 1
    fi
fi
