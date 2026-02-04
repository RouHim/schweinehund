const API_BASE = '/api';

function initModal() {
    const modal = document.getElementById('task-modal');
    const form = document.getElementById('task-form');
    const closeBtn = document.querySelector('[data-close-modal]');
    
    if (closeBtn) {
        closeBtn.addEventListener('click', closeModal);
    }
    
    if (form) {
        form.addEventListener('submit', handleTaskSubmit);
    }
    
    if (modal) {
        modal.addEventListener('click', (event) => {
            if (event.target === modal) {
                closeModal();
            }
        });
    }
    
    const addDailyBtn = document.getElementById('add-daily-task-btn');
    if (addDailyBtn) {
        addDailyBtn.addEventListener('click', () => {
            openModal('daily');
        });
    }
    
    const addDeepCleaningBtn = document.getElementById('add-deep-cleaning-btn');
    if (addDeepCleaningBtn) {
        addDeepCleaningBtn.addEventListener('click', () => {
            openModal('deep-cleaning');
        });
    }
}

function openModal(type, task = null) {
    const modal = document.getElementById('task-modal');
    const form = document.getElementById('task-form');
    const title = document.getElementById('modal-title');
    const typeInput = document.getElementById('task-type');
    const idInput = document.getElementById('task-id');
    const nameInput = document.getElementById('task-name');
    const descInput = document.getElementById('task-description');
    const zoneInput = document.getElementById('task-zone');
    const dayField = document.getElementById('day-of-week-field');
    const dayInput = document.getElementById('task-day-of-week');
    
    if (!modal || !form) return;
    
    form.reset();
    typeInput.value = type;
    
    if (type === 'daily') {
        dayField.style.display = 'block';
        dayInput.required = true;
    } else {
        dayField.style.display = 'none';
        dayInput.required = false;
    }
    
    if (task) {
        title.textContent = 'Edit Task';
        idInput.value = task.id;
        nameInput.value = task.name;
        descInput.value = task.description || '';
        zoneInput.value = task.zone || '';
        if (type === 'daily') {
            dayInput.value = task.day_of_week;
        }
    } else {
        title.textContent = 'Add Task';
        idInput.value = '';
        if (type === 'daily') {
            const today = new Date().getDay();
            const apiDay = today === 0 ? 7 : today;
            dayInput.value = apiDay;
        }
    }
    
    modal.showModal();
}

window.openModal = openModal;

function closeModal() {
    const modal = document.getElementById('task-modal');
    if (modal) {
        modal.close();
        document.getElementById('task-form').reset();
    }
}

async function handleTaskSubmit(event) {
    event.preventDefault();
    
    const form = event.target;
    const formData = new FormData(form);
    const type = formData.get('task-type');
    const id = formData.get('task-id');
    const isEdit = !!id;
    
    const data = {
        name: formData.get('name'),
        description: formData.get('description'),
        zone: formData.get('zone')
    };
    
    if (type === 'daily') {
        data.day_of_week = parseInt(formData.get('day_of_week'));
    }
    
    const endpoint = type === 'daily' ? 'tasks' : 'deep-cleaning';
    const url = isEdit 
        ? `${API_BASE}/${endpoint}/${id}`
        : `${API_BASE}/${endpoint}`;
        
    const method = isEdit ? 'PUT' : 'POST';
    
    try {
        const response = await fetch(url, {
            method: method,
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        });
        
        if (!response.ok) {
            throw new Error(`Failed to save task: ${response.statusText}`);
        }
        
        closeModal();
        
        if (type === 'daily') {
            fetchTodayTasks();
        } else {
            fetchDeepCleaning();
        }
        
    } catch (error) {
        console.error('Error saving task:', error);
        alert(`Failed to save task: ${error.message}`);
    }
}

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

const EDIT_ICON = `<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
  <path d="M12.146 0.146a.5.5 0 0 1 .708 0l3 3a.5.5 0 0 1 0 .708l-10 10a.5.5 0 0 1-.168.11l-5 2a.5.5 0 0 1-.65-.65l2-5a.5.5 0 0 1 .11-.168l10-10z"/>
</svg>`;

const DELETE_ICON = `<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
  <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
  <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1z"/>
</svg>`;

async function deleteTask(type, id) {
    if (!confirm('Are you sure you want to delete this task?')) {
        return;
    }

    const endpoint = type === 'daily' ? `tasks/${id}` : `deep-cleaning/${id}`;
    
    try {
        const response = await fetch(`${API_BASE}/${endpoint}`, {
            method: 'DELETE'
        });

        if (!response.ok) {
            throw new Error(`Failed to delete task: ${response.statusText}`);
        }

        if (type === 'daily') {
            await fetchTodayTasks();
        } else {
            await fetchDeepCleaning();
        }
    } catch (error) {
        console.error('Error deleting task:', error);
        alert(`Failed to delete task: ${error.message}`);
    }
}

function renderTasks(tasks) {
    const listEl = document.getElementById('tasks-list');
    
    if (tasks.length === 0) {
        listEl.innerHTML = '<li class="empty-state"><div class="empty-state-icon">✨</div><p>No tasks for today — enjoy your free time!</p></li>';
        updateProgress(0, 0);
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
                <div class="task-actions">
                    <button data-testid="edit-btn" data-task-id="${task.id}" class="icon-btn edit-btn" aria-label="Edit task">
                        ${EDIT_ICON}
                    </button>
                    <button data-testid="delete-btn" data-task-id="${task.id}" class="icon-btn delete-btn" aria-label="Delete task">
                        ${DELETE_ICON}
                    </button>
                </div>
            </li>
        `;
    }).join('');
    
    attachTaskListeners();
    
    const completedCount = tasks.filter(t => t.completed).length;
    updateProgress(completedCount, tasks.length);
}

function attachTaskListeners() {
    const listEl = document.getElementById('tasks-list');
    const checkboxes = listEl.querySelectorAll('.task-checkbox');
    
    checkboxes.forEach(checkbox => {
        checkbox.addEventListener('change', handleTaskToggle);
    });

    listEl.querySelectorAll('.edit-btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            e.stopPropagation(); // Prevent triggering checkbox
            const id = parseInt(btn.dataset.taskId);
            const task = state.tasks.find(t => t.id === id);
            if (task) window.openModal('daily', task);
        });
    });

    listEl.querySelectorAll('.delete-btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            e.stopPropagation(); // Prevent triggering checkbox
            const id = parseInt(btn.dataset.taskId);
            deleteTask('daily', id);
        });
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
    
    const completedCount = state.tasks.filter(t => t.id === parseInt(taskId) ? isChecked : t.completed).length;
    updateProgress(completedCount, state.tasks.length);
    
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
        
        const revertedCount = state.tasks.filter(t => t.completed).length;
        updateProgress(revertedCount, state.tasks.length);
        
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
        listEl.innerHTML = '<li class="empty-state"><div class="empty-state-icon">🧹</div><p>No deep cleaning tasks in queue</p></li>';
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
                <div class="task-actions">
                    <button data-testid="edit-btn" data-task-id="${task.id}" class="icon-btn edit-btn" aria-label="Edit task">
                        ${EDIT_ICON}
                    </button>
                    <button data-testid="delete-btn" data-task-id="${task.id}" class="icon-btn delete-btn" aria-label="Delete task">
                        ${DELETE_ICON}
                    </button>
                </div>
            </li>
        `;
    }).join('');
    
    attachDeepCleaningListeners();
}

function attachDeepCleaningListeners() {
    const listEl = document.getElementById('deep-cleaning-list');
    const checkboxes = listEl.querySelectorAll('.task-checkbox');
    
    checkboxes.forEach(checkbox => {
        checkbox.addEventListener('change', handleDeepCleaningToggle);
    });

    listEl.querySelectorAll('.edit-btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            e.stopPropagation();
            const id = parseInt(btn.dataset.taskId);
            const task = state.deepCleaning.find(t => t.id === id);
            if (task) window.openModal('deep-cleaning', task);
        });
    });

    listEl.querySelectorAll('.delete-btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            e.stopPropagation();
            const id = parseInt(btn.dataset.taskId);
            deleteTask('deep-cleaning', id);
        });
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

function updateProgress(done, total) {
    const container = document.getElementById('tasks-progress');
    const doneEl = document.getElementById('tasks-done');
    const totalEl = document.getElementById('tasks-total');
    const fillEl = document.getElementById('progress-fill');
    
    if (!container || !doneEl || !totalEl || !fillEl) return;
    
    if (total === 0) {
        container.style.display = 'none';
        return;
    }
    
    container.style.display = 'block';
    doneEl.textContent = done;
    totalEl.textContent = total;
    
    const percent = Math.round((done / total) * 100);
    fillEl.style.width = `${percent}%`;
    
    if (done === total) {
        fillEl.classList.add('all-done');
    } else {
        fillEl.classList.remove('all-done');
    }
}

function init() {
    initTheme();
    initModal();
    
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
