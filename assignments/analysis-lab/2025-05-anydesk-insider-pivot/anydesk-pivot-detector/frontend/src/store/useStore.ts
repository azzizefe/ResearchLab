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
  riskScore: number;
  isMonitoring: boolean;
  addAlert: (alert: Alert) => void;
  setMonitoring: (status: boolean) => void;
  updateRiskScore: (score: number) => void;
}

export const useStore = create<AppState>((set) => ({
  alerts: [],
  riskScore: 0,
  isMonitoring: false,
  addAlert: (alert) => set((state) => ({ 
    alerts: [alert, ...state.alerts].slice(0, 100),
    riskScore: Math.min(100, state.riskScore + (alert.severity === "Critical" ? 20 : alert.severity === "High" ? 10 : 5))
  })),
  setMonitoring: (status) => set({ isMonitoring: status }),
  updateRiskScore: (score) => set({ riskScore: score }),
}));
