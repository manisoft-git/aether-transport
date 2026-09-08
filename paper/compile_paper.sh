#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

echo "[*] Generating figures if missing..."
python3 ../scripts/generate_charts.py || true

echo "[*] Checking for pdflatex / xelatex..."
if command -v pdflatex >/dev/null 2>&1; then
    pdflatex -interaction=nonstopmode aether_nsdi.tex
    bibtex aether_nsdi || true
    pdflatex -interaction=nonstopmode aether_nsdi.tex
    pdflatex -interaction=nonstopmode aether_nsdi.tex
    echo "[+] Successfully compiled aether_nsdi.pdf"
else
    echo "[!] pdflatex not found in local environment. PDF will be generated via CI or Dockerfile runner."
fi
