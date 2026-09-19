from collections import Counter
from dataclasses import dataclass
from datetime import datetime
from typing import Iterable


@dataclass(frozen=True)
class LogEntry:
    timestamp: datetime
    level: str
    service: str
    message: str


def parse_line(line: str) -> LogEntry:
    timestamp, level, service, message = line.strip().split("|", maxsplit=3)
    return LogEntry(
        timestamp=datetime.fromisoformat(timestamp),
        level=level.upper(),
        service=service,
        message=message,
    )


def analyze(lines: Iterable[str]) -> None:
    entries = [parse_line(line) for line in lines if line.strip()]
    levels = Counter(entry.level for entry in entries)
    services = Counter(entry.service for entry in entries)

    print("Log level counts:")
    for level, count in sorted(levels.items()):
        print(f"  {level}: {count}")

    print("\nEvents by service:")
    for service, count in services.most_common():
        print(f"  {service}: {count}")

    errors = [entry for entry in entries if entry.level == "ERROR"]
    print(f"\nErrors found: {len(errors)}")
    for entry in errors:
        print(f"  [{entry.timestamp}] {entry.service}: {entry.message}")


if __name__ == "__main__":
    sample_logs = [
        "2026-09-19T09:00:00|INFO|portfolio-api|Started successfully",
        "2026-09-19T09:01:05|WARN|notification-worker|Queue latency is elevated",
        "2026-09-19T09:02:10|ERROR|analytics-service|Database connection failed",
        "2026-09-19T09:03:15|INFO|portfolio-api|Request completed",
        "2026-09-19T09:04:20|ERROR|analytics-service|Retry limit reached",
    ]

    analyze(sample_logs)
