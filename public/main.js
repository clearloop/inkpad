"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var electron_1 = require("electron");
var path_1 = __importDefault(require("path"));
var electron_reloader_1 = __importDefault(require("electron-reloader"));
var ipc_types_1 = require("./ipc-types");
try {
    electron_reloader_1.default(module);
}
catch (err) {
    console.log(err);
}
var openExternalLink = function (url) {
    if (url.toLowerCase().startsWith("http://") ||
        url.toLowerCase().startsWith("https://")) {
        electron_1.shell.openExternal(url);
    }
    else {
        console.warn("User tried opening URL with invalid URI scheme: " + url);
    }
};
// The default value of app.allowRendererProcessReuse is deprecated, it is
// currently "false".  It will change to be "true" in Electron 9.  For more
// information please check https://github.com/electron/electron/issues/18397
electron_1.app.allowRendererProcessReuse = true;
var WindowManager = /** @class */ (function () {
    function WindowManager() {
        this.window = null;
        this.messages = [];
    }
    // Send a message on the "message" channel to the renderer window
    WindowManager.prototype.sendMessage = function (message) {
        if (this.window === null || this.window.webContents.isLoading()) {
            this.messages.push(message);
        }
        else {
            this.window.webContents.send("message", message);
        }
    };
    WindowManager.prototype.reload = function () {
        if (this.window) {
            this.window.reload();
        }
    };
    WindowManager.prototype.open = function () {
        var _this = this;
        if (this.window) {
            return;
        }
        console.log(this.window);
        console.log("open window");
        var window = new electron_1.BrowserWindow({
            width: 1200,
            height: 680,
            show: false,
            webPreferences: {
                contextIsolation: false,
                preload: path_1.default.join(__dirname, "preload.js"),
            },
        });
        window.once("ready-to-show", function () {
            window.maximize();
            window.show();
        });
        window.webContents.on("will-navigate", function (event, url) {
            event.preventDefault();
            openExternalLink(url);
        });
        window.webContents.on("new-window", function (event, url) {
            event.preventDefault();
            openExternalLink(url);
        });
        window.on("closed", function () {
            _this.window = null;
        });
        window.webContents.on("did-finish-load", function () {
            _this.messages.forEach(function (message) {
                window.webContents.send("message", message);
            });
            _this.messages = [];
        });
        window.loadURL("file://" + path_1.default.join(__dirname, "index.html"));
        this.window = window;
    };
    return WindowManager;
}());
var windowManager = new WindowManager();
electron_1.ipcMain.handle(ipc_types_1.RendererMessage.DIALOG_SHOWOPENDIALOG, function () { return __awaiter(void 0, void 0, void 0, function () {
    var window, result;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                console.log("Render message - DIALOG");
                window = windowManager.window;
                console.log(window);
                if (window === null) {
                    return [2 /*return*/];
                }
                return [4 /*yield*/, electron_1.dialog.showOpenDialog(window, {
                        properties: ["openDirectory", "showHiddenFiles", "createDirectory"],
                    })];
            case 1:
                result = _a.sent();
                if (result.filePaths.length === 1) {
                    return [2 /*return*/, result.filePaths[0]];
                }
                else {
                    return [2 /*return*/, ""];
                }
                return [2 /*return*/];
        }
    });
}); });
// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
electron_1.app.on("ready", function () {
    windowManager.open();
});
// Quit when all windows are closed.
electron_1.app.on("window-all-closed", function () {
    // On macOS it is common for applications and their menu bar
    // to stay active until the user quits explicitly with Cmd + Q
    if (process.platform !== "darwin") {
        electron_1.app.quit();
    }
});
electron_1.app.on("activate", function () {
    if (electron_1.app.isReady() && !windowManager.window) {
        windowManager.open();
    }
});
