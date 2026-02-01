const API_BASE = '/api';

const state = {
    tasks: [],
    deepCleaning: [],
    settings: null
};

function initTheme() {
    const savedTheme = localStorage.getItem('theme') || 'light';
    document.documentElement.setAttribute('data-theme', savedTheme);
    updateThemeIcon(savedTheme);
}

function updateThemeIcon(theme) {
    const icon = document.querySelector('.theme-icon');
    if (icon) {
        icon.textContent = theme === 'dark' ? '☀️' : '🌙';
    }
}

function toggleTheme() {
    const currentTheme = document.documentElement.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
    updateThemeIcon(newTheme);
}

async function fetchTodayTasks() {
    const loadingEl = document.getElementById('tasks-loading');
    const errorEl = document.getElementById('tasks-error');
    const listEl = document.getElementById('tasks-list');

    try {
        const response = await fetch(`${API_BASE}/tasks/today`);
        if (!response.ok) {
            throw new Error(`Failed to fetch tasks: ${response.statusText}`);
        }
        
        const tasks = await response.json();
        state.tasks = tasks;
        
        loadingEl.style.display = 'none';
        errorEl.style.display = 'none';
        listEl.style.display = 'block';
        
        renderTasks(tasks);
    } catch (error) {
        console.error('Error fetching tasks:', error);
        loadingEl.style.display = 'none';
        errorEl.textContent = `Error: ${error.message}`;
        errorEl.style.display = 'block';
    }
}

function renderTasks(tasks) {
    const listEl = document.getElementById('tasks-list');
    
    if (tasks.length === 0) {
        listEl.innerHTML = '<li style="text-align: center; padding: 2rem; color: var(--text-secondary);">No tasks for today!</li>';
        return;
    }
    
    listEl.innerHTML = tasks.map(task => {
        const completed = task.completed || false;
        const completedClass = completed ? 'completed' : '';
        
        return `
            <li class="task-item ${completedClass}" data-task-id="${task.id}">
                <label class="task-checkbox-wrapper">
                    <input 
                        type="checkbox" 
                        class="task-checkbox" 
                        data-task-id="${task.id}"
                        ${completed ? 'checked' : ''}
                    >
                    <div class="task-content">
                        <h3 class="task-name">${escapeHtml(task.name)}</h3>
                        ${task.description ? `<p class="task-description">${escapeHtml(task.description)}</p>` : ''}
                        <div class="task-meta">
                            ${task.zone ? `<span class="task-badge">${escapeHtml(task.zone)}</span>` : ''}
                            ${task.day_of_week ? `<span class="task-badge">${getDayName(task.day_of_week)}</span>` : ''}
                        </div>
                    </div>
                </label>
            </li>
        `;
    }).join('');
    
    attachTaskCheckboxListeners();
}

function attachTaskCheckboxListeners() {
    const checkboxes = document.querySelectorAll('.task-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.addEventListener('change', handleTaskToggle);
    });
}

async function handleTaskToggle(event) {
    const checkbox = event.target;
    const taskId = checkbox.dataset.taskId;
    const isChecked = checkbox.checked;
    
    const taskItem = checkbox.closest('.task-item');
    if (isChecked) {
        taskItem.classList.add('completed');
    } else {
        taskItem.classList.remove('completed');
    }
    
    try {
        const response = await fetch(`${API_BASE}/tasks/${taskId}/toggle`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            }
        });
        
        if (!response.ok) {
            throw new Error(`Failed to toggle task: ${response.statusText}`);
        }
        
        const updatedTask = await response.json();
        
        const taskIndex = state.tasks.findIndex(t => t.id === parseInt(taskId));
        if (taskIndex !== -1) {
            state.tasks[taskIndex] = updatedTask;
        }
        
    } catch (error) {
        console.error('Error toggling task:', error);
        
        checkbox.checked = !isChecked;
        if (isChecked) {
            taskItem.classList.remove('completed');
        } else {
            taskItem.classList.add('completed');
        }
        
        alert(`Failed to update task: ${error.message}`);
    }
}

async function fetchDeepCleaning() {
    const loadingEl = document.getElementById('deep-cleaning-loading');
    const errorEl = document.getElementById('deep-cleaning-error');
    const listEl = document.getElementById('deep-cleaning-list');

    try {
        const response = await fetch(`${API_BASE}/deep-cleaning`);
        if (!response.ok) {
            throw new Error(`Failed to fetch deep cleaning: ${response.statusText}`);
        }
        
        const tasks = await response.json();
        state.deepCleaning = tasks;
        
        loadingEl.style.display = 'none';
        errorEl.style.display = 'none';
        listEl.style.display = 'block';
        
        renderDeepCleaning(tasks);
    } catch (error) {
        console.error('Error fetching deep cleaning:', error);
        loadingEl.style.display = 'none';
        errorEl.textContent = `Error: ${error.message}`;
        errorEl.style.display = 'block';
    }
}

function renderDeepCleaning(tasks) {
    const listEl = document.getElementById('deep-cleaning-list');
    
    if (tasks.length === 0) {
        listEl.innerHTML = '<li style="text-align: center; padding: 2rem; color: var(--text-secondary);">No deep cleaning tasks in queue!</li>';
        return;
    }
    
    listEl.innerHTML = tasks.map(task => {
        const completed = !!task.completed_at;
        const completedClass = completed ? 'completed' : '';
        
        return `
            <li class="task-item ${completedClass}" data-deep-cleaning-id="${task.id}">
                <label class="task-checkbox-wrapper">
                    <input 
                        type="checkbox" 
                        class="task-checkbox" 
                        data-deep-cleaning-id="${task.id}"
                        ${completed ? 'checked' : ''}
                    >
                    <div class="task-content">
                        <h3 class="task-name">${escapeHtml(task.name)}</h3>
                        ${task.description ? `<p class="task-description">${escapeHtml(task.description)}</p>` : ''}
                        <div class="task-meta">
                            <span class="task-badge">Queue #${task.queue_position}</span>
                        </div>
                    </div>
                </label>
            </li>
        `;
    }).join('');
    
    attachDeepCleaningCheckboxListeners();
}

function attachDeepCleaningCheckboxListeners() {
    const checkboxes = document.querySelectorAll('[data-deep-cleaning-id]');
    checkboxes.forEach(checkbox => {
        checkbox.addEventListener('change', handleDeepCleaningToggle);
    });
}

async function handleDeepCleaningToggle(event) {
    const checkbox = event.target;
    const taskId = checkbox.dataset.deepCleaningId;
    const isChecked = checkbox.checked;
    
    if (!isChecked) {
        checkbox.checked = true;
        return;
    }
    
    const taskItem = checkbox.closest('.task-item');
    taskItem.classList.add('completed');
    
    try {
        const response = await fetch(`${API_BASE}/deep-cleaning/${taskId}/complete`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            }
        });
        
        if (!response.ok) {
            throw new Error(`Failed to complete deep cleaning: ${response.statusText}`);
        }
        
        await fetchDeepCleaning();
        
    } catch (error) {
        console.error('Error completing deep cleaning:', error);
        
        checkbox.checked = false;
        taskItem.classList.remove('completed');
        
        alert(`Failed to complete task: ${error.message}`);
    }
}

async function fetchSettings() {
    const loadingEl = document.getElementById('settings-loading');
    const errorEl = document.getElementById('settings-error');
    const formEl = document.getElementById('settings-form');

    try {
        const response = await fetch(`${API_BASE}/settings`);
        if (!response.ok) {
            throw new Error(`Failed to fetch settings: ${response.statusText}`);
        }
        
        const settings = await response.json();
        state.settings = settings;
        
        loadingEl.style.display = 'none';
        errorEl.style.display = 'none';
        formEl.style.display = 'block';
        
        populateSettings(settings);
    } catch (error) {
        console.error('Error fetching settings:', error);
        loadingEl.style.display = 'none';
        errorEl.textContent = `Error: ${error.message}`;
        errorEl.style.display = 'block';
    }
}

function populateSettings(settings) {
    const notificationEnabledEl = document.getElementById('notification-enabled');
    const notificationTimeEl = document.getElementById('notification-time');
    
    notificationEnabledEl.checked = settings.notification_enabled || false;
    notificationTimeEl.value = settings.notification_time || '09:00';
}

async function handleSettingsSubmit(event) {
    event.preventDefault();
    
    const notificationEnabled = document.getElementById('notification-enabled').checked;
    const notificationTime = document.getElementById('notification-time').value;
    
    try {
        const response = await fetch(`${API_BASE}/settings`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                notification_enabled: notificationEnabled,
                notification_time: notificationTime
            })
        });
        
        if (!response.ok) {
            throw new Error(`Failed to save settings: ${response.statusText}`);
        }
        
        const updatedSettings = await response.json();
        state.settings = updatedSettings;
        
        alert('Settings saved successfully!');
    } catch (error) {
        console.error('Error saving settings:', error);
        alert(`Failed to save settings: ${error.message}`);
    }
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function getDayName(dayNumber) {
    const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];
    return days[dayNumber - 1] || '';
}

function init() {
    initTheme();
    
    const themeToggle = document.getElementById('theme-toggle');
    if (themeToggle) {
        themeToggle.addEventListener('click', toggleTheme);
    }
    
    const settingsForm = document.getElementById('settings-form');
    if (settingsForm) {
        settingsForm.addEventListener('submit', handleSettingsSubmit);
    }
    
    fetchTodayTasks();
    fetchDeepCleaning();
    fetchSettings();
}

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}

// Register Service Worker for PWA
if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        navigator.serviceWorker.register('/sw.js')
            .then(registration => {
                console.log('[SW] Service Worker registered successfully:', registration);
            })
            .catch(error => {
                console.error('[SW] Service Worker registration failed:', error);
            });
    });
}
