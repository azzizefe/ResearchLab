import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useStore, Alert } from "../store/useStore";

export function useTauriEvents() {
  const { addAlert, addLog, setMonitoring, updateRiskScore } = useStore();

  useEffect(() => {
    let unlistenAlerts: (() => void) | undefined;
    let unlistenLogs: (() => void) | undefined;
    let unlistenRisk: (() => void) | undefined;

    const setupListeners = async () => {
      // 1. Listen for new alerts
      unlistenAlerts = await listen<Alert>("new-alert", (event) => {
        console.log("New alert received:", event.payload);
        addAlert(event.payload);
      });

      // 2. Listen for raw log entries
      unlistenLogs = await listen<string>("log-entry", (event) => {
        addLog(event.payload);
      });

      // 3. Listen for risk score updates
      unlistenRisk = await listen<number>("risk-update", (event) => {
        updateRiskScore(event.payload);
      });

      // 3. Automatically start monitoring when the hook mounts
      try {
        await invoke("start_monitor");
        setMonitoring(true);
        console.log("Monitoring started successfully");
      } catch (error) {
        console.error("Failed to start monitoring:", error);
      }
    };

    setupListeners();

    return () => {
      if (unlistenAlerts) unlistenAlerts();
      if (unlistenLogs) unlistenLogs();
      if (unlistenRisk) unlistenRisk();
    };
  }, [addAlert, setMonitoring]);
}
