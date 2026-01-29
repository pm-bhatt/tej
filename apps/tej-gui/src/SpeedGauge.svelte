<script>
  export let value = 0;
  export let max = 1000;
  export let label = "";
  export let unit = "Mbps";

  $: clampedValue = Math.min(value, max);
  $: angle = -135 + (clampedValue / max) * 270;
  $: needleX = 150 + 100 * Math.cos(((angle - 90) * Math.PI) / 180);
  $: needleY = 150 + 100 * Math.sin(((angle - 90) * Math.PI) / 180);

  // Generate tick marks
  const ticks = [];
  const tickCount = 10;
  for (let i = 0; i <= tickCount; i++) {
    const tickAngle = -135 + (i / tickCount) * 270;
    const rad = ((tickAngle - 90) * Math.PI) / 180;
    const innerR = 108;
    const outerR = 120;
    ticks.push({
      x1: 150 + innerR * Math.cos(rad),
      y1: 150 + innerR * Math.sin(rad),
      x2: 150 + outerR * Math.cos(rad),
      y2: 150 + outerR * Math.sin(rad),
      label: Math.round((i / tickCount) * max),
      labelX: 150 + 95 * Math.cos(rad),
      labelY: 150 + 95 * Math.sin(rad),
    });
  }
</script>

<div class="gauge-container">
  {#if label}
    <div class="gauge-label">{label}</div>
  {/if}

  <svg viewBox="0 0 300 200" class="gauge-svg">
    <!-- Background arc -->
    <path
      d="M 30 220 A 120 120 0 1 1 270 220"
      fill="none"
      stroke="#2a2a3e"
      stroke-width="20"
      stroke-linecap="round"
    />

    <!-- Value arc -->
    {#if clampedValue > 0}
      <path
        d="M 30 220 A 120 120 0 {angle > 0 ? 1 : 0} 1 {needleX} {needleY}"
        fill="none"
        stroke="url(#gaugeGradient)"
        stroke-width="20"
        stroke-linecap="round"
      />
    {/if}

    <!-- Gradient -->
    <defs>
      <linearGradient id="gaugeGradient" x1="0%" y1="0%" x2="100%" y2="0%">
        <stop offset="0%" style="stop-color:#22c55e" />
        <stop offset="50%" style="stop-color:#eab308" />
        <stop offset="100%" style="stop-color:#ef4444" />
      </linearGradient>
    </defs>

    <!-- Tick marks -->
    {#each ticks as tick}
      <line
        x1={tick.x1}
        y1={tick.y1}
        x2={tick.x2}
        y2={tick.y2}
        stroke="#666"
        stroke-width="2"
      />
      <text
        x={tick.labelX}
        y={tick.labelY}
        text-anchor="middle"
        dominant-baseline="central"
        fill="#888"
        font-size="10"
      >
        {tick.label}
      </text>
    {/each}

    <!-- Needle -->
    <line
      x1="150"
      y1="150"
      x2={needleX}
      y2={needleY}
      stroke="white"
      stroke-width="3"
      stroke-linecap="round"
      style="transition: all 0.3s ease-out"
    />
    <circle cx="150" cy="150" r="6" fill="white" />

    <!-- Center value -->
    <text x="150" y="180" text-anchor="middle" fill="white" font-size="28" font-weight="bold">
      {value.toFixed(1)}
    </text>
    <text x="150" y="196" text-anchor="middle" fill="#888" font-size="12">
      {unit}
    </text>
  </svg>
</div>

<style>
  .gauge-container {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .gauge-label {
    color: #aaa;
    font-size: 14px;
    text-transform: uppercase;
    letter-spacing: 2px;
    margin-bottom: 4px;
  }

  .gauge-svg {
    width: 100%;
    max-width: 300px;
  }
</style>
