<div style="text-align: center;">
<h1>Rust Authentication service boilerplate</h1>

[![🦀 Rust](https://github.com/hhertout/rs_auth_ms_boilerplate/actions/workflows/rust.yml/badge.svg)](https://github.com/hhertout/rs_auth_ms_boilerplate/actions/workflows/rust.yml)

</div>


# Features
-[x] User management
-[x] JWT Authentication with cookie
-[x] JWR Authentication with token
-[x] CSRF token provider
-[x] Docker integration 
-[ ] OAuth

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