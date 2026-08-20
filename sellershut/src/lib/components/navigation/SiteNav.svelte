<script lang="ts">
import { onMount } from 'svelte';
import type { User } from '$lib/types/user';
import DesktopMegaMenu from './DesktopMegaMenu.svelte';
import NavBar from './NavBar.svelte';
import type { MenuKey } from './navigation';

let {
  user = undefined,
  domain,
  brandName = 'Sellershut',
  brandHref = '/',
}: {
  user?: User;
  domain: string;
  brandName?: string;
  brandHref?: string;
} = $props();

let signOutAction = '/api/logout';
let activeMenu = $state<MenuKey | null>(null);
let scrolled = $state(false);

function updateScrollState() {
  scrolled = window.scrollY > 8;
}

function closeMenu() {
  activeMenu = null;
}

function setMenu(menu: MenuKey | null) {
  activeMenu = menu;
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape') {
    closeMenu();
  }
}

function closeOnPointerLeave(node: HTMLElement) {
  function handlePointerLeave() {
    closeMenu();
  }

  node.addEventListener('pointerleave', handlePointerLeave);

  return {
    destroy() {
      node.removeEventListener('pointerleave', handlePointerLeave);
    },
  };
}

onMount(() => {
  updateScrollState();
});
</script>

<svelte:window onscroll={updateScrollState} onkeydown={handleKeydown} />

<header class="sticky top-0 z-50" use:closeOnPointerLeave>
  <NavBar
    {user}
    {domain}
    {signOutAction}
    {brandName}
    {brandHref}
    {scrolled}
    {activeMenu}
    onMenuChange={setMenu}
    onCloseMenu={closeMenu}
  />

  <DesktopMegaMenu {activeMenu} onClose={closeMenu} />
</header>
