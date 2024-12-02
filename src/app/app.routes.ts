/* sys lib */
import { Routes } from "@angular/router";

/* components */
import { HomeComponent } from "@views/home/home.component";
import { AboutComponent } from "@views/about/about.component";

export const routes: Routes = [
  { path: "", pathMatch: "full", redirectTo: "home" },
  { path: "home", data: { breadcrumbs: "Home" }, children: [
    { path: "", component: HomeComponent, title: "Home", data: { breadcrumbs: "Home" }, },
  ] },

  { path: "about", component: AboutComponent, title: "About", data: { breadcrumbs: "About" }, },
];
