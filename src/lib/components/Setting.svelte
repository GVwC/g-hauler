<!-- src/lib/components/Setting.svelte -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import Switch from './ui/Switch.svelte';

  // Types kept in sync with backend
  type SelectOption = { value: string; label: string; description?: string };
  type SettingType =
    | { type: 'toggle' }
    | { type: 'text'; placeholder?: string; validation?: { pattern?: string; min_length?: number; max_length?: number } }
    | { type: 'number'; min?: number; max?: number; step?: number; unit?: string }
    | { type: 'select'; options: SelectOption[] }
    | { type: 'path'; directory: boolean; extensions?: string[] }
    | { type: 'color' }
    | { type: 'keybind' };

  export type Setting = {
    key: string;
    label: string;
    description?: string;
    category: string;
    default_value: unknown;
    setting_type: SettingType;
    requires_restart: boolean;
    system_managed: boolean;
  };

  export type SettingItemState = {
    key: string;
    user_value: unknown;
    effective_value: unknown;
    in_sync: boolean;
    capable: boolean;
    error?: string | null;
  };

  let {
    item,
    stateByKey,
    onStatePatched = (_: SettingItemState[]) => {}
  } = $props<{
    item: Setting;
    stateByKey: Map<string, SettingItemState>;
    onStatePatched?: (items: SettingItemState[]) => void;
  }>();

  let saving = $state(false);

  function rowState(): SettingItemState | undefined {
    return stateByKey.get(item.key);
  }

  function currentValue<T = unknown>(): T {
    return (rowState()?.user_value ?? item.default_value) as T;
  }

  async function applyValue(value: unknown) {
    if (saving) return;
    saving = true;
    try {
      const next = await invoke<SettingItemState[]>('settings_set_and_apply', {
        key: item.key,
        value
      });
      onStatePatched(next);
    } catch (e) {
      console.error(`Failed to set "${item.key}"`, e);
    } finally {
      saving = false;
    }
  }

  async function browsePath() {
    const kind = item.setting_type;
    if (kind.type !== 'path') return;

    // Start from current path if available
    const defaultPath = String(currentValue<string>() ?? '');

    if (kind.directory) {
      const picked = await open({
        directory: true,
        multiple: false,
        defaultPath // optional; ignored if empty
      });
      if (picked && typeof picked === 'string') {
        await applyValue(picked);
      }
    } else {
      const picked = await open({
        multiple: false,
        filters: kind.extensions && kind.extensions.length
          ? [{ name: 'Files', extensions: kind.extensions }]
          : undefined,
        defaultPath
      });
      if (picked && typeof picked === 'string') {
        await applyValue(picked);
      }
    }
  }
</script>

<div class="w-full">
  <div class="flex items-center justify-between gap-4">
    <div class="min-w-0">
      <label for={`${item.key}-setting`} class="text-white font-dm-sans text-base truncate">
        {item.label}
      </label>
      {#if item.description}
        <div class="text-xs text-gray-300 mt-1 truncate">{item.description}</div>
      {/if}
      {#if rowState() && !rowState()!.in_sync}
        <div class="text-[10px] text-yellow-400 mt-1">Not in sync with system</div>
      {/if}
      {#if rowState() && rowState()!.error}
        <div class="text-[10px] text-red-400 mt-1">{rowState()!.error}</div>
      {/if}
    </div>

    {#if item.setting_type.type === 'toggle'}
      <Switch
        name={item.key}
        checked={Boolean(currentValue<boolean>())}
        disabled={saving || (rowState() && !rowState()!.capable)}
        onChange={({ checked }) => applyValue(checked)}
      />
    {:else if item.setting_type.type === 'path'}
      <div class="flex items-center gap-2">
        <input
          id={`${item.key}-setting`}
          class="bg-neutral-800 text-white text-sm px-3 py-1.5 rounded-md w-80 outline-none border border-neutral-700/70"
          value={String(currentValue<string>() ?? '')}
          readonly
        />
        <button
          type="button"
          class="px-3 py-1.5 rounded-md bg-blue-600 hover:bg-blue-700 transition text-white text-sm disabled:opacity-60"
          onclick={browsePath}
          disabled={saving}
        >
          Browse…
        </button>
      </div>
    {:else}
      <div class="text-sm text-gray-400">Unsupported: {item.setting_type.type}</div>
    {/if}
  </div>
  <div class="h-4"></div>
</div>
