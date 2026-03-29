# Monitoring and Alerting

## Overview

This document outlines how to monitor the Stellar Tip Jar backend, track performance, and set up alerting for production environments.

---

## Key Metrics

### Application Metrics

- Request rate (requests/second)
- Response time (p50, p95, p99)
- Error rate (% of failed requests)
- Active API connections

---

### System Metrics

- CPU usage (%)
- Memory usage (%)
- Disk usage (%)
- Network I/O

---

### Database Metrics

- Active connections
- Query latency
- Slow queries
- Connection pool usage

---

### Blockchain Metrics

- Stellar Horizon API latency
- Transaction success rate
- Failed transaction count

---

## Logging

### Log Sources

- Application logs (stdout/stderr)
- System logs (systemd / journald)

### Log Levels

- ERROR — Critical failures
- WARN — Potential issues
- INFO — Normal operations
- DEBUG — Detailed diagnostics

### Recommended Setup

- Use `journalctl` for systemd-based deployments:
  ```bash
  journalctl -u tipjar-backend -f
  ```

- Optionally:
 - Centralize logs using ELK stack or Loki
 - Retain logs for at least 7–30 days

## Monitoring Tools (Recommended)

### Basic Setup
If no monitoring stack is installed:
- Use:
 - `top` / `htop` (CPU & memory)
- `df -h` (disk usage)
- `netstat` or `ss` (network)

## Advanced Setup (Optional)

### Prometheus (Metrics scraping)
Example config:
```YAML
scrape_configs:
  - job_name: 'tipjar-backend'
    static_configs:
      - targets: ['localhost:8000']
```


### Grafana Dashboards
Recommended dashboards:
- API performance
- Database performance
- System health

## Alerting Rules

### Critical Alerts (Immediate Response)
- Service down for > 1 minute
- Error rate > 5%
- Database unavailable
- Disk usage > 90%

### Warning Alerts
- Error rate > 1%
- Response time p95 > 1s
- Memory usage > 80%
- High API latency

### Alert Channels
- PagerDuty (critical alerts)
- Slack/Discord (warnings)
- Email (notifications)

### Health Checks
Ensure availability of:
- API health endpoint (e.g., /health)
- Database connectivity check

## Monitoring Checklist
 - Logs are accessible
 - Metrics are tracked
 - Alerts configured
 - Health checks working

