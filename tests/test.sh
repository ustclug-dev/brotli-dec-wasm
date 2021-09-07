#!/bin/bash
set -euo pipefail

(cd tests/webpack5 && npm run build)
