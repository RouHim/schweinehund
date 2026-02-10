import { test as base, expect } from '@playwright/test';

export const test = base.extend<{ failOnJsError: void }>({
    failOnJsError: [async ({ page }, use) => {
        const errors: Error[] = [];
        page.on('pageerror', (error) => {
            errors.push(error);
        });

        await use();

        expect(errors, `Uncaught JS errors on page: ${errors.map(e => e.message).join(', ')}`).toHaveLength(0);
    }, { auto: true }],
});

export { expect } from '@playwright/test';
