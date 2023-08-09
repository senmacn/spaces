import { FilterType } from '@/views/projects/common/types';
import { defineStore } from 'pinia';

interface ProjectItemState {
  projectItems: ProjectItem[];
  filter: FilterType;
  search: string;
}

const state = {
  projectItems: [],
  filter: FilterType.ALL,
  search: '',
} as ProjectItemState;

export const useProjectStore = defineStore({
  id: 'app-store',
  state: (): ProjectItemState => state,
  getters: {
    getProjectItems(): ProjectItem[] {
      return this.projectItems;
    },
    getFilter(): FilterType {
      return this.filter;
    },
    getSearch(): string {
      return this.search;
    },
  },
  actions: {
    setProjectItems(projectItemList: ProjectItem[]): void {
      this.projectItems = projectItemList;
    },
    setFilter(filters: FilterType): void {
      this.filter = filters;
    },
    setSearch(search: string): void {
      this.search = search;
    },
  },
});
