/**
 * Theme Manager - Dark/Light Mode Toggle
 * Handles theme switching with localStorage persistence and system preference detection
 */

const ThemeManager = {
    // Theme constants
    STORAGE_KEY: 'fib-dashboard-theme',
    THEMES: {
        DARK: 'dark',
        LIGHT: 'light'
    },
    
    // DOM elements
    toggle: null,
    
    /**
     * Initialize theme manager
     */
    init() {
        this.toggle = document.getElementById('themeToggle');
        
        // Set initial theme
        const savedTheme = this.getSavedTheme();
        const systemTheme = this.getSystemTheme();
        const initialTheme = savedTheme || systemTheme;
        
        this.setTheme(initialTheme, false);
        
        // Bind events
        this.bindEvents();
        
        // Listen for system theme changes
        this.watchSystemTheme();
    },
    
    /**
     * Get theme saved in localStorage
     * @returns {string|null}
     */
    getSavedTheme() {
        try {
            return localStorage.getItem(this.STORAGE_KEY);
        } catch (e) {
            console.warn('localStorage not available:', e);
            return null;
        }
    },
    
    /**
     * Save theme to localStorage
     * @param {string} theme
     */
    saveTheme(theme) {
        try {
            localStorage.setItem(this.STORAGE_KEY, theme);
        } catch (e) {
            console.warn('Could not save theme preference:', e);
        }
    },
    
    /**
     * Get system color scheme preference
     * @returns {string}
     */
    getSystemTheme() {
        if (window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches) {
            return this.THEMES.LIGHT;
        }
        return this.THEMES.DARK;
    },
    
    /**
     * Watch for system theme changes
     */
    watchSystemTheme() {
        if (window.matchMedia) {
            window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
                // Only auto-switch if user hasn't manually set a preference
                if (!this.getSavedTheme()) {
                    this.setTheme(e.matches ? this.THEMES.DARK : this.THEMES.LIGHT, false);
                }
            });
        }
    },
    
    /**
     * Get current theme
     * @returns {string}
     */
    getCurrentTheme() {
        return document.documentElement.getAttribute('data-theme') || this.THEMES.DARK;
    },
    
    /**
     * Set theme
     * @param {string} theme - Theme name ('dark' or 'light')
     * @param {boolean} save - Whether to save to localStorage
     */
    setTheme(theme, save = true) {
        // Validate theme
        if (!Object.values(this.THEMES).includes(theme)) {
            console.warn(`Invalid theme: ${theme}`);
            theme = this.THEMES.DARK;
        }
        
        // Apply theme
        document.documentElement.setAttribute('data-theme', theme);
        
        // Update toggle button aria-label
        if (this.toggle) {
            const label = theme === this.THEMES.DARK 
                ? 'Passer au mode clair' 
                : 'Passer au mode sombre';
            this.toggle.setAttribute('aria-label', label);
        }
        
        // Save preference
        if (save) {
            this.saveTheme(theme);
        }
        
        // Update chart colors if charts exist
        this.updateChartColors(theme);
        
        // Dispatch custom event
        window.dispatchEvent(new CustomEvent('themechange', { detail: { theme } }));
    },
    
    /**
     * Toggle between dark and light themes
     */
    toggleTheme() {
        const currentTheme = this.getCurrentTheme();
        const newTheme = currentTheme === this.THEMES.DARK 
            ? this.THEMES.LIGHT 
            : this.THEMES.DARK;
        
        this.setTheme(newTheme);
    },
    
    /**
     * Update Chart.js colors based on theme
     * @param {string} theme
     */
    updateChartColors(theme) {
        if (typeof Chart === 'undefined') return;
        
        const isDark = theme === this.THEMES.DARK;
        const textColor = isDark ? '#f1f5f9' : '#0f172a';
        const gridColor = isDark ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)';
        
        // Update Chart.js defaults
        Chart.defaults.color = textColor;
        Chart.defaults.borderColor = gridColor;
        
        // Update all existing charts
        Object.values(Chart.instances).forEach(chart => {
            if (chart.options.scales) {
                // Update X axis
                if (chart.options.scales.x) {
                    chart.options.scales.x.ticks.color = textColor;
                    chart.options.scales.x.grid.color = gridColor;
                    if (chart.options.scales.x.title) {
                        chart.options.scales.x.title.color = textColor;
                    }
                }
                // Update Y axis
                if (chart.options.scales.y) {
                    chart.options.scales.y.ticks.color = textColor;
                    chart.options.scales.y.grid.color = gridColor;
                    if (chart.options.scales.y.title) {
                        chart.options.scales.y.title.color = textColor;
                    }
                }
            }
            
            // Update legend
            if (chart.options.plugins && chart.options.plugins.legend) {
                chart.options.plugins.legend.labels.color = textColor;
            }
            
            // Update title
            if (chart.options.plugins && chart.options.plugins.title) {
                chart.options.plugins.title.color = textColor;
            }
            
            chart.update('none');
        });
    },
    
    /**
     * Bind event listeners
     */
    bindEvents() {
        // Toggle button click
        if (this.toggle) {
            this.toggle.addEventListener('click', () => {
                this.toggleTheme();
            });
            
            // Keyboard support
            this.toggle.addEventListener('keydown', (e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    this.toggleTheme();
                }
            });
        }
        
        // Keyboard shortcut (Ctrl/Cmd + Shift + L)
        document.addEventListener('keydown', (e) => {
            if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'l') {
                e.preventDefault();
                this.toggleTheme();
            }
        });
    }
};

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    ThemeManager.init();
});

// Export for use in other modules
window.ThemeManager = ThemeManager;
