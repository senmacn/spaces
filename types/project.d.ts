interface ProjectItem {
  id: string;
  name: string;
  icon: IconType;
  defaultScheme: string | null;
  description: string;
  usedAt: string;
  deletedAt: string | null;
  path: string;
  favorite: '1' | '0';
  tags: string;
}

interface StartScheme {
  id: string;
  projectId: string;
  name: string;
  program: string;
  args: string;
}

enum BuiltInIconType {
  DEFAULT = '',
  FILE = 'file',
  FOLDER = 'folder',
  TXT = 'txt',
  JAVA = 'java',
  JAVASCRIPT = 'javascript',
  IDEA = 'idea',
  VSCODE = 'vscode',
}

type IconType = BuiltInIconType | `custom:${string}`;
