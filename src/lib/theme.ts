type Mode = 'light' | 'dark' | 'system';

let media: MediaQueryList | null = null;
let listener: ((e: MediaQueryListEvent) => void) | null = null;

function applyDarkClass(on: boolean) {
  const root = document.documentElement;
  if (on) root.classList.add('dark');
  else root.classList.remove('dark');
}

export function setTheme(mode: Mode) {
  if (media && listener) {
    media.removeEventListener('change', listener);
    listener = null;
  }

  if (mode === 'system') {
    media = window.matchMedia('(prefers-color-scheme: dark)');
    const apply = () => applyDarkClass(media!.matches);
    apply();

    listener = () => apply();
    media.addEventListener('change', listener);
    return;
  }

  applyDarkClass(mode === 'dark');
}

export async function initTheme(fetchTheme?: () => Promise<Mode | string | null | undefined>) {
  let mode: Mode = 'system';
  try {
    if (fetchTheme) {
      const v = await fetchTheme();
      if (v === 'light' || v === 'dark' || v === 'system') mode = v;
    }
  } catch {
    // fall back to system
  }
  setTheme(mode);
}
