#!/usr/bin/env python3
"""Compare OpenAPI spec endpoints to implemented SDK service routes, grouped by tag."""

from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    print(
        "PyYAML is required. Install it with: pip install pyyaml",
        file=sys.stderr,
    )
    sys.exit(2)

ROOT = Path(__file__).resolve().parent.parent
SPEC_PATH = ROOT / "api_spec" / "prime-public-api-spec.json"
SERVICES_DIR = ROOT / "src" / "services"

TAG_TO_MODULE: dict[str, str] = {
    "Activities": "activities",
    "Address Book": "address_book",
    "Advanced Transfer": "advanced_transfers",
    "Allocations": "allocations",
    "Assets": "assets",
    "Balances": "balances",
    "Commission": "commission",
    "Financing": "financing",
    "Futures": "futures",
    "Invoice": "invoices",
    "Onchain Address Book": "onchain_address_groups",
    "Orders": "orders",
    "Payment Methods": "payment_methods",
    "Portfolios": "portfolios",
    "Positions": "positions",
    "Products": "products",
    "Staking": "staking",
    "Transactions": "transactions",
    "Travel Rule": "travel_rule",
    "Users": "users",
    "Wallets": "wallets",
}

HTTP_METHODS = frozenset({"get", "post", "put", "delete", "patch"})

PATH_FORMAT_ASSIGN_RE = re.compile(
    r'let\s+path\s*=\s*format!\(\s*\n?\s*"([^"]+)"',
    re.MULTILINE,
)
PATH_LITERAL_ASSIGN_RE = re.compile(r'let\s+path\s*=\s*"([^"]+)"\s*;')
HTTP_REQUEST_PATH_RE = re.compile(
    r"HttpRequest::new\(HttpMethod::(\w+),\s*&?path\b"
)
HTTP_REQUEST_INLINE_RE = re.compile(
    r'HttpRequest::new\(HttpMethod::(\w+),\s*format!\(\s*\n?\s*"([^"]+)"',
    re.MULTILINE,
)
HTTP_REQUEST_STRING_RE = re.compile(
    r'HttpRequest::new\(HttpMethod::(\w+),\s*"([^"]+)"'
)


def load_spec_endpoints() -> list[dict]:
    with SPEC_PATH.open(encoding="utf-8") as f:
        spec = yaml.safe_load(f)

    endpoints: list[dict] = []
    for path, methods in spec.get("paths", {}).items():
        for method, op in methods.items():
            if method not in HTTP_METHODS:
                continue
            tag = (op.get("tags") or ["Untagged"])[0]
            endpoints.append(
                {
                    "method": method.upper(),
                    "path": path,
                    "tag": tag,
                    "module": TAG_TO_MODULE.get(tag, tag.lower().replace(" ", "_")),
                }
            )
    return endpoints


def normalize_path_template(path: str) -> str:
    if path.startswith("/v1/"):
        return path[4:]
    if path.startswith("/v2/"):
        return "v2/" + path[4:]
    return path


def _is_path_param(segment: str) -> bool:
    return segment == "{}" or (
        segment.startswith("{") and segment.endswith("}")
    )


def path_templates_match(spec_path: str, impl_template: str) -> bool:
    spec_parts = normalize_path_template(spec_path).split("/")
    impl_parts = impl_template.split("/")
    if len(spec_parts) != len(impl_parts):
        return False
    for spec_part, impl_part in zip(spec_parts, impl_parts, strict=True):
        if _is_path_param(impl_part) or _is_path_param(spec_part):
            continue
        if spec_part != impl_part:
            return False
    return True


def extract_routes_from_service_content(content: str) -> list[dict]:
    """Extract HTTP method + path template pairs from a service.rs file."""
    routes: list[dict] = []

    for match in HTTP_REQUEST_INLINE_RE.finditer(content):
        routes.append({"method": match.group(1).upper(), "template": match.group(2)})

    for match in HTTP_REQUEST_STRING_RE.finditer(content):
        routes.append({"method": match.group(1).upper(), "template": match.group(2)})

    for match in HTTP_REQUEST_PATH_RE.finditer(content):
        method = match.group(1).upper()
        prefix = content[: match.start()]
        template = None

        format_assigns = list(PATH_FORMAT_ASSIGN_RE.finditer(prefix))
        if format_assigns:
            template = format_assigns[-1].group(1)
        else:
            literal_assigns = list(PATH_LITERAL_ASSIGN_RE.finditer(prefix))
            if literal_assigns:
                template = literal_assigns[-1].group(1)

        if template:
            routes.append({"method": method, "template": template})

    return routes


def extract_implemented_routes() -> dict[str, list[dict]]:
    routes_by_module: dict[str, list[dict]] = {}

    if not SERVICES_DIR.is_dir():
        return routes_by_module

    for entry in SERVICES_DIR.iterdir():
        if not entry.is_dir():
            continue
        service_file = entry / "service.rs"
        if not service_file.is_file():
            continue

        module_name = entry.name
        content = service_file.read_text(encoding="utf-8")
        routes_by_module[module_name] = extract_routes_from_service_content(content)

    return routes_by_module


def endpoint_covered(endpoint: dict, routes_by_module: dict[str, list[dict]]) -> bool:
    routes = routes_by_module.get(endpoint["module"], [])
    return any(
        route["method"].upper() == endpoint["method"]
        and path_templates_match(endpoint["path"], route["template"])
        for route in routes
    )


def main() -> int:
    spec_endpoints = load_spec_endpoints()
    implemented = extract_implemented_routes()

    by_tag: dict[str, list[dict]] = {}
    for endpoint in spec_endpoints:
        by_tag.setdefault(endpoint["tag"], []).append(endpoint)

    missing_by_tag: dict[str, list[dict]] = {}
    covered = 0
    for endpoint in spec_endpoints:
        if endpoint_covered(endpoint, implemented):
            covered += 1
        else:
            missing_by_tag.setdefault(endpoint["tag"], []).append(endpoint)

    total = len(spec_endpoints)
    print("Prime SDK endpoint coverage")
    print(f"Spec: {SPEC_PATH.relative_to(ROOT)}")
    print(f"Overall: {covered}/{total} ({100.0 * covered / total:.1f}%)")
    print()

    for tag in sorted(by_tag):
        tag_total = len(by_tag[tag])
        tag_missing = missing_by_tag.get(tag, [])
        tag_covered = tag_total - len(tag_missing)
        status = "OK" if not tag_missing else "MISSING"
        module = TAG_TO_MODULE.get(tag, tag.lower().replace(" ", "_"))
        print(f"[{status}] {tag} ({module}) — {tag_covered}/{tag_total}")
        for endpoint in sorted(tag_missing, key=lambda e: (e["method"], e["path"])):
            print(f"    {endpoint['method']} {endpoint['path']}")

    if any(missing_by_tag.values()):
        print()
        print(f"Missing {total - covered} endpoint(s).")
        return 1

    print()
    print(f"All {total} endpoints are covered.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
