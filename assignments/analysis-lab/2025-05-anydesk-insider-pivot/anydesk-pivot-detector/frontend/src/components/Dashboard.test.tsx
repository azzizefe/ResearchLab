import { render, screen } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { MemoryRouter } from "react-router-dom";
import Dashboard from "./Dashboard";

// Mock the store and language hooks
vi.mock("../store/useStore", () => ({
  useStore: () => ({
    riskScore: 45,
    alerts: [
      { id: "1", title: "Test Alert", description: "Test Desc", severity: "High", timestamp: new Date().toISOString() }
    ],
  }),
}));

vi.mock("../store/useLanguage", () => ({
  useLanguage: () => ({
    t: (key: string) => key,
  }),
}));

// Mock Recharts since it doesn't play well with JSDOM
vi.mock("recharts", () => ({
  ResponsiveContainer: ({ children }: any) => <div>{children}</div>,
  PieChart: ({ children }: any) => <div>{children}</div>,
  Pie: ({ children }: any) => <div>{children}</div>,
  Cell: () => null,
}));

describe("Dashboard Component", () => {
  it("renders the dashboard title", () => {
    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>
    );
    expect(screen.getByText("dashboard")).toBeDefined();
  });

  it("displays the risk score", () => {
    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>
    );
    expect(screen.getByText("45")).toBeDefined();
  });

  it("displays the recent alerts", () => {
    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>
    );
    expect(screen.getByText("Test Alert")).toBeDefined();
  });
});
