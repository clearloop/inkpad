import { app, BrowserWindow, ipcMain, shell, dialog } from "electron";
import path from "path";
import reloader from "electron-reloader";
import { RendererMessage, MainMessage } from "./ipc-types";

try {
  reloader(module);
} catch (err) {
  console.log(err);
}

const openExternalLink = (url: string): void => {
  if (
    url.toLowerCase().startsWith("http://") ||
    url.toLowerCase().startsWith("https://")
  ) {
    shell.openExternal(url);
  } else {
    console.warn(`User tried opening URL with invalid URI scheme: ${url}`);
  }
};

// The default value of app.allowRendererProcessReuse is deprecated, it is
// currently "false".  It will change to be "true" in Electron 9.  For more
// information please check https://github.com/electron/electron/issues/18397
app.allowRendererProcessReuse = true;

class WindowManager {
  public window: BrowserWindow | null;
  private messages: MainMessage[];

  constructor() {
    this.window = null;
    this.messages = [];
  }

  // Send a message on the "message" channel to the renderer window
  sendMessage(message: MainMessage) {
    if (this.window === null || this.window.webContents.isLoading()) {
      this.messages.push(message);
    } else {
      this.window.webContents.send("message", message);
    }
  }

  reload() {
    if (this.window) {
      this.window.reload();
    }
  }

  open() {
    if (this.window) {
      return;
    }

    const window = new BrowserWindow({
      width: 800,
      height: 600,
      show: false,
      webPreferences: {
        contextIsolation: false,
        preload: path.join(__dirname, "preload.js"),
      },
    });

    window.once("ready-to-show", () => {
      window.maximize();
      window.show();
    });

    window.webContents.on("will-navigate", (event, url) => {
      event.preventDefault();
      openExternalLink(url);
    });

    window.webContents.on("new-window", (event, url) => {
      event.preventDefault();
      openExternalLink(url);
    });

    window.on("closed", () => {
      this.window = null;
    });

    window.webContents.on("did-finish-load", () => {
      this.messages.forEach((message) => {
        window.webContents.send("message", message);
      });
      this.messages = [];
    });

    window.loadURL(`file://${path.join(__dirname, "index.html")}`);

    this.window = window;
  }
}

const windowManager = new WindowManager();

ipcMain.handle(RendererMessage.DIALOG_SHOWOPENDIALOG, async () => {
  console.log("Render message - DIALOG");
  const window = windowManager.window;
  console.log(window);
  if (window === null) {
    return;
  }

  const result = await dialog.showOpenDialog(window, {
    properties: ["openDirectory", "showHiddenFiles", "createDirectory"],
  });

  if (result.filePaths.length === 1) {
    return result.filePaths[0];
  } else {
    return "";
  }
});

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on("ready", () => {
  windowManager.open();
});

// Quit when all windows are closed.
app.on("window-all-closed", () => {
  // On macOS it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (app.isReady() && !windowManager.window) {
    windowManager.open();
  }
});
