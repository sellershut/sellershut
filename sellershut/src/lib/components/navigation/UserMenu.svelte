<script lang="ts">
import { Heart, LogOut, Mail, Package, Server, Settings, User } from '@lucide/svelte';
import { DropdownMenu } from 'bits-ui';

import type { User as UserType } from '$lib/types/user';

let {
  user,
  domain,
  signOutHref,
}: {
  user: UserType;
  domain: string;
  signOutHref: string;
} = $props();

function initials(name: string) {
  return name
    .trim()
    .split(/\s+/)
    .slice(0, 2)
    .map((part) => part[0])
    .join('')
    .toUpperCase();
}

const itemClass = `
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
	`;
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger
    type="button"
    aria-label={`Account menu for user.handle`}
    class="
			ml-0.5
			flex size-8
			items-center
			justify-center
			overflow-hidden
			rounded-full
			border
			border-border
			bg-muted
			text-[11px]
			font-semibold
			transition
			hover:ring-2
			hover:ring-border
			focus-visible:outline-none
			focus-visible:ring-2
			focus-visible:ring-ring
		"
  >
    {#if user.icon?.url}
      <img src={user.icon.url} alt="" class="size-full object-cover">
    {:else}
      <span aria-hidden="true">
        {initials(user.name ?? user.preferredUsername)}
      </span>
    {/if}
  </DropdownMenu.Trigger>

  <DropdownMenu.Portal>
    <DropdownMenu.Content
      align="end"
      sideOffset={8}
      class="
				z-[100]
				w-64
				rounded-2xl
				border
				border-border/80
				bg-card/95
				p-1.5
				text-card-foreground
				shadow-xl
				shadow-black/10
				backdrop-blur-2xl
				outline-none
				origin-(--bits-dropdown-menu-content-transform-origin)
				transition
				duration-150
				data-[starting-style]:scale-[0.97]
				data-[starting-style]:opacity-0
				data-[ending-style]:scale-[0.97]
				data-[ending-style]:opacity-0
			"
    >
      <div class="px-2.5 pb-2 pt-2">
        <div class="truncate text-[13px] font-semibold">
          {user.name ?? user.preferredUsername}
        </div>

        <div
          class="
						mt-0.5
						truncate
						text-[11px]
						text-muted-foreground
					"
        >
          {`@${user.preferredUsername}@${domain}`}
        </div>
      </div>

      <DropdownMenu.Separator class="my-1 h-px bg-border" />

      <DropdownMenu.Group aria-label="Account">
        <DropdownMenu.Item textValue="View profile" class={itemClass}>
          {#snippet child({ props })}
            <a {...props} href="/account/profile">
              <User class="size-4 text-muted-foreground" strokeWidth={1.8} />

              <span>View profile</span>
            </a>
          {/snippet}
        </DropdownMenu.Item>

        <DropdownMenu.Item textValue="My listings" class={itemClass}>
          {#snippet child({ props })}
            <a {...props} href="/account/listings">
              <Package class="size-4 text-muted-foreground" strokeWidth={1.8} />

              <span>My listings</span>
            </a>
          {/snippet}
        </DropdownMenu.Item>

        <DropdownMenu.Item textValue="Saved listings" class={itemClass}>
          {#snippet child({ props })}
            <a {...props} href="/saved">
              <Heart class="size-4 text-muted-foreground" strokeWidth={1.8} />

              <span>Saved listings</span>
            </a>
          {/snippet}
        </DropdownMenu.Item>

        <DropdownMenu.Item textValue="Messages" class={itemClass}>
          {#snippet child({ props })}
            <a {...props} href="/messages">
              <Mail class="size-4 text-muted-foreground" strokeWidth={1.8} />

              <span>Messages</span>
            </a>
          {/snippet}
        </DropdownMenu.Item>
      </DropdownMenu.Group>

      <DropdownMenu.Separator class="my-1 h-px bg-border" />

      <div class="px-2.5 py-2">
        <div
          class="
						flex
						items-start
						gap-2.5
					"
        >
          <Server
            class="
							mt-0.5
							size-4
							shrink-0
							text-muted-foreground
						"
            strokeWidth={1.8}
          />

          <div class="min-w-0">
            <div
              class="
								truncate
								text-[12px]
								font-medium
							"
            >
              {domain}
            </div>

            <div
              class="
								mt-0.5
								text-[10px]
								text-muted-foreground
							"
            >
              Your home instance
            </div>
          </div>
        </div>
      </div>

      <DropdownMenu.Separator class="my-1 h-px bg-border" />

      <DropdownMenu.Group aria-label="Settings">
        <DropdownMenu.Item textValue="Settings" class={itemClass}>
          {#snippet child({ props })}
            <a {...props} href="/settings">
              <Settings class="size-4 text-muted-foreground" strokeWidth={1.8} />

              <span>Settings</span>
            </a>
          {/snippet}
        </DropdownMenu.Item>

        <DropdownMenu.Item
          textValue="Sign out"
          class={`
						${itemClass}
						text-primary
						data-[highlighted]:bg-primary/10
					`}
        >
          {#snippet child({ props })}
            <a {...props} href={signOutHref}>
              <LogOut class="size-4" strokeWidth={1.8} />
              <span>Sign out</span>
            </a>
          {/snippet}
        </DropdownMenu.Item>
      </DropdownMenu.Group>
    </DropdownMenu.Content>
  </DropdownMenu.Portal>
</DropdownMenu.Root>
