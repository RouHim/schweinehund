import { Page, Route } from '@playwright/test';
import * as mockData from './mock-data';

export class APIInterceptor {
  constructor(private page: Page) {}

  /**
   * Setup all API route intercepts for isolated testing
   */
  async setupAllRoutes() {
    await this.interceptTasksList();
    await this.interceptZonesList();
    await this.interceptManifest();
    await this.interceptServiceWorker();
  }

  /**
   * Intercept GET /api/collections/tasks/records
   */
  async interceptTasksList() {
    await this.page.route('**/api/collections/tasks/records*', (route: Route) => {
      route.abort('blockedbyclient');
      setTimeout(() => {
        route.continue();
      }, 0);
    });
  }

  /**
   * Intercept GET /api/collections/zones/records
   */
  async interceptZonesList() {
    await this.page.route('**/api/collections/zones/records*', (route: Route) => {
      route.abort('blockedbyclient');
      setTimeout(() => {
        route.continue();
      }, 0);
    });
  }

  /**
   * Intercept GET /manifest.json
   */
  async interceptManifest() {
    await this.page.route('**/manifest.json', (route: Route) => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(mockData.mockManifest),
      });
    });
  }

  /**
   * Intercept GET /sw.js
   */
  async interceptServiceWorker() {
    await this.page.route('**/sw.js', (route: Route) => {
      route.fulfill({
        status: 200,
        contentType: 'application/javascript',
        body: `
          // Mock Service Worker
          self.addEventListener('install', (event) => {
            event.waitUntil(self.skipWaiting());
          });
          self.addEventListener('activate', (event) => {
            event.waitUntil(self.clients.claim());
          });
        `,
      });
    });
  }

  /**
   * Mock task completion PATCH request
   */
  async mockTaskComplete(taskId: string) {
    await this.page.route(`**/api/collections/tasks/records/${taskId}`, (route: Route) => {
      if (route.request().method() === 'PATCH') {
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify(mockData.taskCompletedResponse),
        });
      } else {
        route.continue();
      }
    });
  }

  /**
   * Mock zone creation POST request
   */
  async mockZoneCreate() {
    await this.page.route('**/api/collections/zones/records', (route: Route) => {
      if (route.request().method() === 'POST') {
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify(mockData.zoneCreatedResponse),
        });
      } else {
        route.continue();
      }
    });
  }

  /**
   * Mock zone update PATCH request
   */
  async mockZoneUpdate(zoneId: string) {
    await this.page.route(`**/api/collections/zones/records/${zoneId}`, (route: Route) => {
      if (route.request().method() === 'PATCH') {
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify(mockData.zoneUpdatedResponse),
        });
      } else {
        route.continue();
      }
    });
  }
}
