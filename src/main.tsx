import React from "react";
import ReactDOM from "react-dom/client";
import DragNDrop from "./DragNDrop";
import { appWindow } from '@tauri-apps/api/window';
import "./pico.min.css";

import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import AppStateProvider from "./AppState";
import ModelSelection from "./ModelSelection";
import Transcribe from "./Transcribe";
import Success from "./Success";
import ErrorScreen from "./Error";
import Credits from "./Credits";

const router = createBrowserRouter([
  {
    path: "/",
    element: <DragNDrop />,
  },
  {
    path: "/model-selection",
    element: <ModelSelection />,
  },
  {
    path: "/transcribe",
    element: <Transcribe></Transcribe>
  },
  {
    path: "/success",
    element: <Success></Success>
  },
  {
    path: "/error",
    element: <ErrorScreen></ErrorScreen>,
  },
  {
    path: "/credits",
    element: <Credits></Credits>
  }
]);

document!
  .getElementById('titlebar-minimize')!
  .addEventListener('click', () => appWindow.minimize())
// document!
//   .getElementById('titlebar-maximize')!
//   .addEventListener('click', () => appWindow.toggleMaximize())
document!
  .getElementById('titlebar-close')!
  .addEventListener('click', () => appWindow.close())

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <AppStateProvider>
      <RouterProvider router={router}></RouterProvider>
    </AppStateProvider>
);
