<script lang="ts">
import { ArrowUpRight } from '@lucide/svelte';

import { cubicOut } from 'svelte/easing';
import { fade, fly } from 'svelte/transition';

import { type MenuKey, menus } from './navigation';

let {
  activeMenu,
  onClose,
}: {
  activeMenu: MenuKey | null;
  onClose: () => void;
} = $props();
</script>

{#if activeMenu}
  <button
    type="button"
    tabindex="-1"
    aria-label="Close navigation menu"
    onclick={onClose}
    class="
			fixed
			inset-x-0
			bottom-0
			top-12
			z-30
			hidden
			cursor-default
			bg-black/20
			backdrop-blur-[1px]
			lg:block
		"
  ></button>

  {#key activeMenu}
    <section
      id={`nav-panel-${activeMenu}`}
      aria-label={menus[activeMenu].heading}
      in:fly={{
				y: -10,
				duration: 320,
				easing: cubicOut
			}}
      out:fade={{
				duration: 130
			}}
      class="
				absolute
				inset-x-0
				top-12
				z-40
				hidden
				border-b
				border-border/60
				bg-background/95
				shadow-2xl
				shadow-black/10
				backdrop-blur-2xl
				lg:block
			"
    >
      <div
        class="
					mx-auto
					grid
					max-w-[1120px]
					grid-cols-[minmax(0,1.5fr)_minmax(220px,0.7fr)]
					gap-16
					px-6
					pb-10
					pt-8
				"
      >
        <div>
          <p
            class="
							mb-4
							text-[11px]
							font-medium
							text-muted-foreground
						"
          >
            {menus[activeMenu].heading}
          </p>

          <div class="grid grid-cols-2 gap-x-8 gap-y-1">
            {#each menus[activeMenu].primary as link, index}
              <a
                href={link.href}
                onclick={onClose}
                in:fly={{
									y: -7,
									delay: 40 + index * 30,
									duration: 270,
									easing: cubicOut
								}}
                class="
									group
									rounded-xl
									px-3
									py-2.5
									transition-colors
									hover:bg-muted
								"
              >
                <div
                  class="
										font-title
										text-[18px]
										font-semibold
										tracking-[-0.025em]
									"
                >
                  {link.label}
                </div>

                {#if link.description}
                  <div
                    class="
											mt-0.5
											text-[12px]
											text-muted-foreground
										"
                  >
                    {link.description}
                  </div>
                {/if}
              </a>
            {/each}
          </div>
        </div>

        <div>
          <p
            class="
							mb-4
							text-[11px]
							font-medium
							text-muted-foreground
						"
          >
            More
          </p>

          <div class="flex flex-col">
            {#each menus[activeMenu].secondary as link}
              <a
                href={link.href}
                target={link.external ? '_blank' : undefined}
                rel={link.external ? 'noreferrer' : undefined}
                onclick={onClose}
                class="
									group
									flex
									items-center
									gap-1.5
									py-1.5
									text-[13px]
									font-medium
									text-foreground/75
									transition-colors
									hover:text-foreground
								"
              >
                {link.label}

                {#if link.external}
                  <ArrowUpRight
                    size={12}
                    strokeWidth={1.8}
                    class="
											text-muted-foreground
											transition-transform
											group-hover:-translate-y-px
											group-hover:translate-x-px
										"
                  />
                {/if}
              </a>
            {/each}
          </div>
        </div>
      </div>
    </section>
  {/key}
{/if}
