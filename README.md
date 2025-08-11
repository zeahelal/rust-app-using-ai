# App Using AI - Rust Full-Stack Template

This repository implements a secure, modern full-stack Rust application as outlined in `app-plan.pdf`.

- Backend: Axum + SQLx + JWT + Argon2
- Frontend: Yew (WASM) + Trunk with dark/light theming
- Data: PostgreSQL, Redis (cache)
- Docker Compose for local services

## Quickstart

1) Copy env

```bash
cp .env.example .env
```

2) Start services (Postgres, Redis)

```bash
docker compose -f docker/docker-compose.yml up -d postgres redis
```

3) Build backend and frontend

```bash
./scripts/build.sh
```

4) Run backend locally

```bash
cargo run -p backend
```

5) Run frontend (requires trunk)

```bash
trunk serve --config frontend/Trunk.toml
```

## Structure

- `backend/` — Axum server, security middleware, DB access
- `frontend/` — Yew app, routing, theming
- `shared/` — shared models and DTOs
- `docker/` — docker-compose and Dockerfiles
- `scripts/` — build/test/deploy helpers
- `.env.example` — example environment variables
