import { test, expect } from './fixtures';

test.describe('Notification Debug Endpoints', () => {
  test('debug notify-status returns runtime config', async ({ request }) => {
    const response = await request.get('/api/debug/notify-status');
    
    expect(response.status()).toBe(200);
    
    const json = await response.json();
    expect(json).toHaveProperty('topic_masked');
    expect(json).toHaveProperty('server');
    expect(json).toHaveProperty('topic_source');
    expect(json).toHaveProperty('server_source');
    
    // Verify types
    expect(typeof json.topic_masked).toBe('string');
    expect(typeof json.server).toBe('string');
    expect(typeof json.topic_source).toBe('string');
    expect(typeof json.server_source).toBe('string');
    
    // Verify server matches localhost:8199 (from playwright.config.ts)
    expect(json.server).toContain('localhost:8199');
  });

  test('debug trigger-notification returns 200 with notification_triggered message', async ({ request }) => {
    const response = await request.post('/api/debug/trigger-notification');
    
    expect(response.status()).toBe(200);
    
    const json = await response.json();
    expect(json).toHaveProperty('message');
    expect(json.message).toBe('notification_triggered');
  });

  test('debug notify sends test notification and delivers to local ntfy', async ({ request }) => {
    // Check if ntfy is reachable
    let ntfyAvailable = false;
    try {
      const healthResponse = await request.get('http://localhost:8199/v1/health');
      ntfyAvailable = healthResponse.ok();
    } catch (error) {
      // ntfy not reachable
      ntfyAvailable = false;
    }

    // Skip if ntfy container not running
    if (!ntfyAvailable) {
      test.skip(true, 'ntfy container not running on localhost:8199');
      return;
    }

    // Send test notification
    const notifyResponse = await request.post('/api/debug/notify');
    expect(notifyResponse.status()).toBe(200);
    
    const notifyJson = await notifyResponse.json();
    expect(notifyJson).toHaveProperty('message');
    expect(typeof notifyJson.message).toBe('string');

    // Verify notification arrived at local ntfy
    // Poll for messages on schweinehund-e2e-test topic (from playwright.config.ts)
    const pollResponse = await request.get('http://localhost:8199/schweinehund-e2e-test/json?poll=1');
    expect(pollResponse.ok()).toBeTruthy();
    
    const pollText = await pollResponse.text();
    
    // Parse NDJSON response (one message per line)
    const messages = pollText.trim().split('\n').filter(line => line.length > 0).map(line => JSON.parse(line));
    
    // Find the test notification
    const testNotification = messages.find((msg: any) => msg.title === 'Test Notification');
    expect(testNotification).toBeDefined();
    expect(testNotification.title).toBe('Test Notification');
  });
});
