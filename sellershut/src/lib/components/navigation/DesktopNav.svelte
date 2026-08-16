<script lang="ts">
import { type MenuKey, navItems } from './navigation';

let {
  activeMenu,
  onMenuChange,
}: {
  activeMenu: MenuKey | null;
  onMenuChange: (menu: MenuKey | null) => void;
} = $props();

function toggleMenu(menu: MenuKey) {
  onMenuChange(activeMenu === menu ? null : menu);
}
</script>

<div class="hidden h-full items-center lg:flex">
  {#each navItems as item}
    <button
      type="button"
      aria-haspopup="true"
      aria-expanded={activeMenu === item.key}
      aria-controls={`nav-panel-${item.key}`}
      onclick={() => toggleMenu(item.key)}
      onmouseenter={() => onMenuChange(item.key)}
      onfocus={() => onMenuChange(item.key)}
      class="
				relative
				flex h-full
				items-center
				px-3.5
				text-[12px]
				font-medium
				text-foreground/75
				transition-colors
				duration-200
				hover:text-foreground
				focus-visible:outline-none
				focus-visible:text-foreground
			"
    >
      {item.label}

      <span
        class={`absolute inset-x-3 bottom-0 h-px bg-foreground transition-opacity duration-200 ${
					activeMenu === item.key
						? 'opacity-100'
						: 'opacity-0'
				}`}
      ></span>
    </button>
  {/each}
</div>
