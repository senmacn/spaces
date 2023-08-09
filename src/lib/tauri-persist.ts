import { useProjectStore } from '@/store/modules/project-state';
import { getRandomId } from '@/utils/uuid';
import { invoke } from '@tauri-apps/api/tauri';

export default class TauriPersist {
  private static _tauri: TauriPersist;

  private projectItemList: Readonly<ProjectItem[]> = [];

  async init() {
    const projectState = useProjectStore();
    projectState.setProjectItems(await this.getProjectItemListFromTauri());
  }

  private async getProjectItemListFromTauri() {
    const data = await invoke('get_project_item_list');
    if (Array.isArray(data)) {
      return data;
    }
    return [];
  }

  getProjectItemList(): ProjectItem[] {
    return this.projectItemList.slice();
  }

  async addProjectItem(item: ProjectItem) {
    const projectItem = Object.assign({}, item);
    const defaultSchemeId = getRandomId();
    Reflect.set(projectItem, 'default_scheme', defaultSchemeId);
    Reflect.set(projectItem, 'favorite', projectItem.favorite === '1' ? true : false);
    // 这个地方default_scheme不会自动转换
    await invoke('add_project_item', { item: projectItem });
    const projectState = useProjectStore();
    projectState.getProjectItems.push(projectItem);
    // 同时创建默认的启动配置（打开文件位置）
    await this.addStartScheme({
      id: defaultSchemeId,
      projectId: projectItem.id,
      name: '打开文件位置',
      program: 'start {path}',
      args: '',
    });
  }

  async updateProjectItem(item: ProjectItem) {
    const projectItem = Object.assign({}, item);
    Reflect.set(projectItem, 'favorite', projectItem.favorite === '1' ? true : false);
    await invoke('update_project_item', { item: projectItem });
    const projectState = useProjectStore();
    projectState.getProjectItems.forEach((v) => {
      if (v.id === projectItem.id) {
        Object.assign(v, projectItem);
      }
    });
  }

  async updateProjectItemProperty(id: string, updates: OptionalStringKeys<ProjectItem>) {
    await invoke('update_project_item_property', { id, updates });
    const projectState = useProjectStore();
    projectState.getProjectItems.forEach((v) => {
      if (v.id === id) {
        Object.assign(v, updates);
        if (Reflect.has(updates, 'default_scheme')) {
          Reflect.set(v, 'defaultScheme', Reflect.get(updates, 'default_scheme'));
        }
        if (Reflect.has(updates, 'used_at')) {
          Reflect.set(v, 'usedAt', Reflect.get(updates, 'd'));
        }
      }
    });
  }

  async deleteProjectItem(id: string) {
    await invoke('delete_project_item', { deletedId: id });
    const projectState = useProjectStore();
    const index = projectState.getProjectItems.findIndex((v) => v.id === id);
    projectState.getProjectItems.splice(index, 1);
  }

  async getSchemeById(schemeId: string): Promise<StartScheme> {
    return await invoke('get_scheme_by_id', { schemeId });
  }

  async getStartSchemeList(projectId: string) {
    return await invoke('get_start_scheme_list', { projectId });
  }

  async addStartScheme(scheme: StartScheme) {
    await invoke('add_start_scheme', { scheme });
  }

  async updateStartScheme(scheme: StartScheme) {
    await invoke('update_start_scheme', { scheme });
  }

  async deleteStartScheme(id: string) {
    await invoke('delete_start_scheme', { deletedId: id });
  }

  static getInstance() {
    if (this._tauri) {
      return this._tauri;
    } else {
      this._tauri = new TauriPersist();
      return this._tauri;
    }
  }
}
