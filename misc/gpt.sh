#!/bin/sh

# Use OpenAI's API to translate win32 prototypes to retrowin32 Rust definitions.
# https://inuh.net/@evmar/112001414385042731
# The 'gpt' command here is https://github.com/evmar/gpt

set -e

cd ~/projects/gpt
source apikey.sh
exec ./gpt -server openai text -sys 'translate c to rust, following pattern from examples' -multi "$(<prompts)" -
