export const mockTasks = [
  {
    id: 'task-001',
    collectionId: 'tasks',
    collectionName: 'tasks',
    created: '2026-01-31T10:00:00.000Z',
    updated: '2026-01-31T10:00:00.000Z',
    title: 'Test Task 1',
    description: 'First test task',
    zone: 'zone-001',
    completed: false,
    dueDate: '2026-01-31T23:59:59.000Z',
  },
  {
    id: 'task-002',
    collectionId: 'tasks',
    collectionName: 'tasks',
    created: '2026-01-31T10:05:00.000Z',
    updated: '2026-01-31T10:05:00.000Z',
    title: 'Test Task 2',
    description: 'Second test task',
    zone: 'zone-001',
    completed: false,
    dueDate: '2026-02-01T23:59:59.000Z',
  },
];

export const mockZones = [
  {
    id: 'zone-001',
    collectionId: 'zones',
    collectionName: 'zones',
    created: '2026-01-30T10:00:00.000Z',
    updated: '2026-01-30T10:00:00.000Z',
    name: 'Kitchen',
    description: 'Kitchen cleaning zone',
    color: '#FF7F50',
  },
  {
    id: 'zone-002',
    collectionId: 'zones',
    collectionName: 'zones',
    created: '2026-01-30T10:05:00.000Z',
    updated: '2026-01-30T10:05:00.000Z',
    name: 'Bathroom',
    description: 'Bathroom cleaning zone',
    color: '#3498db',
  },
];

export const mockManifest = {
  name: 'Schweinehund',
  short_name: 'Schweinehund',
  description: 'A progressive web application for task management',
  start_url: '/',
  display: 'standalone',
  scope: '/',
  orientation: 'portrait-primary',
  background_color: '#ffffff',
  theme_color: '#FF7F50',
  icons: [
    {
      src: '/assets/icon-192.png',
      sizes: '192x192',
      type: 'image/png',
    },
    {
      src: '/assets/icon-512.png',
      sizes: '512x512',
      type: 'image/png',
    },
  ],
};

export const tasksListResponse = {
  page: 1,
  perPage: 30,
  totalItems: 2,
  totalPages: 1,
  items: mockTasks,
};

export const zonesListResponse = {
  page: 1,
  perPage: 30,
  totalItems: 2,
  totalPages: 1,
  items: mockZones,
};

export const taskCompletedResponse = {
  ...mockTasks[0],
  completed: true,
  updated: new Date().toISOString(),
};

export const zoneCreatedResponse = {
  id: 'zone-new',
  collectionId: 'zones',
  collectionName: 'zones',
  created: new Date().toISOString(),
  updated: new Date().toISOString(),
  name: 'New Zone',
  description: 'Newly created zone',
  color: '#e74c3c',
};

export const zoneUpdatedResponse = {
  ...mockZones[0],
  name: 'Updated Kitchen',
  updated: new Date().toISOString(),
};
