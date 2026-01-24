// Fibonacci Benchmark Report - JavaScript

document.addEventListener('DOMContentLoaded', function() {
    // Tab navigation
    const tabButtons = document.querySelectorAll('.tab-btn');
    const tabContents = document.querySelectorAll('.tab-content');
    const tabList = document.querySelector('.tabs');

    function activateTab(button) {
        const tabId = button.dataset.tab;

        tabButtons.forEach(btn => {
            btn.classList.remove('active');
            btn.setAttribute('aria-selected', 'false');
            btn.setAttribute('tabindex', '-1');
        });

        tabContents.forEach(content => {
            content.classList.remove('active');
        });

        button.classList.add('active');
        button.setAttribute('aria-selected', 'true');
        button.setAttribute('tabindex', '0');
        button.focus();

        document.getElementById(tabId).classList.add('active');
    }

    tabButtons.forEach(button => {
        button.addEventListener('click', () => {
            activateTab(button);
        });
    });

    // Keyboard navigation
    tabList.addEventListener('keydown', (e) => {
        const key = e.key;
        const currentTab = document.activeElement;

        if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(key)) return;
        if (!Array.from(tabButtons).includes(currentTab)) return;

        let nextTab = null;
        const index = Array.from(tabButtons).indexOf(currentTab);

        switch (key) {
            case 'ArrowLeft':
                nextTab = tabButtons[index - 1] || tabButtons[tabButtons.length - 1];
                break;
            case 'ArrowRight':
                nextTab = tabButtons[index + 1] || tabButtons[0];
                break;
            case 'Home':
                nextTab = tabButtons[0];
                break;
            case 'End':
                nextTab = tabButtons[tabButtons.length - 1];
                break;
        }

        if (nextTab) {
            e.preventDefault();
            activateTab(nextTab);
        }
    });

    // Load and visualize data
    loadComplexityData();
    loadBinetData();
    loadGoldenData();
});

// Try to get data from embedded source first, then fall back to fetch
async function getData(type, filename) {
    // Check for embedded data first (for file:// protocol)
    if (window.EMBEDDED_DATA && window.EMBEDDED_DATA[type]) {
        // If embedded data is a string (JSON), return it as-is
        // If it's already parsed, it will be handled by the caller
        return typeof window.EMBEDDED_DATA[type] === 'string' 
            ? window.EMBEDDED_DATA[type] 
            : JSON.stringify(window.EMBEDDED_DATA[type]);
    }
    
    // Fall back to fetch for server-based viewing
    try {
        const response = await fetch(`data/${filename}`);
        if (!response.ok) throw new Error('Fetch failed');
        return await response.text();
    } catch (error) {
        throw new Error(`Data not available for ${type}`);
    }
}

async function loadComplexityData() {
    try {
        const text = await getData('complexity', 'complexity_comparison.json');
        const data = parseJSON(text);
        
        if (data.length === 0) {
            throw new Error('No data parsed');
        }
        
        const ctx = document.getElementById('complexityChart').getContext('2d');
        new Chart(ctx, {
            type: 'line',
            data: {
                labels: data.map(row => row.n),
                datasets: [
                    {
                        label: 'Iterative (ns)',
                        data: data.map(row => Number(row.iterative_ns) || 0),
                        borderColor: '#f59e0b',
                        backgroundColor: 'rgba(245, 158, 11, 0.1)',
                        fill: true,
                        tension: 0.4
                    },
                    {
                        label: 'Matrix (ns)',
                        data: data.map(row => Number(row.matrix_ns) || 0),
                        borderColor: '#10b981',
                        backgroundColor: 'rgba(16, 185, 129, 0.1)',
                        fill: true,
                        tension: 0.4
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        labels: { color: '#f1f5f9' }
                    },
                    title: {
                        display: true,
                        text: 'Execution Time vs Input Size',
                        color: '#f1f5f9'
                    }
                },
                scales: {
                    x: {
                        title: { display: true, text: 'n', color: '#94a3b8' },
                        ticks: { color: '#94a3b8' },
                        grid: { color: '#334155' }
                    },
                    y: {
                        title: { display: true, text: 'Time (ns)', color: '#94a3b8' },
                        ticks: { color: '#94a3b8' },
                        grid: { color: '#334155' }
                    }
                }
            }
        });

        const minN = data.length > 0 ? Math.min(...data.map(row => row.n)) : 0;
        const maxN = data.length > 0 ? Math.max(...data.map(row => row.n)) : 0;
        document.getElementById('complexityData').innerHTML = 
            `<strong>Data Points:</strong> ${data.length} measurements from n=${minN} to n=${maxN}`;
    } catch (error) {
        console.error('Failed to load complexity data:', error);
        document.getElementById('complexityData').innerHTML = 
            '<em>Data file not found. Run generate_report script first.</em>';
    }
}

async function loadBinetData() {
    try {
        const text = await getData('binet', 'binet_accuracy.json');
        const data = parseJSON(text);
        
        if (data.length === 0) {
            throw new Error('No data parsed');
        }
        
        const ctx = document.getElementById('binetChart').getContext('2d');
        new Chart(ctx, {
            type: 'line',
            data: {
                labels: data.map(row => row.n),
                datasets: [
                    {
                        label: 'Relative Error',
                        data: data.map(row => {
                            const val = Number(row.rel_error);
                            return isNaN(val) || val === 0 ? 1e-20 : val;
                        }),
                        borderColor: '#ef4444',
                        backgroundColor: 'rgba(239, 68, 68, 0.1)',
                        fill: true,
                        tension: 0.4
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        labels: { color: '#f1f5f9' }
                    },
                    title: {
                        display: true,
                        text: 'Binet Formula Relative Error',
                        color: '#f1f5f9'
                    }
                },
                scales: {
                    x: {
                        title: { display: true, text: 'n', color: '#94a3b8' },
                        ticks: { color: '#94a3b8' },
                        grid: { color: '#334155' }
                    },
                    y: {
                        title: { display: true, text: 'Relative Error', color: '#94a3b8' },
                        ticks: { color: '#94a3b8' },
                        grid: { color: '#334155' },
                        type: 'logarithmic'
                    }
                }
            }
        });

        const accurateCount = data.filter(row => Number(row.rel_error) === 0).length;
        document.getElementById('binetData').innerHTML = 
            `<strong>Accuracy:</strong> Binet formula is exact for ${accurateCount} values (n <= 78)`;
    } catch (error) {
        console.error('Failed to load Binet data:', error);
        document.getElementById('binetData').innerHTML = 
            '<em>Data file not found. Run generate_report script first.</em>';
    }
}

async function loadGoldenData() {
    try {
        const text = await getData('golden', 'golden_ratio_convergence.json');
        const data = parseJSON(text);
        
        if (data.length === 0) {
            throw new Error('No data parsed');
        }
        
        const ctx = document.getElementById('goldenChart').getContext('2d');
        new Chart(ctx, {
            type: 'line',
            data: {
                labels: data.map(row => row.n),
                datasets: [
                    {
                        label: 'F(n+1)/F(n)',
                        data: data.map(row => Number(row.ratio)),
                        borderColor: '#6366f1',
                        backgroundColor: 'rgba(99, 102, 241, 0.1)',
                        fill: true,
                        tension: 0.4
                    },
                    {
                        label: 'Golden Ratio (phi)',
                        data: data.map(() => 1.618033988749895),
                        borderColor: '#10b981',
                        borderDash: [5, 5],
                        fill: false
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        labels: { color: '#f1f5f9' }
                    },
                    title: {
                        display: true,
                        text: 'Convergence to Golden Ratio',
                        color: '#f1f5f9'
                    }
                },
                scales: {
                    x: {
                        title: { display: true, text: 'n', color: '#94a3b8' },
                        ticks: { color: '#94a3b8' },
                        grid: { color: '#334155' }
                    },
                    y: {
                        title: { display: true, text: 'Ratio', color: '#94a3b8' },
                        ticks: { color: '#94a3b8' },
                        grid: { color: '#334155' },
                        min: 1.5,
                        max: 2.0
                    }
                }
            }
        });

        const lastError = data[data.length - 1]?.error_from_phi;
        const lastN = data[data.length - 1]?.n || 0;
        document.getElementById('goldenData').innerHTML = 
            `<strong>Convergence:</strong> Error from phi at n=${lastN}: ${Number(lastError).toExponential(4)}`;
    } catch (error) {
        console.error('Failed to load golden ratio data:', error);
        document.getElementById('goldenData').innerHTML = 
            '<em>Data file not found. Run generate_report script first.</em>';
    }
}

function parseCSV(text) {
    if (!text || text.trim() === '') return [];
    
    const lines = text.trim().split('\n');
    if (lines.length < 2) return [];
    
    const headers = lines[0].split(',').map(h => h.trim());
    
    return lines.slice(1).map(line => {
        const values = line.split(',');
        const obj = {};
        headers.forEach((header, i) => {
            obj[header] = values[i]?.trim();
        });
        return obj;
    }).filter(row => Object.keys(row).length > 0);
}

function parseJSON(text) {
    if (!text || text.trim() === '') return [];
    
    try {
        const data = JSON.parse(text);
        return Array.isArray(data) ? data : [];
    } catch (error) {
        console.error('Failed to parse JSON:', error);
        return [];
    }
}
