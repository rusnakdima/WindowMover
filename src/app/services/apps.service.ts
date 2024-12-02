/* sys lib */
import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';

/* models */
import { Response } from '@models/response';
import { ActiveApp } from '@models/active_app';

@Injectable({
  providedIn: 'root'
})
export class AppsService {
  constructor() {}

  async getListWindows(): Promise<Response> {
    const rawRes = (await invoke("get_list_windows")) as string;
    return Response.fromJson(JSON.parse(rawRes), true);
  }
  
  async startActionOnApp(listActiveApps: Array<ActiveApp>) {
    const rawRes = (await invoke("start_action_on_app", { rawListActiveApps: JSON.stringify(listActiveApps) })) as string;
    return Response.fromJson(JSON.parse(rawRes));
  }
}
