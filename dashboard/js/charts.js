/**
 * Charts Module - Advanced Chart.js Configuration
 * Interactive charts with zoom, pan, custom tooltips and annotations
 */

const ChartsManager = {
    // Store chart instances
    charts: {},
    
    // Data storage
    data: {
        complexity: null,
        binet: null,
        golden: null
    },
    
    // Chart color schemes
    colors: {
        primary: '#6366f1',
        primaryLight: 'rgba(99, 102, 241, 0.2)',
        secondary: '#10b981',
        secondaryLight: 'rgba(16, 185, 129, 0.2)',
        accent: '#f59e0b',
        accentLight: 'rgba(245, 158, 11, 0.2)',
        danger: '#ef4444',
        dangerLight: 'rgba(239, 68, 68, 0.2)',
        purple: '#8b5cf6',
        purpleLight: 'rgba(139, 92, 246, 0.2)'
    },
    
    /**
     * Initialize charts
     */
    async init() {
        // Register zoom plugin if available
        if (typeof Chart !== 'undefined' && typeof ChartZoom !== 'undefined') {
            Chart.register(ChartZoom);
        }
        
        // Set global Chart.js defaults
        this.setChartDefaults();
        
        // Load data and create charts
        await this.loadAllData();
        
        // Bind reset zoom buttons
        this.bindResetButtons();
    },
    
    /**
     * Set global Chart.js defaults
     */
    setChartDefaults() {
        if (typeof Chart === 'undefined') return;
        
        const isDark = document.documentElement.getAttribute('data-theme') !== 'light';
        const textColor = isDark ? '#f1f5f9' : '#0f172a';
        const gridColor = isDark ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)';
        
        Chart.defaults.color = textColor;
        Chart.defaults.borderColor = gridColor;
        Chart.defaults.font.family = "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif";
        Chart.defaults.animation.duration = 800;
        Chart.defaults.animation.easing = 'easeOutQuart';
        Chart.defaults.responsive = true;
        Chart.defaults.maintainAspectRatio = false;
    },
    
    /**
     * Load all chart data
     */
    async loadAllData() {
        try {
            const [complexity, binet, golden] = await Promise.all([
                this.loadData('complexity_comparison.json'),
                this.loadData('binet_accuracy.json'),
                this.loadData('golden_ratio_convergence.json')
            ]);
            
            this.data.complexity = complexity;
            this.data.binet = binet;
            this.data.golden = golden;
            
            // Create charts
            this.createComplexityChart();
            this.createBinetChart();
            this.createGoldenChart();
            
        } catch (error) {
            console.error('Error loading chart data:', error);
        }
    },
    
    /**
     * Load JSON data file
     * @param {string} filename
     * @returns {Promise<Array>}
     */
    async loadData(filename) {
        try {
            const response = await fetch(`data/${filename}`);
            if (!response.ok) throw new Error(`HTTP ${response.status}`);
            return await response.json();
        } catch (error) {
            console.warn(`Could not load ${filename}:`, error);
            return [];
        }
    },
    
    /**
     * Get common chart options
     * @returns {Object}
     */
    getCommonOptions() {
        const isDark = document.documentElement.getAttribute('data-theme') !== 'light';
        const textColor = isDark ? '#f1f5f9' : '#0f172a';
        const textMuted = isDark ? '#94a3b8' : '#64748b';
        const gridColor = isDark ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)';
        
        return {
            responsive: true,
            maintainAspectRatio: false,
            interaction: {
                mode: 'index',
                intersect: false
            },
            plugins: {
                legend: {
                    display: true,
                    position: 'top',
                    labels: {
                        color: textColor,
                        usePointStyle: true,
                        pointStyle: 'circle',
                        padding: 20,
                        font: {
                            size: 12,
                            weight: '500'
                        }
                    }
                },
                tooltip: {
                    enabled: true,
                    backgroundColor: isDark ? 'rgba(15, 23, 42, 0.95)' : 'rgba(255, 255, 255, 0.95)',
                    titleColor: textColor,
                    bodyColor: textMuted,
                    borderColor: isDark ? 'rgba(99, 102, 241, 0.3)' : 'rgba(99, 102, 241, 0.2)',
                    borderWidth: 1,
                    cornerRadius: 8,
                    padding: 12,
                    boxPadding: 6,
                    usePointStyle: true,
                    titleFont: {
                        size: 14,
                        weight: '600'
                    },
                    bodyFont: {
                        size: 13
                    }
                },
                zoom: {
                    pan: {
                        enabled: true,
                        mode: 'xy',
                        modifierKey: null
                    },
                    zoom: {
                        wheel: {
                            enabled: true,
                            speed: 0.1
                        },
                        pinch: {
                            enabled: true
                        },
                        mode: 'xy'
                    },
                    limits: {
                        x: { min: 'original', max: 'original' },
                        y: { min: 'original', max: 'original' }
                    }
                }
            },
            scales: {
                x: {
                    grid: {
                        color: gridColor,
                        drawBorder: false
                    },
                    ticks: {
                        color: textMuted,
                        font: {
                            size: 11
                        }
                    },
                    title: {
                        display: true,
                        color: textColor,
                        font: {
                            size: 12,
                            weight: '500'
                        }
                    }
                },
                y: {
                    grid: {
                        color: gridColor,
                        drawBorder: false
                    },
                    ticks: {
                        color: textMuted,
                        font: {
                            size: 11
                        }
                    },
                    title: {
                        display: true,
                        color: textColor,
                        font: {
                            size: 12,
                            weight: '500'
                        }
                    }
                }
            }
        };
    },
    
    /**
     * Create Complexity Comparison Chart
     */
    createComplexityChart() {
        const canvas = document.getElementById('complexityChart');
        if (!canvas || !this.data.complexity || this.data.complexity.length === 0) return;
        
        const ctx = canvas.getContext('2d');
        const options = this.getCommonOptions();
        
        // Custom tooltip
        options.plugins.tooltip.callbacks = {
            title: (items) => `n = ${items[0].label}`,
            label: (context) => {
                const value = context.parsed.y;
                const method = context.dataset.label;
                return `${method}: ${value} ns`;
            },
            afterBody: (items) => {
                if (items.length >= 2) {
                    const iterative = items.find(i => i.dataset.label.includes('Iteratif'))?.parsed.y || 0;
                    const matrix = items.find(i => i.dataset.label.includes('Matrix'))?.parsed.y || 0;
                    if (iterative > 0 && matrix > 0) {
                        const ratio = (iterative / matrix).toFixed(2);
                        return [`\nRatio: ${ratio}x`];
                    }
                }
                return [];
            }
        };
        
        options.scales.x.title.text = 'Valeur de n';
        options.scales.y.title.text = 'Temps (ns)';
        
        this.charts.complexity = new Chart(ctx, {
            type: 'line',
            data: {
                labels: this.data.complexity.map(d => d.n),
                datasets: [
                    {
                        label: 'Iteratif O(n)',
                        data: this.data.complexity.map(d => d.iterative_ns),
                        borderColor: this.colors.accent,
                        backgroundColor: this.colors.accentLight,
                        borderWidth: 2,
                        fill: true,
                        tension: 0.4,
                        pointRadius: 4,
                        pointHoverRadius: 6,
                        pointBackgroundColor: this.colors.accent,
                        pointBorderColor: '#fff',
                        pointBorderWidth: 2
                    },
                    {
                        label: 'Matrix O(log n)',
                        data: this.data.complexity.map(d => d.matrix_ns),
                        borderColor: this.colors.secondary,
                        backgroundColor: this.colors.secondaryLight,
                        borderWidth: 2,
                        fill: true,
                        tension: 0.4,
                        pointRadius: 4,
                        pointHoverRadius: 6,
                        pointBackgroundColor: this.colors.secondary,
                        pointBorderColor: '#fff',
                        pointBorderWidth: 2
                    }
                ]
            },
            options
        });
    },
    
    /**
     * Create Binet Accuracy Chart
     */
    createBinetChart() {
        const canvas = document.getElementById('binetChart');
        if (!canvas || !this.data.binet || this.data.binet.length === 0) return;
        
        const ctx = canvas.getContext('2d');
        const options = this.getCommonOptions();
        
        // Use logarithmic scale for better visualization
        options.scales.y.type = 'logarithmic';
        options.scales.y.min = 1e-17;
        options.scales.y.title.text = 'Erreur Relative (echelle log)';
        options.scales.x.title.text = 'Valeur de n';
        
        // Custom tooltip for scientific notation
        options.plugins.tooltip.callbacks = {
            title: (items) => `n = ${items[0].label}`,
            label: (context) => {
                const value = context.parsed.y;
                const exact = this.data.binet[context.dataIndex]?.exact;
                const binet = this.data.binet[context.dataIndex]?.binet;
                return [
                    `Erreur relative: ${value.toExponential(2)}`,
                    `Valeur exacte: ${exact?.toLocaleString() || 'N/A'}`,
                    `Valeur Binet: ${binet?.toLocaleString() || 'N/A'}`
                ];
            },
            afterBody: (items) => {
                const n = parseInt(items[0].label);
                if (n <= 78) {
                    return ['\n✓ Dans la zone de precision'];
                }
                return ['\n⚠ Hors zone de precision'];
            }
        };
        
        // Filter out zero values for log scale
        const filteredData = this.data.binet.map(d => {
            const val = d.rel_error;
            return val === 0 ? 1e-17 : val;
        });
        
        this.charts.binet = new Chart(ctx, {
            type: 'line',
            data: {
                labels: this.data.binet.map(d => d.n),
                datasets: [
                    {
                        label: 'Erreur Relative',
                        data: filteredData,
                        borderColor: this.colors.danger,
                        backgroundColor: this.colors.dangerLight,
                        borderWidth: 2,
                        fill: true,
                        tension: 0.3,
                        pointRadius: 3,
                        pointHoverRadius: 5,
                        pointBackgroundColor: this.colors.danger,
                        pointBorderColor: '#fff',
                        pointBorderWidth: 1
                    }
                ]
            },
            options
        });
        
        // Update stats
        const accurateCount = this.data.binet.filter(d => d.rel_error === 0).length;
        const maxError = Math.max(...this.data.binet.map(d => d.abs_error));
        
        const accurateEl = document.getElementById('binetAccurateCount');
        const maxErrorEl = document.getElementById('binetMaxError');
        
        if (accurateEl) accurateEl.textContent = accurateCount;
        if (maxErrorEl) maxErrorEl.textContent = maxError.toExponential(2);
    },
    
    /**
     * Create Golden Ratio Convergence Chart
     */
    createGoldenChart() {
        const canvas = document.getElementById('goldenChart');
        if (!canvas || !this.data.golden || this.data.golden.length === 0) return;
        
        const ctx = canvas.getContext('2d');
        const options = this.getCommonOptions();
        
        const PHI = 1.618033988749895;
        
        options.scales.x.title.text = 'Valeur de n';
        options.scales.y.title.text = 'Ratio F(n+1)/F(n)';
        options.scales.y.min = 1.4;
        options.scales.y.max = 2.1;
        
        // Custom tooltip
        options.plugins.tooltip.callbacks = {
            title: (items) => `n = ${items[0].label}`,
            label: (context) => {
                if (context.dataset.label.includes('Phi')) {
                    return `φ (Nombre d'Or): ${PHI}`;
                }
                const ratio = context.parsed.y;
                const error = this.data.golden[context.dataIndex]?.error_from_phi;
                return [
                    `Ratio: ${ratio.toFixed(15)}`,
                    `Erreur vs φ: ${error?.toExponential(4) || '0'}`
                ];
            }
        };
        
        // Add annotation for phi line
        options.plugins.annotation = {
            annotations: {
                phiLine: {
                    type: 'line',
                    yMin: PHI,
                    yMax: PHI,
                    borderColor: this.colors.secondary,
                    borderWidth: 2,
                    borderDash: [6, 4],
                    label: {
                        display: true,
                        content: 'φ = 1.618...',
                        position: 'end'
                    }
                }
            }
        };
        
        this.charts.golden = new Chart(ctx, {
            type: 'line',
            data: {
                labels: this.data.golden.map(d => d.n),
                datasets: [
                    {
                        label: 'Ratio F(n+1)/F(n)',
                        data: this.data.golden.map(d => d.ratio),
                        borderColor: this.colors.primary,
                        backgroundColor: this.colors.primaryLight,
                        borderWidth: 2,
                        fill: true,
                        tension: 0.4,
                        pointRadius: 4,
                        pointHoverRadius: 6,
                        pointBackgroundColor: this.colors.primary,
                        pointBorderColor: '#fff',
                        pointBorderWidth: 2
                    },
                    {
                        label: 'Phi (φ)',
                        data: this.data.golden.map(() => PHI),
                        borderColor: this.colors.secondary,
                        borderWidth: 2,
                        borderDash: [6, 4],
                        fill: false,
                        pointRadius: 0,
                        pointHoverRadius: 0
                    }
                ]
            },
            options
        });
        
        // Update stats
        const convergeN = this.data.golden.findIndex(d => d.error_from_phi === 0);
        const lastError = this.data.golden[this.data.golden.length - 1]?.error_from_phi;
        
        const convergeEl = document.getElementById('goldenConvergeN');
        const errorEl = document.getElementById('goldenFinalError');
        
        if (convergeEl) convergeEl.textContent = convergeN > 0 ? `n = ${convergeN}` : 'n > 50';
        if (errorEl) errorEl.textContent = lastError === 0 ? '0' : lastError?.toExponential(2) || '0';
    },
    
    /**
     * Reset zoom on a specific chart
     * @param {string} chartName
     */
    resetZoom(chartName) {
        if (this.charts[chartName]) {
            this.charts[chartName].resetZoom();
        }
    },
    
    /**
     * Bind reset zoom buttons
     */
    bindResetButtons() {
        const buttons = {
            'resetZoomComplexity': 'complexity',
            'resetZoomBinet': 'binet',
            'resetZoomGolden': 'golden'
        };
        
        Object.entries(buttons).forEach(([buttonId, chartName]) => {
            const button = document.getElementById(buttonId);
            if (button) {
                button.addEventListener('click', () => {
                    this.resetZoom(chartName);
                });
            }
        });
    },
    
    /**
     * Destroy all charts
     */
    destroyAll() {
        Object.values(this.charts).forEach(chart => {
            if (chart) chart.destroy();
        });
        this.charts = {};
    },
    
    /**
     * Refresh all charts (useful after theme change)
     */
    async refresh() {
        this.destroyAll();
        this.setChartDefaults();
        
        this.createComplexityChart();
        this.createBinetChart();
        this.createGoldenChart();
    }
};

// Listen for theme changes
window.addEventListener('themechange', () => {
    ChartsManager.refresh();
});

// Export for use in other modules
window.ChartsManager = ChartsManager;
