# FutureStack Lab

**Polyglot Developer Toolkit · Portfolio Project**

FutureStack Lab is a learning-focused portfolio project that demonstrates how different programming languages can solve different software-engineering problems.

It complements my existing work in C#, PHP, JavaScript, C++, Unity, and web development by adding practical examples in **TypeScript, Python, Go, Java, and Rust**.

## Why these languages?

| Language | Mini-project | What it demonstrates |
|---|---|---|
| TypeScript | Typed service registry | Type safety, interfaces, async APIs |
| Python | Log analyzer | Automation, parsing, developer tooling |
| Go | Concurrent health checker | Concurrency, networking, backend tooling |
| Java | Priority job scheduler | OOP, collections, enterprise-style programming |
| Rust | Configuration validator | Safe systems programming, error handling |

## Project scenario

Imagine a small software team operating several services. This lab contains independent tools that help describe, inspect, monitor, schedule, and validate those services.

```text
future-stack-lab/
├── typescript/service-registry.ts
├── python/log_analyzer.py
├── go/health_checker.go
├── java/JobScheduler.java
└── rust/config_validator.rs
```

## Run the examples

### TypeScript

Requires Node.js and TypeScript.

```bash
npx tsc typescript/service-registry.ts --target ES2022 --module commonjs
node typescript/service-registry.js
```

### Python

```bash
python python/log_analyzer.py
```

### Go

```bash
go run go/health_checker.go
```

### Java

```bash
javac java/JobScheduler.java
java -cp java JobScheduler
```

### Rust

```bash
rustc rust/config_validator.rs -o config_validator
./config_validator
```

## What I would add next

- Unit tests for every language
- Docker-based local environment
- REST API version of the service registry
- CI workflow that builds every language
- PostgreSQL persistence
- Observability and structured metrics

## Portfolio purpose

This repository is intentionally small and readable. It is designed to show that I can learn beyond one language and understand where different tools fit in a modern software stack.

> Status: active learning project — examples are intentionally dependency-light and will be expanded over time.
