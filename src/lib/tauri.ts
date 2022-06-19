import { invoke } from '@tauri-apps/api/tauri';
import type { Location } from '../types/Location';

export async function openDirDialog() {
  let result = await invoke('open_dialog');
  if (result === null) {
    return;
  }

  let location: Location = {
    root: result as string,
    parsed: false,
  };

  console.log('openDirDialog', location);

  return location;
}

export async function analyseLocation(root: string) {
  let result = await invoke('analyse_location', { root });
  if (result === null) {
    return;
  }

  console.log(result);
}
