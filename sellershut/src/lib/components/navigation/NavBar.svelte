<script lang="ts">
import { Mail, Plus } from '@lucide/svelte';
import SellershutIcon from '$lib/components/icons/SellershutIcon.svelte';
import type { User } from '$lib/types/user';
import DesktopNav from './DesktopNav.svelte';
import MobileNavDialog from './MobileNavDialog.svelte';
import type { MenuKey } from './navigation';
import SearchDialog from './SearchDialog.svelte';
import ThemeMenu from './ThemeMenu.svelte';
import UserMenu from './UserMenu.svelte';

let {
  user,
  domain,
  brandName,
  brandHref,
  scrolled,
  activeMenu,
  onMenuChange,
  onCloseMenu,
  signOutAction,
}: {
  user?: User;
  domain: string;
  brandName: string;
  brandHref: string;
  signOutAction: string;
  scrolled: boolean;
  activeMenu: MenuKey | null;
  onMenuChange: (menu: MenuKey | null) => void;
  onCloseMenu: () => void;
} = $props();
</script>

<div
  class={`relative z-50 transition-[background-color,border-color,box-shadow,backdrop-filter] duration-300 ${
		scrolled
			? 'border-b border-border/60 bg-background/75 shadow-sm shadow-black/5 backdrop-blur-2xl backdrop-saturate-150'
			: 'border-b border-transparent bg-background'
	}`}
>
  <nav
    aria-label="Global navigation"
    class="
			mx-auto
			flex h-12
			max-w-280
			items-center
			justify-between
			gap-4
			px-4
			sm:px-6
		"
  >
    <a
      href={brandHref}
      aria-label={`${brandName} home`}
      class="
				flex shrink-0
				items-center
				gap-1
				rounded-md
				font-title
				text-[15px]
				font-semibold
				tracking-[-0.02em]
				focus-visible:outline-none
				focus-visible:ring-2
				focus-visible:ring-ring
			"
    >
      <SellershutIcon
        class="
					flex size-7
					items-center
					justify-center
                    bg-transparent
                    text-primary
				"
      > </SellershutIcon>
      <span>sellershut</span>
    </a>

    <DesktopNav {activeMenu} {onMenuChange} />

    <div class="flex shrink-0 items-center gap-1">
      <SearchDialog onOpen={onCloseMenu} />

      <ThemeMenu />

      {#if user}
        <a
          href="/sell"
          class="
						mx-1
						hidden h-8
						items-center
						gap-1.5
						rounded-full
						bg-foreground
						px-3.5
						text-[12px]
						font-semibold
						text-background
						transition
						hover:opacity-80
						focus-visible:outline-none
						focus-visible:ring-2
						focus-visible:ring-ring
						focus-visible:ring-offset-2
						focus-visible:ring-offset-background
						md:inline-flex
					"
        >
          <Plus size={14} strokeWidth={2} />

          List item
        </a>

        <a
          href="/messages"
          aria-label="Messages"
          class="
						hidden size-8
						items-center
						justify-center
						rounded-full
						text-foreground/70
						transition-colors
						hover:bg-muted
						hover:text-foreground
						focus-visible:outline-none
						focus-visible:ring-2
						focus-visible:ring-ring
						sm:flex
					"
        >
          <Mail size={17} strokeWidth={1.8} />
        </a>

        <div class="hidden sm:block">
          <UserMenu {user} {signOutAction} {domain} />
        </div>
      {:else}
        <a
          href="/login"
          class="
						ml-1
						hidden h-8
						items-center
						rounded-full
						bg-foreground
						px-3.5
						text-[12px]
						font-semibold
						text-background
						transition-opacity
						hover:opacity-80
						focus-visible:outline-none
						focus-visible:ring-2
						focus-visible:ring-ring
						focus-visible:ring-offset-2
						focus-visible:ring-offset-background
						sm:inline-flex
					"
        >
          Sign in
        </a>
      {/if}

      <MobileNavDialog {user} {domain} {signOutAction} onOpen={onCloseMenu} />
    </div>
  </nav>
</div>
