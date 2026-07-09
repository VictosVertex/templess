<script lang="ts">
	import { nav, AppSlide, SlideLabels } from '$lib/navigation.svelte';

	let { slides = [] }: { slides: AppSlide[] } = $props();
</script>

{#if nav.activeSlide > 0}
	<div
		class="pointer-events-none fixed top-1/2 left-6 z-40 hidden -translate-y-1/2 flex-col items-center md:flex"
	>
		<div
			class="absolute top-0 left-1/2 -z-10 h-full w-px -translate-x-1/2 bg-linear-to-b from-transparent via-primary/20 to-transparent"
		></div>

		<div class="flex flex-col gap-12 py-8">
			{#each slides as slide (slide)}
				<button
					class="group pointer-events-auto relative flex cursor-pointer items-center justify-center outline-none"
					onclick={() => nav.scrollTo(slide)}
					aria-label="Navigate to {SlideLabels[slide]}"
				>
					<div
						class="h-2.5 w-2.5 rotate-45 border border-primary/40 transition-all duration-300
                        {nav.activeSlide === slide
							? 'scale-125 border-primary bg-primary'
							: 'bg-background group-hover:border-primary group-hover:bg-primary/20'}"
					></div>

					<span
						class="font-sovereign absolute left-6 font-bold tracking-[0.2em] whitespace-nowrap uppercase transition-all duration-300
                        {nav.activeSlide === slide
							? 'translate-x-2 text-[10px] text-primary'
							: 'text-[9px] text-primary/60 group-hover:translate-x-2 group-hover:text-primary'}"
					>
						{SlideLabels[slide]}
					</span>
				</button>
			{/each}
		</div>
	</div>

	<div
		class="pb-safe fixed bottom-0 left-0 z-40 flex w-full flex-col border-t border-primary/20 bg-background/95 backdrop-blur-md md:hidden"
	>
		<div class="flex w-full items-center justify-around px-2 py-3">
			{#each slides as slide (slide)}
				<button
					type="button"
					class="group relative flex h-12 flex-1 flex-col items-center justify-center py-1 outline-none"
					onclick={() => nav.scrollTo(slide)}
					aria-label="Go to {SlideLabels[slide]}"
				>
					<div
						class="h-2.5 w-2.5 rotate-45 border transition-all duration-300
                        {nav.activeSlide === slide
							? 'scale-110 border-primary bg-primary'
							: slide < nav.activeSlide
								? 'border-primary/40 bg-primary/40'
								: 'border-primary/30 bg-transparent active:scale-90'}"
					></div>

					<span
						class="mt-2 inline-block font-reading text-[8px] tracking-wider text-primary uppercase transition-colors group-hover:text-primary"
					>
						{SlideLabels[slide]}.
					</span>
				</button>
			{/each}
		</div>
	</div>
{/if}
