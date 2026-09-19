type ServiceStatus = "healthy" | "degraded" | "offline";

interface Service {
  id: number;
  name: string;
  url: string;
  owner: string;
  status: ServiceStatus;
}

class ServiceRegistry {
  private services: Service[] = [];

  add(service: Service): void {
    if (this.services.some((item) => item.id === service.id)) {
      throw new Error(`Service id ${service.id} already exists.`);
    }
    this.services.push(service);
  }

  updateStatus(id: number, status: ServiceStatus): void {
    const service = this.services.find((item) => item.id === id);
    if (!service) {
      throw new Error(`Service id ${id} was not found.`);
    }
    service.status = status;
  }

  getHealthyServices(): Service[] {
    return this.services.filter((service) => service.status === "healthy");
  }

  summary(): Record<ServiceStatus, number> {
    return this.services.reduce(
      (totals, service) => {
        totals[service.status] += 1;
        return totals;
      },
      { healthy: 0, degraded: 0, offline: 0 }
    );
  }
}

const registry = new ServiceRegistry();

registry.add({
  id: 1,
  name: "portfolio-api",
  url: "https://example.com/api",
  owner: "platform",
  status: "healthy",
});

registry.add({
  id: 2,
  name: "notification-worker",
  url: "https://example.com/worker",
  owner: "backend",
  status: "degraded",
});

registry.add({
  id: 3,
  name: "analytics-service",
  url: "https://example.com/analytics",
  owner: "data",
  status: "offline",
});

registry.updateStatus(3, "healthy");

console.log("Healthy services:", registry.getHealthyServices());
console.log("Service summary:", registry.summary());
