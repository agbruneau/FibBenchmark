/**
 * Main Application Module
 * Handles initialization, scroll animations, navigation, and interactions
 */

const App = {
    // Configuration
    config: {
        animationThreshold: 0.15,
        navScrollThreshold: 50,
        countAnimationDuration: 2000
    },
    
    // State
    state: {
        isScrolling: false,
        activeSection: 'hero',
        mobileMenuOpen: false
    },
    
    /**
     * Initialize application
     */
    init() {
        // Wait for DOM
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', () => this.setup());
        } else {
            this.setup();
        }
    },
    
    /**
     * Setup all components
     */
    setup() {
        this.initNavigation();
        this.initScrollAnimations();
        this.initCountAnimations();
        this.initSmoothScroll();
        this.initMobileMenu();
        this.initCharts();
        this.setReportDate();
        
        // Mark page as loaded
        document.body.classList.add('loaded');
    },
    
    /**
     * Initialize navigation behavior
     */
    initNavigation() {
        const navbar = document.getElementById('navbar');
        const navLinks = document.querySelectorAll('.nav-link');
        const sections = document.querySelectorAll('section[id]');
        
        // Scroll handler for navbar
        let lastScroll = 0;
        
        const handleScroll = () => {
            const currentScroll = window.scrollY;
            
            // Add/remove scrolled class
            if (currentScroll > this.config.navScrollThreshold) {
                navbar?.classList.add('scrolled');
            } else {
                navbar?.classList.remove('scrolled');
            }
            
            // Update active nav link based on scroll position
            let currentSection = 'hero';
            
            sections.forEach(section => {
                const sectionTop = section.offsetTop - 100;
                const sectionHeight = section.offsetHeight;
                
                if (currentScroll >= sectionTop && currentScroll < sectionTop + sectionHeight) {
                    currentSection = section.getAttribute('id');
                }
            });
            
            if (currentSection !== this.state.activeSection) {
                this.state.activeSection = currentSection;
                
                navLinks.forEach(link => {
                    link.classList.remove('active');
                    if (link.getAttribute('href') === `#${currentSection}`) {
                        link.classList.add('active');
                    }
                });
            }
            
            lastScroll = currentScroll;
        };
        
        // Throttled scroll listener
        let ticking = false;
        window.addEventListener('scroll', () => {
            if (!ticking) {
                requestAnimationFrame(() => {
                    handleScroll();
                    ticking = false;
                });
                ticking = true;
            }
        });
        
        // Initial check
        handleScroll();
    },
    
    /**
     * Initialize scroll-triggered animations
     */
    initScrollAnimations() {
        const animatedElements = document.querySelectorAll('[data-animate]');
        
        if ('IntersectionObserver' in window) {
            const observer = new IntersectionObserver(
                (entries) => {
                    entries.forEach(entry => {
                        if (entry.isIntersecting) {
                            entry.target.classList.add('animate-in');
                            observer.unobserve(entry.target);
                        }
                    });
                },
                {
                    threshold: this.config.animationThreshold,
                    rootMargin: '0px 0px -50px 0px'
                }
            );
            
            animatedElements.forEach(el => observer.observe(el));
        } else {
            // Fallback: show all immediately
            animatedElements.forEach(el => el.classList.add('animate-in'));
        }
    },
    
    /**
     * Initialize count-up animations for metric values
     */
    initCountAnimations() {
        const countElements = document.querySelectorAll('[data-count]');
        
        if ('IntersectionObserver' in window) {
            const observer = new IntersectionObserver(
                (entries) => {
                    entries.forEach(entry => {
                        if (entry.isIntersecting) {
                            this.animateCount(entry.target);
                            observer.unobserve(entry.target);
                        }
                    });
                },
                { threshold: 0.5 }
            );
            
            countElements.forEach(el => observer.observe(el));
        } else {
            // Fallback: set values immediately
            countElements.forEach(el => {
                el.textContent = el.dataset.count;
            });
        }
    },
    
    /**
     * Animate a count-up effect
     * @param {HTMLElement} element
     */
    animateCount(element) {
        const target = parseInt(element.dataset.count, 10);
        const duration = this.config.countAnimationDuration;
        const startTime = performance.now();
        
        const updateCount = (currentTime) => {
            const elapsed = currentTime - startTime;
            const progress = Math.min(elapsed / duration, 1);
            
            // Easing function (ease-out)
            const easeOut = 1 - Math.pow(1 - progress, 3);
            const current = Math.floor(easeOut * target);
            
            element.textContent = current;
            
            if (progress < 1) {
                requestAnimationFrame(updateCount);
            } else {
                element.textContent = target;
            }
        };
        
        requestAnimationFrame(updateCount);
    },
    
    /**
     * Initialize smooth scrolling for anchor links
     */
    initSmoothScroll() {
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', (e) => {
                const href = anchor.getAttribute('href');
                
                if (href === '#') return;
                
                const target = document.querySelector(href);
                if (target) {
                    e.preventDefault();
                    
                    // Close mobile menu if open
                    this.closeMobileMenu();
                    
                    // Scroll to target
                    const navHeight = document.getElementById('navbar')?.offsetHeight || 72;
                    const targetPosition = target.offsetTop - navHeight;
                    
                    window.scrollTo({
                        top: targetPosition,
                        behavior: 'smooth'
                    });
                    
                    // Update URL without jumping
                    history.pushState(null, '', href);
                }
            });
        });
    },
    
    /**
     * Initialize mobile menu
     */
    initMobileMenu() {
        const toggle = document.getElementById('mobileToggle');
        const navLinks = document.querySelector('.nav-links');
        
        if (!toggle || !navLinks) return;
        
        toggle.addEventListener('click', () => {
            this.state.mobileMenuOpen = !this.state.mobileMenuOpen;
            
            if (this.state.mobileMenuOpen) {
                navLinks.classList.add('mobile-open');
                toggle.classList.add('active');
                document.body.style.overflow = 'hidden';
            } else {
                this.closeMobileMenu();
            }
        });
        
        // Close on escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && this.state.mobileMenuOpen) {
                this.closeMobileMenu();
            }
        });
        
        // Close on click outside
        document.addEventListener('click', (e) => {
            if (this.state.mobileMenuOpen && 
                !navLinks.contains(e.target) && 
                !toggle.contains(e.target)) {
                this.closeMobileMenu();
            }
        });
    },
    
    /**
     * Close mobile menu
     */
    closeMobileMenu() {
        const toggle = document.getElementById('mobileToggle');
        const navLinks = document.querySelector('.nav-links');
        
        this.state.mobileMenuOpen = false;
        navLinks?.classList.remove('mobile-open');
        toggle?.classList.remove('active');
        document.body.style.overflow = '';
    },
    
    /**
     * Initialize charts
     */
    initCharts() {
        // ChartsManager is defined in charts.js
        if (typeof ChartsManager !== 'undefined') {
            ChartsManager.init();
        }
    },
    
    /**
     * Set report date from embedded data or current date
     */
    setReportDate() {
        const dateElement = document.getElementById('reportDate');
        if (!dateElement) return;
        
        // Try to get date from page or use current
        const now = new Date();
        const options = { day: 'numeric', month: 'long', year: 'numeric' };
        const formattedDate = now.toLocaleDateString('fr-FR', options);
        
        dateElement.textContent = formattedDate;
    },
    
    /**
     * Utility: Debounce function
     * @param {Function} func
     * @param {number} wait
     * @returns {Function}
     */
    debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    },
    
    /**
     * Utility: Throttle function
     * @param {Function} func
     * @param {number} limit
     * @returns {Function}
     */
    throttle(func, limit) {
        let inThrottle;
        return function(...args) {
            if (!inThrottle) {
                func.apply(this, args);
                inThrottle = true;
                setTimeout(() => inThrottle = false, limit);
            }
        };
    }
};

// Add mobile menu styles dynamically
const mobileStyles = document.createElement('style');
mobileStyles.textContent = `
    @media (max-width: 1024px) {
        .nav-links {
            position: fixed;
            top: var(--navbar-height);
            left: 0;
            right: 0;
            bottom: 0;
            flex-direction: column;
            background: var(--bg-card);
            padding: 2rem;
            gap: 0.5rem;
            transform: translateX(-100%);
            transition: transform 0.3s ease;
            overflow-y: auto;
        }
        
        .nav-links.mobile-open {
            display: flex;
            transform: translateX(0);
        }
        
        .nav-link {
            padding: 1rem;
            font-size: 1.1rem;
            border-radius: var(--radius-md);
        }
        
        .nav-mobile-toggle.active span:nth-child(1) {
            transform: translateY(7px) rotate(45deg);
        }
        
        .nav-mobile-toggle.active span:nth-child(2) {
            opacity: 0;
        }
        
        .nav-mobile-toggle.active span:nth-child(3) {
            transform: translateY(-7px) rotate(-45deg);
        }
    }
`;
document.head.appendChild(mobileStyles);

// Initialize app
App.init();

// Export for debugging
window.App = App;
