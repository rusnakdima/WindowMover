/* sys lib */
import { CommonModule } from "@angular/common";
import { Component, CUSTOM_ELEMENTS_SCHEMA, OnInit } from "@angular/core";
import { Subject } from "rxjs";

/* services */
import { AppsService } from "@services/apps.service";

/* models */
import { Response } from "@models/response";
import { AppInfo } from "@models/app-info";
import { ActiveApp } from "@models/active_app";

/* components */
import { SearchComponent } from "../shared/fields/search/search.component";
import {
  INotify,
  WindowNotifyComponent,
} from "@views/shared/window-notify/window-notify.component";

@Component({
  selector: "app-home",
  standalone: true,
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
  imports: [CommonModule, SearchComponent, WindowNotifyComponent],
  templateUrl: "./home.component.html",
})
export class HomeComponent implements OnInit {
  constructor(private appsService: AppsService) {}

  dataNotify: Subject<INotify> = new Subject();

  listActiveApps: Array<ActiveApp> = [];
  listFavorites: Array<AppInfo> = [];

  listApps: Array<AppInfo> = [];
  tempListApps: Array<AppInfo> = [];

  ngOnInit() {
    document.addEventListener("keydown", (event: any) => {
      if (
        (event.ctrlKey && event.shiftKey && event.key == "R") ||
        event.key == "F5"
      ) {
        event.preventDefault();
        this.refresh();
      }
    });

    this.refresh();
  }

  refresh() {
    let elem = document.querySelector("#refreshBut") as HTMLElement;
    if (elem != null) {
      elem.className = "animate-spin";
      setTimeout(() => {
        elem.className = "";
      }, 2000);
    }

    this.appsService
      .getListWindows()
      .then((data: Response) => {
        if (data.status == "success") {
          this.listApps = data.data;
          this.tempListApps = this.listApps.slice();
        }
      })
      .catch((err) => {
        console.error(err);
        this.dataNotify.next({
          status: "error",
          text: err.status + " — " + err.message,
        });
      });

    setTimeout(() => {
      if (localStorage["favorites"]) {
        const favorLS = JSON.parse(localStorage["favorites"]);
        let tempFavor: Array<AppInfo> = [];
        favorLS.forEach((favorite: any) => {
          const app = this.tempListApps.find(
            (item: AppInfo) => item.exe_file == favorite.exe_file
          );
          tempFavor.push({
            title: favorite.title,
            exe_file: favorite.exe_file,
            hwnd: app?.hwnd ?? "",
            status: app?.status ?? "",
          });
        });
        this.listFavorites = tempFavor;
      }
    }, 100);
  }

  searchFunc(arr: Array<any>) {
    this.listApps = arr;
  }

  isFavorite(app_info: AppInfo): boolean {
    return (
      this.listFavorites.findIndex((f) => f.exe_file === app_info.exe_file) > -1
    );
  }

  setFavorite(app_info: AppInfo) {
    if (!this.isFavorite(app_info)) {
      if (localStorage["favorites"]) {
        let favorites: Array<any> = JSON.parse(localStorage["favorites"]);
        favorites.push({ exe_file: app_info.exe_file, title: app_info.title });
        localStorage["favorites"] = JSON.stringify(favorites);
      } else {
        localStorage["favorites"] = JSON.stringify([
          { exe_file: app_info.exe_file, title: app_info.title },
        ]);
      }
      this.listFavorites.push(app_info);
    }
  }

  setUnfavorite(app_info: AppInfo) {
    if (this.isFavorite(app_info)) {
      if (localStorage["favorites"]) {
        let favorites: Array<any> = JSON.parse(localStorage["favorites"]);
        const index = favorites.findIndex(
          (f) => f.exe_file === app_info.exe_file
        );
        if (index > -1) {
          favorites.splice(index, 1);
          localStorage["favorites"] = JSON.stringify(favorites);
        }
      }
      this.listFavorites = this.listFavorites.filter(
        (f) => f.exe_file != app_info.exe_file
      );
    }
  }

  setIsMove(app_info: AppInfo, event: any) {
    if (
      this.listActiveApps.findIndex((app) => app.hwnd == app_info.hwnd) > -1
    ) {
      let findedApp = this.listActiveApps.find(
        (app) => app.hwnd == app_info.hwnd
      );
      if (findedApp) {
        findedApp.isMove = event.target.checked;
      }
    } else {
      this.listActiveApps.push({
        hwnd: app_info.hwnd,
        isMove: event.target.checked,
        isOnTop: false,
      });
    }
    this.startActionOnApp();
  }

  setIsOnTop(app_info: AppInfo, event: any) {
    if (
      this.listActiveApps.findIndex((app) => app.hwnd == app_info.hwnd) > -1
    ) {
      let findedApp = this.listActiveApps.find(
        (app) => app.hwnd == app_info.hwnd
      );
      if (findedApp) {
        findedApp.isOnTop = event.target.checked;
      }
    } else {
      this.listActiveApps.push({
        hwnd: app_info.hwnd,
        isMove: false,
        isOnTop: event.target.checked,
      });
    }
    this.startActionOnApp();
  }

  isMove(app_info: AppInfo): boolean {
    return (
      this.listActiveApps.find((app) => app.hwnd === app_info.hwnd)?.isMove ==
      true
    );
  }

  isOnTop(app_info: AppInfo): boolean {
    return (
      this.listActiveApps.find((app) => app.hwnd === app_info.hwnd)?.isOnTop ==
      true
    );
  }

  startActionOnApp() {
    this.appsService
      .startActionOnApp(this.listActiveApps)
      .then((data: Response) => {
        console.log(data);
      })
      .catch((err) => {
        console.error(err);
        this.dataNotify.next({
          status: "error",
          text: err.status + " — " + err.message,
        });
      });
  }
}
