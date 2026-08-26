Simple domain lookup

### Debug

Build and run docker container

```bash
./run -b
```

### Configuration

Rename `.env.example` to `.env` and fill in the values.
You can get them from [whoisjson.com](https://whoisjson.com/) or similar service.

### Run

Inside a docker container run

```bash
cargo run --release
```

Run
```bash
domain_lookup
```

### Usage

Domain WHOIS, NS and subdomain lookup
```bash
curl -X GET http://localhost:3000/info?domain=google.com
```

Domain availability
```bash
curl -X GET http://localhost:3000/lookup?domain=google.com
```

Domain WHOIS, DNS, subdomain lookup or SSL certificate check
```bash
curl -X GET http://localhost:3000/whois?domain=google.com
curl -X GET http://localhost:3000/ns-lookup?domain=google.com
curl -X GET http://localhost:3000/subdomain?domain=google.com
curl -X GET http://localhost:3000/ssl-check?domain=google.com
```

Health check
```bash
curl -X GET http://localhost:3000/health
```
