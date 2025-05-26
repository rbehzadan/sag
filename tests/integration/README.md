# Integration Tests for API Gateway

This directory contains automated integration tests designed to validate the route matching and forwarding behavior of the API Gateway.

Each test sends a real HTTP request to the deployed gateway and asserts that the request was routed to the expected backend service, identified by a unique `server_tag`.

---

## 🧪 Test Description

- Test cases are defined in [`route_test_cases.yaml`](route_test_cases.yaml).
- Each test includes:
  - `name`: Description of the test
  - `path`: Path to request on the API Gateway
  - `expected_tag`: The expected backend `server_tag`

Example:

```yaml
- name: "Exact match"
  path: "/test/exact"
  expected_tag: "gws01"
````

---

## 🔧 Backend Echo Servers

This project uses [simple-http-echo-server](https://github.com/rbehzadan/simple-http-echo-server) as backend services.

A `docker-compose.yaml` is included in this directory to launch 5 echo server containers (`gws01` to `gws05`) behind a Traefik reverse proxy.

### Requirements

* `docker` and `docker-compose`
* Traefik must already be running and joined to an external Docker network named `traefik`
* The environment variable `DOMAIN` should be set (e.g., `lt03.behzadan.com`)

### Launch the backend services:

```bash
cd tests/integration
export DOMAIN=lt03.behzadan.com  # or your domain
docker-compose up -d
```

Each container will be reachable via:

* `https://gws01.${DOMAIN}`
* `https://gws02.${DOMAIN}`
* ...
* `https://gws05.${DOMAIN}`

Each returns a JSON body including a unique `server_tag`.

---

## 🚀 Running the Tests

> All commands below assume you're in the **project root**.

### 1. Create and activate a virtual environment

We recommend using a virtual environment to isolate Python dependencies:

```bash
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
````

You should now see `(venv)` in your shell prompt.

### 2. Install dependencies

Use the provided `requirements.txt` and log the output to `pip-log.txt`:

```bash
pip install -r requirements.txt --log pip-log.txt
```

### 3. Set the API Gateway base URL

Set this before running the tests:

```bash
export GATEWAY_URL=https://your-gateway-domain
```

### 4. Run the tests

```bash
pytest tests/integration -v
```

For more verbose failure output:

```bash
pytest tests/integration -v --tb=short --capture=tee-sys
```

---

## 📁 Directory Layout

```
tests/integration/
├── docker-compose.yaml     # Spins up echo servers
├── route_test_cases.yaml   # Test definitions
├── test_routes.py          # Python test runner
└── README.md               # This file
```

---

## 🔄 Extending the Tests

To add a new test:

1. Add a new entry to `route_test_cases.yaml`
2. Optionally map the new route to one of the echo servers
3. Run `pytest` again

---

✅ Happy Testing!
