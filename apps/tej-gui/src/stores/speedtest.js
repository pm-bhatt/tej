import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const phase = writable("idle");
export const speedMbps = writable(0);
export const progress = writable(0);
export const latencyMs = writable(0);
export const result = writable(null);
export const error = writable(null);
export const running = writable(false);
export const history = writable([]);

let unlisten = null;

export async function startTest() {
  running.set(true);
  phase.set("starting");
  speedMbps.set(0);
  progress.set(0);
  latencyMs.set(0);
  result.set(null);
  error.set(null);

  if (unlisten) {
    unlisten();
  }

  unlisten = await listen("speed-test-progress", (event) => {
    const data = event.payload;
    phase.set(data.phase);
    progress.set(data.progress);
    if (data.speed_mbps !== null) {
      speedMbps.set(data.speed_mbps);
    }
    if (data.latency_ms !== null) {
      latencyMs.set(data.latency_ms);
    }
  });

  try {
    const res = await invoke("start_speed_test", { connections: 6 });
    result.set(res);
    history.update((h) => [...h, res].slice(-100));
  } catch (e) {
    error.set(e.toString());
  } finally {
    running.set(false);
    phase.set("idle");
    if (unlisten) {
      unlisten();
      unlisten = null;
    }
  }
}
