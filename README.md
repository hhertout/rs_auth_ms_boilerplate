<div align="center">
<h1>Rust Authentication service boilerplate</h1>

[![🦀 Rust](https://github.com/hhertout/rs_auth_ms_boilerplate/actions/workflows/rust.yml/badge.svg)](https://github.com/hhertout/rs_auth_ms_boilerplate/actions/workflows/rust.yml)

</div>


# Features
1. [x] User management
2. [x] JWT Authentication with cookie
3. [x] JWR Authentication with token
4. [x] CSRF token provider
5. [x] Docker integration 
6. [ ] OAuth

# Specification

- Axum framework for the API and endpoint management
- sqlx for the database interface
- Work with Postgres (manageable with sqlx lib)

# Quick started

Simply run :

```bash
docker compose up -d
```

Then update the code as you want !

## Boilerplate tour

- All routes are configured in ```/src/api/mod.rs```

- Controller are defined in ```/src/controllers```

- Database connection is configured in ```/src/database```

- Database interface and query are set in ```/src/repository```

- Other stuff used by controllers for example is available in ```/src/services```
