"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.RendererMessage = exports.MainMessageKind = void 0;
var MainMessageKind;
(function (MainMessageKind) {
    MainMessageKind["PROXY_ERROR"] = "PROXY_ERROR";
})(MainMessageKind = exports.MainMessageKind || (exports.MainMessageKind = {}));
// Message kinds sent from the renderer to the main process.
var RendererMessage;
(function (RendererMessage) {
    RendererMessage["CLIPBOARD_WRITETEXT"] = "IPC_CLIPBOARD_WRITETEXT";
    RendererMessage["DIALOG_SHOWOPENDIALOG"] = "IPC_DIALOG_SHOWOPENDIALOG";
    RendererMessage["GET_VERSION"] = "GET_VERSION";
    RendererMessage["OPEN_PATH"] = "IPC_OPEN_PATH";
    RendererMessage["OPEN_URL"] = "IPC_OPEN_URL";
    RendererMessage["GET_GIT_GLOBAL_DEFAULT_BRANCH"] = "GET_GIT_GLOBAL_DEFAULT_BRANCH";
})(RendererMessage = exports.RendererMessage || (exports.RendererMessage = {}));
