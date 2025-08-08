# justfile (root)

# Print exact binary versions — spec's "just env"
env:
    @echo "rust: $(rustc --version)"
    @echo "forge: $(forge --version)"
    @echo "node: $(node --version)"
    @echo "just: $(just --version)"

# Clean artefacts
clean:
    rm -rf ./target ./contracts/out ./contracts/cache
