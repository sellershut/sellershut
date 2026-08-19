<script lang="ts">
import {
  ChevronRight,
  Heart,
  LogOut,
  Mail,
  Menu,
  Orbit,
  Package,
  Settings,
  X,
} from '@lucide/svelte';
import { Dialog } from 'bits-ui';

import { cubicOut } from 'svelte/easing';
import { fade, fly } from 'svelte/transition';
import type { User } from '$lib/types/user';
import { mobileLinks } from './navigation';

let {
  user,
  domain,
  signOutHref,
  onOpen,
}: {
  user?: User;
  domain: string;
  signOutHref: string;
  onOpen: () => void;
} = $props();

let open = $state(false);

function handleOpenChange(next: boolean) {
  open = next;

  if (next) {
    onOpen();
  }
}

function close() {
  open = false;
}

function initials(name: string) {
  return name
    .trim()
    .split(/\s+/)
    .slice(0, 2)
    .map((part) => part[0])
    .join('')
    .toUpperCase();
}
</script>

<Dialog.Root {open} onOpenChange={handleOpenChange}>
  <Dialog.Trigger
    aria-label="Open navigation"
    class="
			flex size-8
			items-center
			justify-center
			rounded-full
			text-foreground/75
			transition
			hover:bg-muted
			focus-visible:outline-none
			focus-visible:ring-2
			focus-visible:ring-ring
			lg:hidden
		"
  >
    <Menu size={18} strokeWidth={1.8} />
  </Dialog.Trigger>

  <Dialog.Portal>
    <Dialog.Overlay
      forceMount
      class="
				fixed
				inset-0
				z-70
				bg-black/20
				lg:hidden
			"
    >
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            transition:fade={{
							duration: 200
						}}
          ></div>
        {/if}
      {/snippet}
    </Dialog.Overlay>

    <Dialog.Content
      forceMount
      class="
				fixed
				inset-0
				z-80
				overflow-y-auto
				bg-background
				lg:hidden
			"
    >
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            in:fly={{
							y: -14,
							duration: 360,
							easing: cubicOut
						}}
            out:fly={{
							y: -8,
							duration: 200,
							easing: cubicOut
						}}
          >
            <Dialog.Title class="sr-only"> Navigation </Dialog.Title>

            <Dialog.Description class="sr-only"> Marketplace navigation. </Dialog.Description>

            <div
              class="
								mx-auto
								flex h-12
								max-w-2xl
								items-center
								justify-between
								px-5
							"
            >
              <a
                href="/"
                onclick={close}
                class="
									flex
									items-center
									gap-2
									font-title
									text-[15px]
									font-semibold
								"
              >
                <Orbit size={18} strokeWidth={1.8} />

                Relay
              </a>

              <Dialog.Close
                aria-label="Close navigation"
                class="
									flex size-8
									items-center
									justify-center
									rounded-full
									hover:bg-muted
									focus-visible:outline-none
									focus-visible:ring-2
									focus-visible:ring-ring
								"
              >
                <X size={18} strokeWidth={1.8} />
              </Dialog.Close>
            </div>

            <div
              class="
								mx-auto
								max-w-2xl
								px-5
								pb-12
								pt-8
							"
            >
              <p
                class="
									mb-5
									text-[11px]
									font-medium
									text-muted-foreground
								"
              >
                Marketplace
              </p>

              <nav aria-label="Mobile navigation" class="flex flex-col">
                {#each mobileLinks as link, index}
                  <a
                    href={link.href}
                    onclick={close}
                    in:fly={{
											y: -10,
											delay: 60 + index * 35,
											duration: 300,
											easing: cubicOut
										}}
                    class="
											group
											flex
											items-center
											justify-between
											border-b
											border-border/70
											py-3.5
											font-title
											text-[24px]
											font-semibold
											tracking-[-0.035em]
										"
                  >
                    {link.label}

                    <ChevronRight
                      size={20}
                      strokeWidth={1.6}
                      class="
												text-muted-foreground
												transition-transform
												group-hover:translate-x-1
											"
                    />
                  </a>
                {/each}
              </nav>

              {#if user}
                <div
                  class="
										mt-10
										rounded-2xl
										border
										border-border
										bg-muted/50
										p-4
									"
                >
                  <div class="flex items-center gap-3">
                    <div
                      class="
												flex size-10
												shrink-0
												items-center
												justify-center
												overflow-hidden
												rounded-full
												bg-foreground
												text-[12px]
												font-semibold
												text-background
											"
                    >
                      {#if user.icon?.url}
                        <img src={user.icon.url} alt="" class="size-full object-cover">
                      {:else}
                        {initials(user.name ?? user.preferredUsername)}
                      {/if}
                    </div>

                    <div class="min-w-0">
                      <div
                        class="
													truncate
													text-[14px]
													font-semibold
												"
                      >
                        {user.name ?? user.preferredUsername}
                      </div>

                      <div
                        class="
													truncate
													text-[11px]
													text-muted-foreground
												"
                      >
                        {`@${user.preferredUsername}@${domain}`}
                      </div>
                    </div>
                  </div>

                  <div
                    class="
											mt-3
											text-[10px]
											text-muted-foreground
										"
                  >
                    Signed in through
                    <span class="font-medium text-foreground">
                      {domain}
                    </span>
                  </div>
                </div>

                <div class="mt-5 flex flex-col">
                  <a
                    href="/account/listings"
                    onclick={close}
                    class="
											flex h-10
											items-center
											gap-3
											text-[13px]
											font-medium
										"
                  >
                    <Package size={17} class="text-muted-foreground" />

                    My listings
                  </a>

                  <a
                    href="/messages"
                    onclick={close}
                    class="
											flex h-10
											items-center
											gap-3
											text-[13px]
											font-medium
										"
                  >
                    <Mail size={17} class="text-muted-foreground" />

                    Messages
                  </a>

                  <a
                    href="/saved"
                    onclick={close}
                    class="
											flex h-10
											items-center
											gap-3
											text-[13px]
											font-medium
										"
                  >
                    <Heart size={17} class="text-muted-foreground" />

                    Saved listings
                  </a>

                  <a
                    href="/settings"
                    onclick={close}
                    class="
											flex h-10
											items-center
											gap-3
											text-[13px]
											font-medium
										"
                  >
                    <Settings size={17} class="text-muted-foreground" />

                    Settings
                  </a>

                  <a
                    href={signOutHref}
                    onclick={close}
                    class="
											flex h-10
											items-center
											gap-3
											text-[13px]
											font-medium
											text-primary
										"
                  >
                    <LogOut size={17} />

                    Sign out
                  </a>
                </div>
              {:else}
                <a
                  href="/login"
                  onclick={close}
                  class="
										mt-8
										flex h-11
										w-full
										items-center
										justify-center
										rounded-full
										bg-foreground
										text-[14px]
										font-semibold
										text-background
									"
                >
                  Sign in
                </a>
              {/if}
            </div>
          </div>
        {/if}
      {/snippet}
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
