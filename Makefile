.PHONY: generate-types clean-types

# Generate Rust types from OpenAPI specification
generate-types:
	openapi-generator-cli generate -i api_spec/prime-public-api-spec.json -g rust --global-property models -o api_spec/types
	cargo build

# Clean generated types
clean-types:
	rm -rf api_spec/types
	rm -rf src/generated

remove-docs:
	rm -rf api_spec/types/docs

# Default target
all: generate-types remove-docs