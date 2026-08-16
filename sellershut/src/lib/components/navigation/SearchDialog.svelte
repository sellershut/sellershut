<script lang="ts">
import { ChevronRight, Orbit, Search, X } from '@lucide/svelte';
import { Dialog } from 'bits-ui';

import { cubicOut } from 'svelte/easing';
import { fade, fly } from 'svelte/transition';

import { quickSearchLinks } from './navigation';

let {
  onOpen,
}: {
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
</script>

<Dialog.Root {open} onOpenChange={handleOpenChange}>
  <Dialog.Trigger
    aria-label="Search marketplace"
    class="
			flex size-8
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
    <Search size={17} strokeWidth={1.8} />
  </Dialog.Trigger>

  <Dialog.Portal>
    <Dialog.Overlay
      forceMount
      class="
				fixed
				inset-0
				z-[70]
				bg-black/25
				backdrop-blur-[2px]
			"
    >
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            transition:fade={{
							duration: 240
						}}
          ></div>
        {/if}
      {/snippet}
    </Dialog.Overlay>

    <Dialog.Content
      forceMount
      class="
				fixed
				inset-x-0
				top-0
				z-[80]
				max-h-[min(480px,100dvh)]
				overflow-y-auto
				border-b
				border-border/70
				bg-background/95
				shadow-2xl
				shadow-black/10
				backdrop-blur-2xl
			"
    >
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            in:fly={{
							y: -18,
							duration: 400,
							easing: cubicOut
						}}
            out:fly={{
							y: -10,
							duration: 220,
							easing: cubicOut
						}}
          >
            <Dialog.Title class="sr-only"> Search the marketplace </Dialog.Title>

            <Dialog.Description class="sr-only">
              Search listings, sellers, categories and marketplace servers.
            </Dialog.Description>

            <div
              class="
								mx-auto
								flex h-12
								max-w-[1120px]
								items-center
								gap-3
								px-4
								sm:px-6
							"
            >
              <a
                href="/"
                aria-label="Home"
                class="
									flex size-8
									shrink-0
									items-center
									justify-center
								"
              >
                <Orbit size={18} strokeWidth={1.8} />
              </a>

              <form
                action="/search"
                method="GET"
                onsubmit={close}
                class="
									flex
									min-w-0
									flex-1
									items-center
									gap-2
								"
              >
                <Search
                  size={18}
                  strokeWidth={1.7}
                  class="
										shrink-0
										text-muted-foreground
									"
                />

                <input
                  name="q"
                  type="search"
                  autocomplete="off"
                  autofocus
                  aria-label="Search marketplace"
                  placeholder="Search listings, sellers and servers"
                  class="
										h-10
										min-w-0
										flex-1
										border-0
										bg-transparent
										text-[17px]
										font-medium
										tracking-[-0.02em]
										text-foreground
										outline-none
										placeholder:text-muted-foreground
									"
                >
              </form>

              <Dialog.Close
                aria-label="Close search"
                class="
									flex size-8
									shrink-0
									items-center
									justify-center
									rounded-full
									transition
									hover:bg-muted
									focus-visible:outline-none
									focus-visible:ring-2
									focus-visible:ring-ring
								"
              >
                <X size={17} strokeWidth={1.8} />
              </Dialog.Close>
            </div>

            <div
              class="
								mx-auto
								max-w-[1120px]
								px-4
								pb-8
								pt-5
								sm:px-6
								md:pb-10
								md:pt-7
							"
            >
              <p
                class="
									mb-3
									text-[11px]
									font-medium
									text-muted-foreground
								"
              >
                Quick links
              </p>

              <div class="flex max-w-lg flex-col">
                {#each quickSearchLinks as link, index}
                  <a
                    href={link.href}
                    onclick={close}
                    in:fly={{
											y: -6,
											delay: 65 + index * 35,
											duration: 280,
											easing: cubicOut
										}}
                    class="
											group
											flex
											items-center
											gap-2
											py-1.5
											text-[14px]
											font-medium
											text-foreground/80
											transition-colors
											hover:text-foreground
										"
                  >
                    <ChevronRight
                      size={14}
                      strokeWidth={1.8}
                      class="
												text-muted-foreground
												transition-transform
												group-hover:translate-x-0.5
											"
                    />

                    {link.label}
                  </a>
                {/each}
              </div>
            </div>
          </div>
        {/if}
      {/snippet}
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
