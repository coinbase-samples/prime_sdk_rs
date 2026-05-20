.PHONY: install-dev-tools generate-types clean-types fetch-spec remove-docs check-endpoints all

BREW_PREFIX := $(shell brew --prefix 2>/dev/null)
JAVA_HOME ?= $(BREW_PREFIX)/opt/openjdk/libexec/openjdk.jdk/Contents/Home
export JAVA_HOME
export PATH := $(JAVA_HOME)/bin:$(BREW_PREFIX)/opt/openapi-generator/bin:$(PATH)

# Install dev tools required for type generation and local development (macOS/Homebrew)
install-dev-tools:
	@command -v brew >/dev/null 2>&1 || { \
		echo "Error: Homebrew is required. Install it from https://brew.sh"; \
		exit 1; \
	}
	@echo "Installing OpenJDK and OpenAPI Generator..."
	brew install openjdk openapi-generator
	@command -v cargo >/dev/null 2>&1 || { \
		echo ""; \
		echo "Rust/Cargo not found. Install it from https://rustup.rs"; \
		exit 1; \
	}
	@command -v python3 >/dev/null 2>&1 || { \
		echo "Error: python3 is required for make check-endpoints"; \
		exit 1; \
	}
	@echo "Installing Python dependencies for endpoint coverage check..."
	pip3 install -q -r scripts/requirements.txt
	@echo ""
	@echo "Dev tools installed successfully."
	@echo ""
	@echo "Add the following to your shell profile (~/.zshrc or ~/.bashrc):"
	@echo '  export JAVA_HOME="$$(brew --prefix openjdk)/libexec/openjdk.jdk/Contents/Home"'
	@echo '  export PATH="$$JAVA_HOME/bin:$$(brew --prefix openapi-generator)/bin:$$PATH"'

fetch-spec:
	curl -o api_spec/prime-public-api-spec.json https://api.prime.coinbase.com/v1/openapi.yaml

# Generate Rust types from OpenAPI specification
generate-types:
	openapi-generator generate -i api_spec/prime-public-api-spec.json -g rust --global-property models -o api_spec/types
	cargo build

# Clean generated types
clean-types:
	rm -rf api_spec/types
	rm -rf src/generated

remove-docs:
	rm -rf api_spec/types/docs

check-endpoints:
	python3 scripts/check_endpoint_coverage.py

# Default target
all: generate-types remove-docs