const API_BASE = '/api';

let deepCleaningSortable = null;

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
    const intervalGroup = document.getElementById('interval-group');
    const intervalInput = document.getElementById('task-interval-weeks');
    
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
        title.textContent = 'Aufgabe bearbeiten';
        idInput.value = task.id;
        nameInput.value = task.name;
        descInput.value = task.description || '';
        zoneInput.value = task.zone || '';
        if (type === 'daily') {
            dayInput.value = task.day_of_week;
            intervalInput.value = task.interval_weeks || 1;
            intervalGroup.style.display = task.day_of_week === -1 ? 'none' : 'block';
        }
    } else {
        title.textContent = 'Aufgabe hinzufügen';
        idInput.value = '';
        if (type === 'daily') {
            const today = new Date().getDay();
            const apiDay = today === 0 ? 7 : today;
            dayInput.value = apiDay;
            intervalInput.value = 1;
            intervalGroup.style.display = 'block';
        }
    }
    
    dayInput.addEventListener('change', () => {
        intervalGroup.style.display = dayInput.value === '-1' ? 'none' : 'block';
    });
    
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
        data.interval_weeks = parseInt(document.getElementById('task-interval-weeks').value) || 1;
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
            throw new Error(`Aufgabe konnte nicht gespeichert werden: ${response.statusText}`);
        }
        
        closeModal();
        
        if (type === 'daily') {
            fetchTodayTasks();
        } else {
            fetchDeepCleaning();
        }
        
    } catch (error) {
        console.error('Fehler beim Speichern der Aufgabe:', error);
        alert(`Aufgabe konnte nicht gespeichert werden: ${error.message}`);
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
            throw new Error(`Aufgaben konnten nicht geladen werden: ${response.statusText}`);
        }
        
        const tasks = await response.json();
        state.tasks = tasks;
        
        loadingEl.style.display = 'none';
        errorEl.style.display = 'none';
        listEl.style.display = 'block';
        
        renderTasks(tasks);
    } catch (error) {
        console.error('Fehler beim Laden der Aufgaben:', error);
        loadingEl.style.display = 'none';
        errorEl.textContent = `Fehler: ${error.message}`;
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
    if (!confirm('Möchtest du diese Aufgabe wirklich löschen?')) {
        return;
    }

    const endpoint = type === 'daily' ? `tasks/${id}` : `deep-cleaning/${id}`;
    
    try {
        const response = await fetch(`${API_BASE}/${endpoint}`, {
            method: 'DELETE'
        });

        if (!response.ok) {
            throw new Error(`Aufgabe konnte nicht gelöscht werden: ${response.statusText}`);
        }

        if (type === 'daily') {
            await fetchTodayTasks();
        } else {
            await fetchDeepCleaning();
        }
    } catch (error) {
        console.error('Fehler beim Löschen der Aufgabe:', error);
        alert(`Aufgabe konnte nicht gelöscht werden: ${error.message}`);
    }
}

function renderTasks(tasks) {
    const listEl = document.getElementById('tasks-list');
    
    if (tasks.length === 0) {
        listEl.innerHTML = '<li class="empty-state"><div class="empty-state-icon">✨</div><p>Keine Aufgaben für heute — genieße deine Freizeit!</p></li>';
        updateProgress(0, 0);
        return;
    }
    
    // Sort tasks: uncompleted first, then completed (stable sort)
    const sortedTasks = [...tasks].sort((a, b) => (a.completed ? 1 : 0) - (b.completed ? 1 : 0));
    
    listEl.innerHTML = sortedTasks.map(task => {
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
                             ${task.interval_weeks > 1 ? `<span class="badge interval-badge">alle ${task.interval_weeks} Wo.</span>` : ''}
                         </div>
                    </div>
                </label>
                <div class="task-actions">
                    <button data-testid="edit-btn" data-task-id="${task.id}" class="icon-btn edit-btn" aria-label="Aufgabe bearbeiten">
                        ${EDIT_ICON}
                    </button>
                    <button data-testid="delete-btn" data-task-id="${task.id}" class="icon-btn delete-btn" aria-label="Aufgabe löschen">
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
            throw new Error(`Aufgabe konnte nicht umgeschaltet werden: ${response.statusText}`);
        }
        
        const updatedTask = await response.json();
        
        const taskIndex = state.tasks.findIndex(t => t.id === parseInt(taskId));
        if (taskIndex !== -1) {
            state.tasks[taskIndex] = updatedTask;
        }
        
        const allCompleted = state.tasks.every(t => t.completed);
        if (isChecked && allCompleted) {
            await showFunFact();
        }
        
    } catch (error) {
        console.error('Fehler beim Umschalten der Aufgabe:', error);
        
        checkbox.checked = !isChecked;
        if (isChecked) {
            taskItem.classList.remove('completed');
        } else {
            taskItem.classList.add('completed');
        }
        
        const revertedCount = state.tasks.filter(t => t.completed).length;
        updateProgress(revertedCount, state.tasks.length);
        
        alert(`Aufgabe konnte nicht aktualisiert werden: ${error.message}`);
    }
}

async function fetchDeepCleaning() {
    const loadingEl = document.getElementById('deep-cleaning-loading');
    const errorEl = document.getElementById('deep-cleaning-error');
    const listEl = document.getElementById('deep-cleaning-list');

    try {
        const response = await fetch(`${API_BASE}/deep-cleaning`);
        if (!response.ok) {
            throw new Error(`Grundreinigung konnte nicht geladen werden: ${response.statusText}`);
        }
        
        const tasks = await response.json();
        state.deepCleaning = tasks;
        
        loadingEl.style.display = 'none';
        errorEl.style.display = 'none';
        listEl.style.display = 'block';
        
        renderDeepCleaning(tasks);
    } catch (error) {
        console.error('Fehler beim Laden der Grundreinigung:', error);
        loadingEl.style.display = 'none';
        errorEl.textContent = `Fehler: ${error.message}`;
        errorEl.style.display = 'block';
    }
}

function renderDeepCleaning(tasks) {
    const listEl = document.getElementById('deep-cleaning-list');
    
    if (tasks.length === 0) {
        listEl.innerHTML = '<li class="empty-state"><div class="empty-state-icon">🧹</div><p>Keine Grundreinigungen in der Warteschlange</p></li>';
        return;
    }
    
    listEl.innerHTML = tasks.map((task, index) => {
        return `
            <li class="task-item" data-deep-cleaning-id="${task.id}">
                <button class="drag-handle" data-testid="drag-handle" aria-label="Aufgabe verschieben" tabindex="-1">
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                        <circle cx="5" cy="3" r="1.5"/><circle cx="11" cy="3" r="1.5"/>
                        <circle cx="5" cy="8" r="1.5"/><circle cx="11" cy="8" r="1.5"/>
                        <circle cx="5" cy="13" r="1.5"/><circle cx="11" cy="13" r="1.5"/>
                    </svg>
                </button>
                <div class="deep-cleaning-position">#${index + 1}</div>
                <div class="task-content">
                    <h3 class="task-name">${escapeHtml(task.name)}</h3>
                    ${task.description ? `<p class="task-description">${escapeHtml(task.description)}</p>` : ''}
                </div>
                <div class="task-actions">
                    <button data-testid="move-up-btn" data-task-id="${task.id}" class="icon-btn move-btn move-up-btn" aria-label="Nach oben verschieben" ${index === 0 ? 'disabled' : ''}>↑</button>
                    <button data-testid="move-down-btn" data-task-id="${task.id}" class="icon-btn move-btn move-down-btn" aria-label="Nach unten verschieben" ${index === tasks.length - 1 ? 'disabled' : ''}>↓</button>
                    <button data-testid="complete-btn" data-task-id="${task.id}" class="complete-btn" aria-label="Aufgabe erledigen">Erledigt</button>
                    <button data-testid="edit-btn" data-task-id="${task.id}" class="icon-btn edit-btn" aria-label="Aufgabe bearbeiten">
                        ${EDIT_ICON}
                    </button>
                    <button data-testid="delete-btn" data-task-id="${task.id}" class="icon-btn delete-btn" aria-label="Aufgabe löschen">
                        ${DELETE_ICON}
                    </button>
                </div>
            </li>
        `;
    }).join('');
    
    attachDeepCleaningListeners();
    
    // Destroy previous Sortable instance
    if (deepCleaningSortable) {
        deepCleaningSortable.destroy();
        deepCleaningSortable = null;
    }
    
    // Only init when 2+ tasks
    if (tasks.length >= 2) {
        deepCleaningSortable = Sortable.create(listEl, {
            handle: '.drag-handle',
            animation: 150,
            forceFallback: true,
            touchStartThreshold: 3,
            ghostClass: 'sortable-ghost',
            chosenClass: 'sortable-chosen',
            dragClass: 'sortable-drag',
            dataIdAttr: 'data-deep-cleaning-id',
            filter: '.empty-state',
            onEnd: function(evt) {
                if (evt.oldIndex === evt.newIndex) return;
                handleDragReorder();
            }
        });
    }
}

function attachDeepCleaningListeners() {
    const listEl = document.getElementById('deep-cleaning-list');

    listEl.querySelectorAll('.complete-btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            e.stopPropagation();
            const taskId = btn.dataset.taskId;
            handleDeepCleaningComplete(taskId, btn);
        });
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

    listEl.querySelectorAll('.move-up-btn').forEach(btn => {
        btn.addEventListener('click', async (e) => {
            e.stopPropagation();
            const taskId = parseInt(btn.dataset.taskId);
            await handleArrowMove(taskId, 'up');
        });
    });

    listEl.querySelectorAll('.move-down-btn').forEach(btn => {
        btn.addEventListener('click', async (e) => {
            e.stopPropagation();
            const taskId = parseInt(btn.dataset.taskId);
            await handleArrowMove(taskId, 'down');
        });
    });
}

async function handleDeepCleaningComplete(taskId, btn) {
    btn.disabled = true;
    btn.textContent = '...';
    
    try {
        const response = await fetch(`${API_BASE}/deep-cleaning/${taskId}/complete`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            }
        });
        
        if (!response.ok) {
            throw new Error(`Grundreinigung konnte nicht abgeschlossen werden: ${response.statusText}`);
        }
        
        await fetchDeepCleaning();
        
    } catch (error) {
        console.error('Fehler beim Abschließen der Grundreinigung:', error);
        
        btn.disabled = false;
        btn.textContent = 'Erledigt';
        
        alert(`Aufgabe konnte nicht abgeschlossen werden: ${error.message}`);
    }
}

async function handleDragReorder() {
    const listEl = document.getElementById('deep-cleaning-list');
    const newOrder = Array.from(listEl.querySelectorAll('.task-item'))
        .map(li => parseInt(li.dataset.deepCleaningId));
    
    try {
        const response = await fetch(`${API_BASE}/deep-cleaning/reorder`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ order: newOrder })
        });
        
        if (!response.ok) {
            throw new Error(`Reorder failed: ${response.statusText}`);
        }
        
        const updated = await response.json();
        state.deepCleaning = updated;
        renderDeepCleaning(state.deepCleaning);
        
    } catch (error) {
        console.error('Reorder error:', error);
        renderDeepCleaning(state.deepCleaning);
    }
}

async function handleArrowMove(taskId, direction) {
    const currentIdx = state.deepCleaning.findIndex(t => t.id === taskId);
    if (currentIdx === -1) return;
    
    const swapIdx = direction === 'up' ? currentIdx - 1 : currentIdx + 1;
    if (swapIdx < 0 || swapIdx >= state.deepCleaning.length) return;
    
    document.querySelectorAll('.move-btn').forEach(btn => btn.disabled = true);
    
    const newOrder = [...state.deepCleaning];
    [newOrder[currentIdx], newOrder[swapIdx]] = [newOrder[swapIdx], newOrder[currentIdx]];
    const orderIds = newOrder.map(t => t.id);
    
    try {
        const response = await fetch(`${API_BASE}/deep-cleaning/reorder`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ order: orderIds })
        });
        
        if (!response.ok) throw new Error(`Reorder failed: ${response.statusText}`);
        
        const updated = await response.json();
        state.deepCleaning = updated;
        renderDeepCleaning(state.deepCleaning);
        
    } catch (error) {
        console.error('Arrow move error:', error);
        renderDeepCleaning(state.deepCleaning);
    }
}

async function fetchSettings() {
    const loadingEl = document.getElementById('settings-loading');
    const errorEl = document.getElementById('settings-error');
    const formEl = document.getElementById('settings-form');

    try {
        const response = await fetch(`${API_BASE}/settings`);
        if (!response.ok) {
            throw new Error(`Einstellungen konnten nicht geladen werden: ${response.statusText}`);
        }
        
        const settings = await response.json();
        state.settings = settings;
        
        loadingEl.style.display = 'none';
        errorEl.style.display = 'none';
        formEl.style.display = 'block';
        
        populateSettings(settings);
    } catch (error) {
        console.error('Fehler beim Laden der Einstellungen:', error);
        loadingEl.style.display = 'none';
        errorEl.textContent = `Fehler: ${error.message}`;
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
            throw new Error(`Einstellungen konnten nicht gespeichert werden: ${response.statusText}`);
        }
        
        const updatedSettings = await response.json();
        state.settings = updatedSettings;
        
        alert('Einstellungen erfolgreich gespeichert!');
    } catch (error) {
        console.error('Fehler beim Speichern der Einstellungen:', error);
        alert(`Einstellungen konnten nicht gespeichert werden: ${error.message}`);
    }
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function getDayName(dayNumber) {
    const days = ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag'];
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

async function fetchJokeWithTimeout(timeoutMs) {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), timeoutMs);
    
    try {
        const response = await fetch(
            'https://v2.jokeapi.dev/joke/Any?lang=de&safe-mode&type=single',
            { signal: controller.signal }
        );
        clearTimeout(timeoutId);
        
        if (!response.ok) {
            throw new Error(`Witz konnte nicht geladen werden: ${response.statusText}`);
        }
        const data = await response.json();
        if (data.error) {
            throw new Error(data.message || 'Kein Witz gefunden');
        }
        return data.joke;
    } catch (error) {
        clearTimeout(timeoutId);
        throw error;
    }
}

function showFunFact() {
    const modal = document.getElementById('fun-fact-modal');
    const jokeText = document.getElementById('fun-fact-text');
    
    if (!modal || !jokeText) {
        console.error('Witz-Modal-Elemente nicht gefunden');
        return;
    }
    
    // Show modal immediately with loading state
    jokeText.textContent = 'Lade Witz...';
    modal.showModal();
    
    // Set up auto-close timeout
    const autoCloseTimeout = setTimeout(() => {
        modal.close();
    }, 15000);
    
    modal.addEventListener('close', () => {
        clearTimeout(autoCloseTimeout);
    }, { once: true });
    
    // Fetch joke in background (non-blocking)
    fetchJokeWithTimeout(5000)
        .then(joke => {
            if (modal.open) {
                jokeText.textContent = joke;
            }
        })
        .catch(error => {
            console.error('Witz konnte nicht geladen werden:', error);
            if (modal.open) {
                jokeText.textContent = 'Gut gemacht! 🎉';
            }
        });
}

function closeFunFactModal() {
    const modal = document.getElementById('fun-fact-modal');
    if (modal) {
        modal.close();
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
                console.log('[SW] Service Worker erfolgreich registriert:', registration);
            })
            .catch(error => {
                console.error('[SW] Service Worker Registrierung fehlgeschlagen:', error);
            });
    });
}
