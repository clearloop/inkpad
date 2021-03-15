export enum Theme {
  Dark = "dark",
  Light = "light",
}

interface Option<T> {
  value: T;
  title: string;
}

export const themeOptions: Option<string>[] = [
  {
    title: "Light",
    value: Theme.Light,
  },
  {
    title: "Dark",
    value: Theme.Dark,
  },
];

// gives back the OS you're using in hotkeys.svelte & shortcuts.svelte
export const isMac: boolean = navigator.platform.includes("Mac");
