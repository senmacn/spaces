import { fs, path } from '@tauri-apps/api';
import { BaseDirectory } from '@tauri-apps/api/fs';
import { defineStore } from 'pinia';

interface AppData {}

interface AppStore {
  ready: boolean;
  app: AppData;
}

const appStore = {
  ready: false,
  app: {},
} as AppStore;

export const useAppStore = defineStore({
  id: 'app-store',
  state: (): AppStore => appStore,
  getters: {

  },
  actions: {
    init: async function () {
      if (this.ready) {
        return;
      }
      const text = await fs.readTextFile('app.json', { dir: BaseDirectory.App }).catch((error) => {
        console.error(error);
        return '{}';
      });
      const appData: Partial<AppStore> = JSON.parse(text);
      if (appData.app) {
        this.app = appData.app;
      }
      this.ready = true;
    },
  },
});

async function writeJson(folder: string, projectItems: ProjectItem[]) {
  await fs.writeTextFile(
    await path.join(folder, 'projectItems.json'),
    JSON.stringify(projectItems),
  );
}

async function writeAppJson(appData: AppData) {
  await fs.createDir('', { dir: BaseDirectory.App, recursive: true });
  await fs.writeTextFile('app.json', JSON.stringify(appData), {
    dir: BaseDirectory.App,
  });
}

async function pathExists(path: string, baseDir?: BaseDirectory) {
  const exists: boolean = await fs.exists(path, { dir: baseDir });
  return exists;
}
