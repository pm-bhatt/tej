// Tej Web Speed Test - JavaScript frontend
// Loads WASM module and handles UI updates

// WASM module will be loaded from pkg/tej_core.js after wasm-pack build
let wasmModule = null;
let isRunning = false;

// DOM elements
const elements = {
    wasmLoading: document.getElementById('wasm-loading'),
    appContent: document.getElementById('app-content'),
    startBtn: document.getElementById('start-btn'),
    gaugeValue: document.getElementById('gauge-value'),
    gaugeUnit: document.getElementById('gauge-unit'),
    gaugeLabel: document.getElementById('gauge-label'),
    phaseIndicator: document.getElementById('phase-indicator'),
    errorMessage: document.getElementById('error-message'),
    results: document.getElementById('results'),
    resultsGrid: document.getElementById('results-grid'),
    canvas: document.getElementById('speed-gauge')
};

// Phase labels
const phaseLabels = {
    idle: 'Ready',
    starting: 'Starting...',
    latency: 'Measuring Latency',
    download: 'Testing Download',
    upload: 'Testing Upload',
    packet_loss: 'Checking Packet Loss',
    done: 'Complete'
};

// Initialize the app
async function init() {
    try {
        // Load WASM module
        // Note: After wasm-pack build, this path should point to the generated pkg
        const wasm = await import('./pkg/tej_core.js');
        await wasm.default();
        wasmModule = wasm;
        
        // Hide loading, show app
        elements.wasmLoading.classList.add('hidden');
        elements.appContent.classList.remove('hidden');
        
        // Initialize gauge
        drawGauge(0);
        
        // Bind start button
        elements.startBtn.addEventListener('click', startTest);
        
    } catch (err) {
        console.error('Failed to load WASM:', err);
        elements.wasmLoading.innerHTML = `
            <p style="color: #ef4444;">Failed to load speed test engine.</p>
            <p style="font-size: 12px; color: #666;">${err.message}</p>
        `;
    }
}

// Draw the speed gauge
function drawGauge(percentage) {
    const canvas = elements.canvas;
    const ctx = canvas.getContext('2d');
    const centerX = canvas.width / 2;
    const centerY = canvas.height - 20;
    const radius = 100;
    
    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Background arc
    ctx.beginPath();
    ctx.arc(centerX, centerY, radius, Math.PI, 0);
    ctx.lineWidth = 12;
    ctx.strokeStyle = '#1a1a2e';
    ctx.stroke();
    
    // Progress arc
    const startAngle = Math.PI;
    const endAngle = Math.PI + (Math.PI * percentage);
    
    // Gradient for progress
    const gradient = ctx.createLinearGradient(0, centerY - radius, canvas.width, centerY);
    gradient.addColorStop(0, '#3b82f6');
    gradient.addColorStop(1, '#8b5cf6');
    
    ctx.beginPath();
    ctx.arc(centerX, centerY, radius, startAngle, endAngle);
    ctx.lineWidth = 12;
    ctx.lineCap = 'round';
    ctx.strokeStyle = gradient;
    ctx.stroke();
    
    // Tick marks
    for (let i = 0; i <= 10; i++) {
        const angle = Math.PI + (Math.PI * i / 10);
        const innerRadius = radius - 20;
        const outerRadius = radius - 8;
        
        const x1 = centerX + Math.cos(angle) * innerRadius;
        const y1 = centerY + Math.sin(angle) * innerRadius;
        const x2 = centerX + Math.cos(angle) * outerRadius;
        const y2 = centerY + Math.sin(angle) * outerRadius;
        
        ctx.beginPath();
        ctx.moveTo(x1, y1);
        ctx.lineTo(x2, y2);
        ctx.lineWidth = 2;
        ctx.strokeStyle = i <= percentage * 10 ? '#fff' : '#444';
        ctx.stroke();
    }
}

// Progress callback for WASM
function onProgress(phase, speedMbps, progress, latencyMs) {
    // Update phase label
    const phaseText = phaseLabels[phase] || phase;
    elements.gaugeLabel.textContent = phaseText;
    
    // Update phase indicator
    if (phase !== 'idle' && phase !== 'done') {
        elements.phaseIndicator.textContent = phaseText + '...';
    } else {
        elements.phaseIndicator.textContent = '';
    }
    
    // Update gauge
    if (speedMbps !== null && speedMbps > 0) {
        elements.gaugeValue.textContent = speedMbps.toFixed(2);
        elements.gaugeUnit.textContent = 'Mbps';
        
        // Update gauge visualization
        const maxSpeed = 1000; // Assume 1 Gbps max for gauge
        const percentage = Math.min(speedMbps / maxSpeed, 1);
        drawGauge(percentage);
    } else if (latencyMs !== null && latencyMs > 0) {
        elements.gaugeValue.textContent = latencyMs.toFixed(1);
        elements.gaugeUnit.textContent = 'ms';
        drawGauge(latencyMs / 200); // 200ms max for latency
    }
}

// Start speed test
async function startTest() {
    if (isRunning) return;
    
    isRunning = true;
    elements.startBtn.disabled = true;
    elements.startBtn.textContent = 'Testing...';
    elements.errorMessage.classList.add('hidden');
    elements.results.classList.add('hidden');
    
    // Reset display
    elements.gaugeValue.textContent = '0';
    elements.gaugeUnit.textContent = 'Mbps';
    drawGauge(0);
    
    try {
        // Create JS callback function for progress
        const progressCallback = Function('phase', 'speed', 'progress', 'latency', `
            window.onTestProgress(phase, speed, progress, latency);
        `);
        
        // Make callback available globally
        window.onTestProgress = onProgress;
        
        // Run the test
        const result = await wasmModule.runSpeedTestWithProgress(progressCallback);
        
        // Display results
        displayResults(result);
        
        // Save to localStorage
        saveResult(result);
        
    } catch (err) {
        console.error('Test failed:', err);
        elements.errorMessage.textContent = 'Test failed: ' + (err.message || err);
        elements.errorMessage.classList.remove('hidden');
    } finally {
        isRunning = false;
        elements.startBtn.disabled = false;
        elements.startBtn.textContent = 'Start Test';
        elements.gaugeLabel.textContent = phaseLabels.idle;
        elements.phaseIndicator.textContent = '';
    }
}

// Display test results
function displayResults(result) {
    elements.results.classList.remove('hidden');
    
    const cards = [];
    
    if (result.server_location) {
        cards.push({
            label: 'Server',
            value: result.server_location,
            highlight: false
        });
    }
    
    if (result.latency) {
        cards.push({
            label: 'Latency',
            value: result.latency.avg_ms.toFixed(1) + ' ms',
            highlight: false
        });
        cards.push({
            label: 'Jitter',
            value: result.latency.jitter_ms.toFixed(1) + ' ms',
            highlight: false
        });
    }
    
    if (result.download) {
        cards.push({
            label: 'Download',
            value: result.download.mbps.toFixed(2) + ' Mbps',
            highlight: true
        });
    }
    
    if (result.upload) {
        cards.push({
            label: 'Upload',
            value: result.upload.mbps.toFixed(2) + ' Mbps',
            highlight: true
        });
    }
    
    if (result.packet_loss !== null && result.packet_loss !== undefined) {
        cards.push({
            label: 'Packet Loss',
            value: result.packet_loss.toFixed(1) + '%',
            highlight: false
        });
    }
    
    elements.resultsGrid.innerHTML = cards.map(card => `
        <div class="result-card ${card.highlight ? 'highlight' : ''}">
            <div class="result-label">${card.label}</div>
            <div class="result-value">${card.value}</div>
        </div>
    `).join('');
}

// Save result to localStorage
function saveResult(result) {
    try {
        const history = JSON.parse(localStorage.getItem('tej-history') || '[]');
        history.push({
            ...result,
            timestamp: new Date().toISOString()
        });
        // Keep last 50 results
        if (history.length > 50) {
            history.shift();
        }
        localStorage.setItem('tej-history', JSON.stringify(history));
    } catch (e) {
        console.warn('Failed to save result:', e);
    }
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', init);
