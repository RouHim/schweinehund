import { request, FullConfig } from '@playwright/test';

async function globalSetup(_config: FullConfig) {
  const requestContext = await request.newContext({
    baseURL: 'http://localhost:9666',
  });

  try {
    const response = await requestContext.post('/api/debug/reset-all');
    if (response.status() !== 200) {
      throw new Error(`Global setup reset failed: ${response.status()}`);
    }
  } finally {
    await requestContext.dispose();
  }
}

export default globalSetup;
