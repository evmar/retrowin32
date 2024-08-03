#!/bin/sh

# Use OpenAI's API to translate win32 prototypes to retrowin32 Rust definitions.
# https://inuh.net/@evmar/112001414385042731
# The 'ai' command here is https://github.com/evmar/ai

set -e

cd ~/projects/ai
source apikey.sh
exec ./ai -server openai text -sys 'translate c to rust, following pattern from examples' -multi "$(<prompts)" -
