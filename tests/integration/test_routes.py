import os
import requests
import yaml
import pytest

# Load YAML test cases
with open(os.path.join(os.path.dirname(__file__), "route_test_cases.yaml")) as f:
    cases = yaml.safe_load(f)["tests"]

GATEWAY_BASE_URL = os.getenv("GATEWAY_URL", "http://localhost:8080")

@pytest.mark.parametrize("case", cases)
def test_route(case):
    url = GATEWAY_BASE_URL.rstrip("/") + case["path"]
    response = requests.get(url)
    assert response.status_code == 200, f"{case['name']} failed with HTTP {response.status_code}"

    json_data = response.json()
    actual_tag = json_data.get("server_tag", "<missing>")

    assert actual_tag == case["expected_tag"], (
        f"{case['name']} failed: expected '{case['expected_tag']}', got '{actual_tag}'"
    )
