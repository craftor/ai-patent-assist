import { test, expect } from '@playwright/test';

test.describe('专利和软著详情页测试', () => {
  // 在每次测试前设置 localStorage 中的 token，跳过登录
  test.beforeEach(async ({ page }) => {
    // 先访问登录页以初始化 localStorage
    await page.goto('/login');
    await page.waitForTimeout(1000);

    // 设置 localStorage 中的 token
    await page.evaluate(() => {
      localStorage.setItem('access_token', 'test-admin-token-12345');
    });

    console.log('已设置 token，跳过登录步骤');
  });

  test('访问专利列表页并检查数据', async ({ page }) => {
    await page.goto('/patents');
    await page.waitForTimeout(3000);

    // 截图调试
    await page.screenshot({ path: 'test-results/patents-list.png' });

    // 等待表格出现
    await page.waitForSelector('.el-table', { timeout: 10000 });

    // 检查是否有专利数据行
    const rows = page.locator('.el-table__row');
    const count = await rows.count();
    console.log(`专利数量：${count}`);

    expect(count).toBeGreaterThan(0);
  });

  test('访问软著列表页并检查数据', async ({ page }) => {
    await page.goto('/copyrights');
    await page.waitForTimeout(3000);

    // 截图调试
    await page.screenshot({ path: 'test-results/copyrights-list.png' });

    // 等待表格出现
    await page.waitForSelector('.el-table', { timeout: 10000 });

    // 检查是否有软著数据行
    const rows = page.locator('.el-table__row');
    const count = await rows.count();
    console.log(`软著数量：${count}`);

    expect(count).toBeGreaterThan(0);
  });

  test('查看专利详情页', async ({ page }) => {
    await page.goto('/patents');
    await page.waitForTimeout(3000);
    await page.waitForSelector('.el-table__row');

    // 获取第一行专利名称
    const firstRow = page.locator('.el-table__row').first();
    const patentName = await firstRow.locator('td').nth(0).textContent();
    console.log(`专利名称：${patentName}`);

    // 点击详情按钮
    const detailButton = firstRow.locator('button').first();
    await detailButton.click();
    await page.waitForTimeout(3000);

    // 截图调试
    await page.screenshot({ path: 'test-results/patent-detail.png' });

    // 检查详情页标题
    const pageTitle = page.locator('.page-title');
    await expect(pageTitle).toBeVisible();
    const titleText = await pageTitle.textContent();
    console.log(`详情页标题：${titleText}`);

    expect(titleText!.trim().length).toBeGreaterThan(0);

    // 检查是否显示基本信息区域
    const sections = page.locator('.section');
    await expect(sections.first()).toBeVisible();
    console.log('专利详情页显示正常');
  });

  test('查看软著详情页', async ({ page }) => {
    await page.goto('/copyrights');
    await page.waitForTimeout(3000);
    await page.waitForSelector('.el-table__row');

    // 获取第一行软著名称
    const firstRow = page.locator('.el-table__row').first();
    const softwareName = await firstRow.locator('td').nth(0).textContent();
    console.log(`软件名称：${softwareName}`);

    // 点击详情按钮
    const detailButton = firstRow.locator('button').first();
    await detailButton.click();
    await page.waitForTimeout(3000);

    // 截图调试
    await page.screenshot({ path: 'test-results/copyright-detail.png' });

    // 检查详情页标题
    const pageTitle = page.locator('.page-title');
    await expect(pageTitle).toBeVisible();
    const titleText = await pageTitle.textContent();
    console.log(`详情页标题：${titleText}`);

    expect(titleText!.trim().length).toBeGreaterThan(0);

    // 检查是否显示基本信息区域
    const sections = page.locator('.section');
    await expect(sections.first()).toBeVisible();
    console.log('软著详情页显示正常');
  });

  test('专利详情页返回功能', async ({ page }) => {
    await page.goto('/patents');
    await page.waitForTimeout(3000);
    await page.waitForSelector('.el-table__row');

    // 点击第一个专利详情
    await page.locator('.el-table__row').first().locator('button').first().click();
    await page.waitForTimeout(3000);

    // 检查当前 URL 是详情页
    const currentUrl = page.url();
    console.log('详情页 URL:', currentUrl);
    expect(currentUrl).toMatch(/\/patents\/[a-f0-9-]+$/);

    // 点击返回按钮
    const backButton = page.locator('.el-page-header__back');
    await backButton.click();
    await page.waitForTimeout(2000);

    // 检查是否返回列表页
    const listUrl = page.url();
    console.log('返回列表 URL:', listUrl);
    expect(listUrl).toMatch(/\/patents$/);
    console.log('返回功能正常');
  });

  test('软著详情页返回功能', async ({ page }) => {
    await page.goto('/copyrights');
    await page.waitForTimeout(3000);
    await page.waitForSelector('.el-table__row');

    // 点击第一个软著详情
    await page.locator('.el-table__row').first().locator('button').first().click();
    await page.waitForTimeout(3000);

    // 检查当前 URL 是详情页
    const currentUrl = page.url();
    console.log('详情页 URL:', currentUrl);
    expect(currentUrl).toMatch(/\/copyrights\/[a-f0-9-]+$/);

    // 点击返回按钮
    const backButton = page.locator('.el-page-header__back');
    await backButton.click();
    await page.waitForTimeout(2000);

    // 检查是否返回列表页
    const listUrl = page.url();
    console.log('返回列表 URL:', listUrl);
    expect(listUrl).toMatch(/\/copyrights$/);
    console.log('返回功能正常');
  });
});
