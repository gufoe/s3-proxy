# S3 Proxy

A simple HTTP server that proxies GET requests to S3 compatible backends.

## Usage

Create a toml configuration file with the following content:

```toml
# Server Configuration
workers = 1
host = "127.0.0.1"
port = 9234

# S3 Credentials
s3_host = "server.s3.com"
s3_bucket = "bucket-name"
s3_key = "ACCESS_KEY"
s3_secret = "SECRET_KEY"
s3_region = "us-east-1"
```

Finally run the server:

```bash
s3-proxy -c config.toml
```
