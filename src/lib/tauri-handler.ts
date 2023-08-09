import { invoke, fs } from '@tauri-apps/api';
import { BaseDirectory } from '@tauri-apps/api/fs';
import TauriPersist from './tauri-persist';

const pathExists = async (path: string, baseDir?: BaseDirectory) => {
  try {
    const exists: boolean = await fs.exists(path, { dir: baseDir });
    return exists;
  } catch (err) {
    throw new Error('路径不存在！');
  }
};

export async function tauriOpenProgram(path: string) {
  const exists = await pathExists(path);
  if (!exists) {
    throw new Error('路径不存在！');
  }
  try {
    const result = await invoke('open_program', { path });
    return result;
  } catch (err) {
    console.error(err);
    throw new Error('启动失败！');
  }
}

function reflectProperty(target: object, str: string) {
  // 保存需要替换的变量
  const propertyNames: string[] = [];
  // 匹配变量
  const matchResult = str.matchAll(/.*\{(.+)\}.*/g);
  let array = matchResult.next();
  while (!array.done) {
    if (array.value.length > 0) {
      propertyNames.push(array.value[1]);
    }
    array = matchResult.next();
  }

  let resultStr = str;
  const errors: string[] = [];
  for (let index = 0; index < propertyNames.length; index++) {
    const propertyName = propertyNames[index.toString()];
    if (Reflect.has(target, propertyName)) {
      const value = Reflect.get(target, propertyName);
      resultStr = str.replace('{' + propertyName + '}', value);
    } else {
      errors.push(propertyName);
    }
  }
  if (errors.length) {
    throw new Error(`变量[${errors.join(',')}]解析失败！`);
  }
  return resultStr;
}

export async function tauriExecuteCommand(command: string, project: ProjectItem) {
  let result;
  try {
    const formatCommand = reflectProperty(project, command);
    result = await invoke('execute_command', { command: formatCommand });
    TauriPersist.getInstance().updateProjectItemProperty(project.id, {
      // @ts-ignore
      used_at: new Date().valueOf().toString(),
    });
  } catch (err) {
    console.error(err);
    throw err;
  }
  return result;
}
