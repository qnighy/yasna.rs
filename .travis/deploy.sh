#!/bin/sh
set -ue

pip install ghp-import --user
$HOME/.local/bin/ghp-import -n target/doc

openssl aes-256-cbc -K $encrypted_93498dd8a6e0_key -iv $encrypted_93498dd8a6e0_iv -in .travis/deploy_key.enc -out .travis/deploy_key -d
chmod 600 .travis/deploy_key
ssh-keygen --help
ssh-add .travis/deploy_key < /dev/null
git push -qf ssh://git@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
