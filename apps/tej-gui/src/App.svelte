<script>
  import SpeedGauge from "./SpeedGauge.svelte";
  import {
    startTest,
    phase,
    speedMbps,
    latencyMs,
    result,
    error,
    running,
  } from "./stores/speedtest.js";

  $: phaseLabel = {
    idle: "",
    starting: "Starting...",
    latency: "Measuring Latency",
    download: "Testing Download",
    upload: "Testing Upload",
    packet_loss: "Checking Packet Loss",
    done: "Complete",
  }[$phase] || "";

  $: gaugeLabel = $phase === "download" ? "Download" :
                  $phase === "upload" ? "Upload" : "";

  $: gaugeValue = ($phase === "download" || $phase === "upload")
    ? $speedMbps
    : $phase === "latency"
      ? $latencyMs
      : $result?.download?.mbps || 0;

  $: gaugeUnit = $phase === "latency" ? "ms" : "Mbps";
  $: gaugeMax = $phase === "latency" ? 200 : 1000;
</script>

<main>
  <h1>Tej</h1>
  <p class="subtitle">Honest Speed Test</p>

  <SpeedGauge
    value={gaugeValue}
    max={gaugeMax}
    label={gaugeLabel}
    unit={gaugeUnit}
  />

  {#if phaseLabel}
    <p class="phase">{phaseLabel}</p>
  {/if}

  <button on:click={startTest} disabled={$running} class="start-btn">
    {$running ? "Testing..." : "Start Test"}
  </button>

  {#if $error}
    <p class="error">{$error}</p>
  {/if}

  {#if $result}
    <div class="results">
      {#if $result.server_location}
        <div class="result-card">
          <span class="result-label">Server</span>
          <span class="result-value">{$result.server_location}</span>
        </div>
      {/if}
      {#if $result.latency}
        <div class="result-card">
          <span class="result-label">Latency</span>
          <span class="result-value">{$result.latency.avg_ms.toFixed(1)} ms</span>
        </div>
        <div class="result-card">
          <span class="result-label">Jitter</span>
          <span class="result-value">{$result.latency.jitter_ms.toFixed(1)} ms</span>
        </div>
      {/if}
      {#if $result.download}
        <div class="result-card highlight">
          <span class="result-label">Download</span>
          <span class="result-value">{$result.download.mbps.toFixed(2)} Mbps</span>
        </div>
      {/if}
      {#if $result.upload}
        <div class="result-card highlight">
          <span class="result-label">Upload</span>
          <span class="result-value">{$result.upload.mbps.toFixed(2)} Mbps</span>
        </div>
      {/if}
      {#if $result.packet_loss !== null && $result.packet_loss !== undefined}
        <div class="result-card">
          <span class="result-label">Packet Loss</span>
          <span class="result-value">{$result.packet_loss.toFixed(1)}%</span>
        </div>
      {/if}
    </div>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background: #0f0f1a;
    color: white;
  }

  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 24px 16px;
    min-height: 100vh;
    box-sizing: border-box;
  }

  h1 {
    margin: 0 0 4px 0;
    font-size: 24px;
    font-weight: 600;
  }

  .subtitle {
    color: #888;
    font-size: 12px;
    margin: 0 0 16px 0;
  }

  .phase {
    color: #aaa;
    font-size: 14px;
    margin: 8px 0;
  }

  .start-btn {
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 24px;
    padding: 12px 48px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    margin: 16px 0;
    transition: background 0.2s;
  }

  .start-btn:hover:not(:disabled) {
    background: #2563eb;
  }

  .start-btn:disabled {
    background: #1e3a5f;
    cursor: not-allowed;
  }

  .error {
    color: #ef4444;
    font-size: 14px;
  }

  .results {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
    width: 100%;
    max-width: 400px;
    margin-top: 8px;
  }

  .result-card {
    background: #1a1a2e;
    border-radius: 12px;
    padding: 12px 16px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .result-card.highlight {
    border: 1px solid #3b82f633;
  }

  .result-label {
    font-size: 11px;
    color: #888;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .result-value {
    font-size: 18px;
    font-weight: 600;
  }
</style>
