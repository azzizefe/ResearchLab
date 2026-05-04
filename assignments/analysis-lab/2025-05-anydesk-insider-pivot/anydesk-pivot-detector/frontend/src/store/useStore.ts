import { create } from "zustand";

export interface Alert {
  id: string;
  timestamp: string;
  severity: "Low" | "Medium" | "High" | "Critical";
  title: string;
  description: string;
  source_module: string;
}

interface AppState {
  alerts: Alert[];
  logs: string[];
  riskScore: number;
  isMonitoring: boolean;
  addAlert: (alert: Alert) => void;
  addLog: (log: string) => void;
  setMonitoring: (status: boolean) => void;
  updateRiskScore: (score: number) => void;
}

export const useStore = create<AppState>((set) => ({
  alerts: [],
  logs: [],
  riskScore: 0,
  isMonitoring: false,
  addAlert: (alert) => set((state) => ({ 
    alerts: [alert, ...state.alerts].slice(0, 100),
    riskScore: Math.min(100, state.riskScore + (alert.severity === "Critical" ? 20 : alert.severity === "High" ? 10 : 5))
  })),
  addLog: (log) => set((state) => ({
    logs: [...state.logs, log].slice(-500)
  })),
  setMonitoring: (status) => set({ isMonitoring: status }),
  updateRiskScore: (score) => set({ riskScore: score }),
}));
