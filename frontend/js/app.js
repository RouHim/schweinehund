/**
 * Schweinehund PWA - Alpine.js Application State
 */

// Alpine.js app state
function app() {
  return {
    // Current view (heute, tasks, zones)
    currentView: 'today',
    
    // Notification state
    notificationsEnabled: false,
    
    // Task state (will be populated from PocketBase)
    tasks: [],
    zones: [],
    
    // Initialize app
    init() {
      console.log('Schweinehund app initialized');
      this.checkNotificationPermission();
    },
    
    // Check if notifications are available
    checkNotificationPermission() {
      if ('Notification' in window) {
        this.notificationsEnabled = Notification.permission === 'granted';
      }
    }
  };
}

// Service Worker Registration
if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('/sw.js')
      .then(registration => {
        console.log('Service Worker registered:', registration.scope);
      })
      .catch(error => {
        console.error('Service Worker registration failed:', error);
      });
  });
}

// PocketBase client initialization (global)
const pb = new PocketBase('http://localhost:8090');

// Log PocketBase connection
console.log('PocketBase client initialized:', pb.baseUrl);

// HTMX configuration (optional but useful)
document.addEventListener('htmx:configRequest', (event) => {
  // Add any custom headers here if needed
  console.log('HTMX request:', event.detail.path);
});

document.addEventListener('htmx:afterSwap', (event) => {
  console.log('HTMX content swapped');
});
