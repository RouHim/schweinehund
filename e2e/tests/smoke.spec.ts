import { test, expect } from '@playwright/test';

test('API health endpoint responds', async ({ request }) => {
  const response = await request.get('/api/health');
  
  expect(response.status()).toBe(200);
  
  const json = await response.json();
  expect(json).toHaveProperty('status');
  expect(json.status).toBe('ok');
});
