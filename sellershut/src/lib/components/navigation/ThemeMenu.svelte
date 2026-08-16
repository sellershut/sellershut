<script lang="ts">
import { Check, Monitor, Moon, Sun } from '@lucide/svelte';
import { DropdownMenu } from 'bits-ui';
import { onMount } from 'svelte';

type Theme = 'light' | 'system' | 'dark';

const STORAGE_KEY = 'theme';

let theme = $state<Theme>('system');

function isTheme(value: unknown): value is Theme {
  return value === 'light' || value === 'system' || value === 'dark';
}

function applyTheme(mode: Theme) {
  const root = document.documentElement;

  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

  const dark = mode === 'dark' || (mode === 'system' && prefersDark);

  root.classList.toggle('dark', dark);
  root.classList.toggle('light', !dark);

  root.dataset.theme = mode;
  root.style.colorScheme = dark ? 'dark' : 'light';
}

function changeTheme(value: string) {
  if (!isTheme(value)) return;

  theme = value;

  try {
    localStorage.setItem(STORAGE_KEY, value);
  } catch {
    // Storage may be unavailable.
  }

  applyTheme(value);
}

onMount(() => {
  const initial = document.documentElement.dataset.theme;

  if (isTheme(initial)) {
    theme = initial;
  }

  const media = window.matchMedia('(prefers-color-scheme: dark)');

  function handleSystemChange() {
    if (theme === 'system') {
      applyTheme('system');
    }
  }

  media.addEventListener('change', handleSystemChange);

  return () => {
    media.removeEventListener('change', handleSystemChange);
  };
});
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger
    type="button"
    aria-label="Change appearance"
    class="
			inline-flex size-8
			shrink-0
			items-center
			justify-center
			rounded-full
			text-foreground/70
			transition
			duration-200
			hover:bg-muted
			hover:text-foreground
			focus-visible:outline-none
			focus-visible:ring-2
			focus-visible:ring-ring
			active:scale-95
		"
  >
    <Sun class="size-[17px] dark:hidden" strokeWidth={1.8} />

    <Moon class="hidden size-[17px] dark:block" strokeWidth={1.8} />
  </DropdownMenu.Trigger>

  <DropdownMenu.Portal>
    <DropdownMenu.Content
      align="end"
      side="bottom"
      sideOffset={8}
      class="
				z-[100]
				w-44
				rounded-xl
				border
				border-border/80
				bg-card/95
				p-1.5
				text-card-foreground
				shadow-xl
				shadow-black/10
				backdrop-blur-2xl
				outline-none
				origin-[var(--bits-dropdown-menu-content-transform-origin)]
				transition
				duration-150
				data-[starting-style]:scale-[0.97]
				data-[starting-style]:opacity-0
				data-[ending-style]:scale-[0.97]
				data-[ending-style]:opacity-0
			"
    >
      <DropdownMenu.RadioGroup value={theme} onValueChange={changeTheme}>
        <DropdownMenu.GroupHeading
          class="
						px-2.5
						pb-1.5
						pt-1
						text-[11px]
						font-medium
						text-muted-foreground
					"
        >
          Appearance
        </DropdownMenu.GroupHeading>

        <DropdownMenu.RadioItem
          value="light"
          textValue="Light"
          class="
						flex h-9
						cursor-default
						select-none
						items-center
						gap-2.5
						rounded-lg
						px-2.5
						text-[13px]
						outline-none
						data-[highlighted]:bg-muted
					"
        >
          {#snippet children({ checked })}
            <Sun class="size-4 shrink-0 text-muted-foreground" strokeWidth={1.8} />

            <span class="flex-1"> Light </span>

            <span class="flex size-4 items-center justify-center">
              {#if checked}
                <Check class="size-3.5 text-primary" strokeWidth={2} />
              {/if}
            </span>
          {/snippet}
        </DropdownMenu.RadioItem>

        <DropdownMenu.RadioItem
          value="system"
          textValue="System"
          class="
						flex h-9
						cursor-default
						select-none
						items-center
						gap-2.5
						rounded-lg
						px-2.5
						text-[13px]
						outline-none
						data-[highlighted]:bg-muted
					"
        >
          {#snippet children({ checked })}
            <Monitor class="size-4 shrink-0 text-muted-foreground" strokeWidth={1.8} />

            <span class="flex-1"> System </span>

            <span class="flex size-4 items-center justify-center">
              {#if checked}
                <Check class="size-3.5 text-primary" strokeWidth={2} />
              {/if}
            </span>
          {/snippet}
        </DropdownMenu.RadioItem>

        <DropdownMenu.RadioItem
          value="dark"
          textValue="Dark"
          class="
						flex h-9
						cursor-default
						select-none
						items-center
						gap-2.5
						rounded-lg
						px-2.5
						text-[13px]
						outline-none
						data-[highlighted]:bg-muted
					"
        >
          {#snippet children({ checked })}
            <Moon class="size-4 shrink-0 text-muted-foreground" strokeWidth={1.8} />

            <span class="flex-1"> Dark </span>

            <span class="flex size-4 items-center justify-center">
              {#if checked}
                <Check class="size-3.5 text-primary" strokeWidth={2} />
              {/if}
            </span>
          {/snippet}
        </DropdownMenu.RadioItem>
      </DropdownMenu.RadioGroup>
    </DropdownMenu.Content>
  </DropdownMenu.Portal>
</DropdownMenu.Root>
