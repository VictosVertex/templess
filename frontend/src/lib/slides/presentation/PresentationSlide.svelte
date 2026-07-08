<script lang="ts">
	import { SlideLabels, type AppSlide } from "$lib/navigation.svelte";
	import type { Snippet } from "svelte";

  

  let {
		index,
		children
	}: {
		index: AppSlide;
		children: Snippet;
	} = $props();

  const colors = {
    navy: "#030c3e",
    navyMid: "#8F92AC"
  };

  let title = $derived(SlideLabels[index])
  let slideNumber = $derived(index + 1)

  let footerAuthor = "Dominik Gerndt";
  let footerTitle = $derived("TempLess (UI) \u2013 " + title);

  let slideMaxNumber = 1337;
</script>

<div class="flex h-dvh w-screen items-center justify-center overflow-hidden bg-[#050505]">
  <div class="@container relative aspect-video max-h-dvh w-full max-w-[calc(100dvh*16/9)] flex flex-col overflow-hidden bg-white font-sans text-[${colors.navy}]">
    
    <header class="px-12 pt-10">
      {#if title}
        <h2 class="text-4xl md:text-7xl font-medium" style="color: {colors.navy}">
          {title}
        </h2>
      {/if}
    </header>

    <main class="flex-1 px-12  mx-auto w-400">
      {@render children()} 
    </main>

    <footer class="flex justify-between items-center w-full px-4 py-2 text-3xl bg-gray-100 text-black">
      <div class="flex-1 text-left whitespace-nowrap overflow-hidden text-ellipsis">
        {footerAuthor}
      </div>
      
      <div class="flex-1 text-center whitespace-nowrap overflow-hidden text-ellipsis">
        {footerTitle}
      </div>
      
      <div class="flex-1 text-right whitespace-nowrap">
        {slideNumber} / {slideMaxNumber}
      </div>
    </footer>
    
  </div>
</div>